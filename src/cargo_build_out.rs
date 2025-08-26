use std::{
    cell::RefCell,
    io::{stdout, Write},
};

thread_local! {
    pub(crate) static CARGO_BUILD_OUT: RefCell<Box<dyn Write>> = RefCell::new(Box::new(stdout()));
}

pub fn set(wr: impl Write + 'static) {
    CARGO_BUILD_OUT.set(Box::new(wr));
}

pub fn reset() {
    CARGO_BUILD_OUT.set(Box::new(stdout()));
}
