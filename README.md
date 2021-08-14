# Rust Crash

This repository demonstrates the miscompilation crash as reported in
https://github.com/rust-lang/rust/issues/87947.

To crash:

   cargo run --release

The program needs to link against libpython3.9. This should be available on
modern Linux distros.
