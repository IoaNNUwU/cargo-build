use std::{io::Write, sync::RwLock};

struct TestWriteVec(RwLock<Vec<u8>>);

impl TestWriteVec {
    const fn new() -> Self {
        TestWriteVec(RwLock::new(Vec::new()))
    }
}

impl Write for &TestWriteVec {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut vec = self.0.write().expect("Unable to aquire write Lock");
        vec.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "macros")]
#[cfg(test)]
mod macro_tests {}

#[cfg(feature = "functions")]
#[cfg(test)]
mod functions_tests {

    use crate::{self as cargo_build, test::TestWriteVec};

    #[test]
    fn change_output_test() {
        let vec_out: &'static TestWriteVec = Box::leak(Box::new(TestWriteVec::new()));

        cargo_build::CARGO_BUILD_OUT.set(vec_out);

        cargo_build::rerun_if_changed(["LICENSE.md"]);

        {
            let out: &[u8] = &vec_out.0.read().expect("Unable to aquire Read lock");
            assert_eq!(out, b"cargo::rerun-if-changed=LICENSE.md");
        }

        // Reset CARGO_BUILD_OUT and try again
        vec_out
            .0
            .write()
            .expect("Unable to aquire Write lock")
            .clear();

        cargo_build::CARGO_BUILD_OUT.reset();

        cargo_build::rerun_if_changed(["LICENSE.md"]);

        {
            let out: &[u8] = &vec_out.0.read().expect("Unable to aquire Read lock");
            assert_eq!(out, b"");
        }
    }
}
