use std::{
    io::Write,
    sync::{LazyLock, RwLock},
};

/// Use this static variable to change `cargo-build` commands output. Defaults to `stdout` - this way
/// cargo commands work inside `build.rs`.
///
/// Useful for debugging and logging.
///
/// ### Example `File` output
/// ```rust
/// use std::{io::Read, fs::File};
///
/// let file = File::create("target/build_output_test.txt").expect("Unable to create file");
///
/// cargo_build::CARGO_BUILD_OUT.set(file);
///
/// cargo_build::rerun_if_changed(["README.md"]);
///
/// let file_contents = std::fs::read_to_string("target/build_output_test.txt")
///                                 .expect("Unable to read file");
///
/// assert_eq!(&file_contents, "cargo::rerun-if-changed=README.md\n")
/// ```
///
/// ### Example `Vec` output
/// ```rust
/// use std::{sync::{Arc, Mutex}, io::Write};
///
/// struct MutexWriteVec(Mutex<Vec<u8>>);
///
/// impl Write for &MutexWriteVec {
///     fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
///         let vec: &mut Vec<u8> = &mut self.0.lock().unwrap();
///         vec.write(&buf)
///     }
///     fn flush(&mut self) -> Result<(), std::io::Error> { Ok(()) }
/// }
///
/// let write_vec: &'static MutexWriteVec = 
///         Box::leak(Box::new(MutexWriteVec(Mutex::new(Vec::new()))));
///
/// cargo_build::CARGO_BUILD_OUT.set(write_vec);
///
/// cargo_build::rerun_if_changed(["README.md"]);
///
/// let vec_contents: &[u8] = &write_vec.0.lock().unwrap();
///
/// assert_eq!(vec_contents, b"cargo::rerun-if-changed=README.md\n")
/// ```
pub static CARGO_BUILD_OUT: CargoBuildOut =
    CargoBuildOut(LazyLock::new(|| RwLock::new(Box::new(std::io::stdout()))));

/// Use [`CARGO_BUILD_OUT`] static variable to change output for cargo build instructions.
pub struct CargoBuildOut(LazyLock<RwLock<Box<dyn Write + Send + Sync>>>);

impl CargoBuildOut {
    
    /// Resets `CARGO_BUILD_OUT` to `stdout`.
    pub fn reset(&self) {
        let mut out = self.0.write().expect("Unable to acquire Write Lock");
        *out = Box::new(std::io::stdout());
    }

    /// Sets `CARGO_BUILD_OUT` to user provided `Write` implementation.
    /// 
    /// see [`CARGO_BUILD_OUT`] docs for examples.
    pub fn set(&self, cargo_build_out: impl Write + Send + Sync + 'static) {
        let mut out = self.0.write().expect("Unable to acquire Write Lock");
        *out = Box::new(cargo_build_out);
    }
}

impl Write for &CargoBuildOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut out = self.0.write().expect("Unable to acquire Write Lock");
        out.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
