// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub struct OxidizedPythonInterpreterConfig {
    pub placeholder: Option<()>,
}

mod config {
    // Crash notes: crash goes away if type not defined in submodule.
    pub struct ResolvedOxidizedPythonInterpreterConfig {
        pub(crate) inner: crate::OxidizedPythonInterpreterConfig,
    }
}

type wchar_t = i32;

use {
    crate::config::ResolvedOxidizedPythonInterpreterConfig,
    std::{
        convert::TryInto,
        ffi::{CString, NulError, OsString},
        os::{
            raw::{c_char, c_int, c_ulong},
            unix::ffi::OsStrExt,
        },
    },
};

type Py_ssize_t = isize;

#[repr(C)]
pub struct PyWideStringList {
    pub length: Py_ssize_t,
    pub items: *mut *mut wchar_t,
}

#[repr(C)]
pub struct PyConfig {
    pub _config_init: c_int,
    pub isolated: c_int,
    pub use_environment: c_int,
    pub dev_mode: c_int,
    pub install_signal_handlers: c_int,
    pub use_hash_seed: c_int,
    pub hash_seed: c_ulong,
    pub faulthandler: c_int,
    pub _use_peg_parser: c_int,
    pub tracemalloc: c_int,
    pub import_time: c_int,
    pub show_ref_count: c_int,
    pub dump_refs: c_int,
    pub malloc_stats: c_int,
    pub filesystem_encoding: *mut wchar_t,
    pub filesystem_errors: *mut wchar_t,
    pub pycache_prefix: *mut wchar_t,
    pub parse_argv: c_int,
    pub argv: PyWideStringList,
    pub program_name: *mut wchar_t,
    pub xoptions: PyWideStringList,
    pub warnoptions: PyWideStringList,
    pub site_import: c_int,
    pub bytes_warning: c_int,
    pub inspect: c_int,
    pub interactive: c_int,
    pub optimization_level: c_int,
    pub parser_debug: c_int,
    pub write_bytecode: c_int,
    pub verbose: c_int,
    pub quiet: c_int,
    pub user_site_directory: c_int,
    pub configure_c_stdio: c_int,
    pub buffered_stdio: c_int,
    pub stdio_encoding: *mut wchar_t,
    pub stdio_errors: *mut wchar_t,
    pub check_hash_pycs_mode: *mut wchar_t,
    pub pathconfig_warnings: c_int,
    pub pythonpath_env: *mut wchar_t,
    pub home: *mut wchar_t,
    pub module_search_paths_set: c_int,
    pub module_search_paths: PyWideStringList,
    pub executable: *mut wchar_t,
    pub base_executable: *mut wchar_t,
    pub prefix: *mut wchar_t,
    pub base_prefix: *mut wchar_t,
    pub exec_prefix: *mut wchar_t,
    pub base_exec_prefix: *mut wchar_t,
    pub platlibdir: *mut wchar_t,
    pub skip_source_first_line: c_int,
    pub run_command: *mut wchar_t,
    pub run_module: *mut wchar_t,
    pub run_filename: *mut wchar_t,
    pub _install_importlib: c_int,
    pub _init_main: c_int,
    pub _isolated_interpreter: c_int,
    pub orig_argv: PyWideStringList,
}

#[repr(C)]
pub struct PyStatus {
    pub func: *const c_char,
    pub err_msg: *const c_char,
    pub exitcode: c_int,
}

extern "C" {
    pub fn PyConfig_SetBytesArgv(
        config: *mut PyConfig,
        argc: Py_ssize_t,
        argv: *mut *const c_char,
    ) -> PyStatus;

    pub fn PyStatus_Exception(err: PyStatus) -> c_int;

    pub fn PyConfig_InitIsolatedConfig(config: *mut PyConfig);
}

pub fn set_argv(config: &mut PyConfig, args: &[OsString]) -> Result<(), &'static str> {
    let argc = args.len() as isize;
    let argv = args
        .iter()
        .map(|x| CString::new(x.as_bytes()))
        .collect::<Result<Vec<_>, NulError>>()
        .map_err(|_| "unable to construct C string from OsString")?;
    let argvp = argv
        .iter()
        .map(|x| x.as_ptr() as *mut i8)
        .collect::<Vec<_>>();

    let status = unsafe {
        PyConfig_SetBytesArgv(
            config as *mut PyConfig as *mut _,
            argc,
            argvp.as_ptr() as *mut _,
        )
    };

    if unsafe { PyStatus_Exception(status) } != 0 {
        Err("PyStatus not 0")
    } else {
        Ok(())
    }
}

pub fn create_config() -> Result<PyConfig, &'static str> {
    let mut config: PyConfig = unsafe { std::mem::zeroed() };
    unsafe { PyConfig_InitIsolatedConfig(&mut config as *mut PyConfig as *mut _) };

    Ok(config)
}

// Crash notes: crash goes away if this conversion function is defined as a regular
// function and not a trait.
impl TryInto<PyConfig> for &ResolvedOxidizedPythonInterpreterConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<PyConfig, Self::Error> {
        // Crash notes: inlining the function call makes crash go away.
        let mut config: PyConfig = create_config()?;

        // Crash notes:
        //
        // * inlining function makes crash go away
        // * `set_argv(&mut config, &[std::ffi::OsString::default()])?` will not crash
        // * removing `args` argument makes crash go away
        set_argv(&mut config, &vec![std::ffi::OsString::default()])?;

        // Crash notes: this line crashes because `self` is NULL.
        assert!(self.inner.placeholder.is_none());

        Ok(config)
    }
}

fn main() {
    let config = ResolvedOxidizedPythonInterpreterConfig {
        inner: OxidizedPythonInterpreterConfig { placeholder: None },
    };

    let py_config: PyConfig = (&config).try_into().unwrap();
}
