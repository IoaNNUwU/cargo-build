#[macro_export]
macro_rules! warning {
    () => {
        println!("cargo::warning=");
    };
    ($($arg:tt)*) => {{
        print!("cargo::warning=");
        println!($($arg)*)
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        println!("cargo::error=");
    };
    ($($arg:tt)*) => {{
        print!("cargo::error=");
        println!($($arg)*)
    }};
}
