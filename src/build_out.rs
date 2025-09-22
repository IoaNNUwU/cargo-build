use std::cell::RefCell;
use std::io::{stdout, Write};

thread_local! {
    pub static CARGO_BUILD_OUT: RefCell<Box<dyn Write>> = RefCell::new(Box::new(stdout()));
}

/// Use this function to set custom output stream for `cargo-build` commands.
///
/// Useful for debugging, logging and testing.
///
/// Use [`reset`] to reset output stream to `stdout`. This is the default and is necessary
/// for `cargo-build` commands to work inside `build.rs`.
///
/// ```rust
/// let file = std::fs::File::create("target/cargo_build_log.txt").unwrap();
///
/// cargo_build::build_out::set(file);
///
/// cargo_build::rerun_if_changed(["README.md"]);
///
/// let out = std::fs::read_to_string("target/cargo_build_log.txt").unwrap();
///
/// assert_eq!(out, "cargo::rerun-if-changed=README.md\n");
/// ```
pub fn set(wr: impl Write + 'static) {
    CARGO_BUILD_OUT.set(Box::new(wr));
}

/// Use this function to reset output stream of `cargo-build` commands to `stdout`. This is necassery for
/// `cargo-build` commands to work inside `build.rs`.
///
/// `stdout` is the default. There is no need to reset output stream of `cargo-build` commands if it wasn't
/// previously changed by [`set`].
pub fn reset() {
    CARGO_BUILD_OUT.set(Box::new(stdout()));
}
