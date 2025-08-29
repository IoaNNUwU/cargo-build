/// Tells Cargo to re-run the build script **ONLY** if file or directory with given name changes.
///
/// The default if no `rerun-if` instructions are emitted is to scan the entire package
/// directory for changes.
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// ```rust
/// cargo_build::rerun_if_changed!("docs_folder");
/// cargo_build::rerun_if_changed!("src/main.c");
///
/// // Note the multiple arguments separated by `;`
/// cargo_build::rerun_if_changed!("LICENSE.md"; "README.md");
///
/// // Each line follows `format!` macro syntax.
/// cargo_build::rerun_if_changed!(
///     "cfg{}", ".toml";
///     "lib-{}", "rust"
/// );
/// // Format runtime variables
/// let host = std::env::var("OS").unwrap_or("linux".to_string());
/// cargo_build::rerun_if_changed!("{host}");
/// ```
///
/// - See [`rerun_if_changed` function](`crate::functions::rerun_if_changed`) if you dont need
/// strings interpolation and are ok with using `rerun_if_changed(["file1", "file2"])` syntax.
///
/// Currently, Cargo only uses the filesystem last-modified “mtime” timestamp to determine if the
/// file has changed. It compares against an internal cached timestamp of when the build script last ran.
///
/// If the path points to a directory, it will scan the entire directory for any modifications.
///
/// If the build script inherently does not need to re-run under any circumstance, then using
/// `cargo_build::rerun_if_changed(["build.rs"])` is a simple way to prevent it from being re-run.
/// Cargo automatically handles whether or not the script itself needs to be recompiled, and of course
/// the script will be re-run after it has been recompiled. Otherwise, specifying build.rs is redundant
/// and unnecessary.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
#[macro_export]
macro_rules! rerun_if_changed {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rerun-if-changed=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Tells Cargo to re-run the build script if environment variable with the given name has changed.
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// ```rust
/// cargo_build::rerun_if_env_changed!("LOG"; "VERBOSE");
///
/// let platform = "WINDOWS";
/// cargo_build::rerun_if_env_changed!("{}-HOME", platform);
/// ```
///
/// - See [`rerun_if_env_changed` function](`crate::functions::rerun_if_env_changed`) if you dont
/// need strings interpolation and are ok with using `rerun_if_env_changed(["ENV1", "ENV2"])` syntax.
///
/// Note that the environment variables here are intended for global environment variables like
/// `CC` and such, it is not possible to use this for environment variables like `TARGET` that Cargo
/// sets for build scripts. The environment variables in use are those received by cargo
/// invocations, not those received by the executable of the build script.
///
/// See [full list of `cargo` environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
///
/// As of 1.46, using `env!` and `option_env!` in source code will automatically detect changes
/// and trigger rebuilds. `rerun-if-env-changed` is no longer needed for variables already
/// referenced by these macros.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
#[macro_export]
macro_rules! rerun_if_env_changed {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rerun-if-env-changed=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for benchmarks, binaries, `cdylib` crates, examples, and tests.
///
/// To set linker flags for specific targets see [`rustc_link_arg_benches!`], [`rustc_link_arg_bins!`],
///   [`rustc_link_arg_cdylib!`], [`rustc_link_arg_examples!`], [`rustc_link_arg_tests!`].
///
/// ```rust
/// cargo_build::rustc_link_arg!("-mlongcalls"; "-ffunction-sections");
/// cargo_build::rustc_link_arg!("-Wl,--cref");
///
/// let stack_size = 8 * 1024 * 1024;
/// cargo_build::rustc_link_arg!(
///     "/stack:{stack_size}";
///     "/WX"
/// );
/// cargo_build::rustc_link_arg!(
///     "/stack:{}", { 8 * 1024 * 1024 };
///     "/WX"
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg` function](`crate::functions::rustc_link_arg`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building supported targets (benchmarks,
/// binaries, cdylib crates, examples, and tests). Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
#[macro_export]
macro_rules! rustc_link_arg {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for `cdylib` crates.
///
/// - To set linker flags for all supported targets see [`rustc_link_arg!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_cdylib!(
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_cdylib` function](`crate::functions::rustc_link_arg_cdylib`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_cdylib(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-cdylib` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building `cdylib` crates. Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg>
#[macro_export]
macro_rules! rustc_link_arg_cdylib {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-cdylib=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for specific binary name.
///
/// - To set linker flags for all bin targets see [`rustc_link_arg_bins!`].
/// - To set linker flags for all supported targets see [`rustc_link_arg!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_bin!("server": "-Wl,--cref");
///
/// cargo_build::rustc_link_arg_bin!(
///     "client":
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_bin` function](`crate::functions::rustc_link_arg_bin`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_bin("bin", ["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-bin` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building specified binary target. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-bin-link-arg>
#[macro_export]
macro_rules! rustc_link_arg_bin {
    () => {};
    ( $bin:tt: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-bin=").expect("Unable to write to CARGO_BUILD_OUT");
                write!(out, "{}", $bin).expect("Unable to write to CARGO_BUILD_OUT");
                write!(out, "=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for binaries.
///
/// To set linker flags for all supported targets see [`rustc_link_arg!`].
/// To set linker flags for specific binary target see [`rustc_link_arg_bin!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_bins!(
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_bins` function](`crate::functions::rustc_link_arg_bins`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_bins(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-bins` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building binary targets. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins>
#[macro_export]
macro_rules! rustc_link_arg_bins {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-bins=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for tests.
///
/// To set linker flags for all supported targets see [`rustc_link_arg!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_tests!(
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_tests` function](`crate::functions::rustc_link_arg_tests`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_tests(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-tests` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building tests. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests>
#[macro_export]
macro_rules! rustc_link_arg_tests {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-tests=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for examples.
///
/// To set linker flags for all supported targets see [`rustc_link_arg!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_examples!(
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_examples` function](`crate::functions::rustc_link_arg_examples`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_examples(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-examples` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building examples. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples>
#[macro_export]
macro_rules! rustc_link_arg_examples {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-examples=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Passes custom flags to a linker for benches.
///
/// To set linker flags for all supported targets see [`rustc_link_arg!`].
///
/// ```rust
/// cargo_build::rustc_link_arg_benches!(
///         "-mlongcalls";
///         "-ffunction-sections";
///         "-Wl,--cref";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_arg_benches` function](`crate::functions::rustc_link_arg_benches`) if you dont
/// need strings interpolation and are ok with using `rustc_link_arg_benches(["arg1", "arg2"])` syntax.
///
/// The `rustc-link-arg-benches` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building benches. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches>
#[macro_export]
macro_rules! rustc_link_arg_benches {
    () => {};
    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-arg-benches=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Adds a library to link.
///
/// ```rust
/// cargo_build::rustc_link_lib!("nghttp2", "libssl", "libcrypto");
///
/// cargo_build::rustc_link_lib!(
///     "nghttp2";
///     static="libssl";
///     dylib="libcrypto";
///     static:+whole-archive="mylib:{rename}", "renamed_lib";
/// );
/// ```
///
/// The `rustc-link-lib` instruction tells Cargo to link the given library using the compiler’s
/// [`-l` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-link-lib).
/// This is typically used to link a native library using [FFI](https://doc.rust-lang.org/nomicon/ffi.html).
///
/// Argument to this function is `LIB` string which which is directly passed to `rustc`.
/// Currently the fully supported syntax for LIB is `[KIND[:MODIFIERS]=]NAME[:RENAME]`.
///
/// The optional `KIND` may be one of `dylib`, `static`, or `framework`. See the
/// [rustc book](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-link-lib)
/// for more detail.
///
/// See more specific [`rustc_link_lib_dylib`], [`rustc_link_lib_static`], [`rustc_link_lib_static`],
/// [`rustc_link_lib_framework`].
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[macro_export]
macro_rules! rustc_link_lib {

    () => {};

    ( static $(: $mod1:tt $(, $mod2:tt $(, $mod3:tt)? )? )? = $( $($fmt_arg:tt),* );*) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            
            let closure = move || {
                let mut out = out.borrow_mut();
                write!(out, "cargo::rustc-link-lib=static").expect("Unable to write to CARGO_BUILD_OUT");
                $(
                    write!(out, ":").expect("Unable to write to CARGO_BUILD_OUT");
                    write!(out, $mod1).expect("Unable to write to CARGO_BUILD_OUT");
                    $(
                        write!(out, ",{}", $mod2).expect("Unable to write to CARGO_BUILD_OUT");
                        $(
                            write!(out, ",{}", $mod3).expect("Unable to write to CARGO_BUILD_OUT");
                        )?
                    )?
                )?
                write!(out, "=").expect("Unable to write to CARGO_BUILD_OUT");
            };

            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    
                    closure();

                    let mut out: std::cell::RefMut<_> = out.borrow_mut();
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( framework $(: ($mods:tt),*)? = $( $($fmt_arg:tt),* );* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-lib=static=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};

    ( dylib $(: ($mods:tt),*)? = $( $($fmt_arg:tt),* );* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-lib=static=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
    
    // "lib1"; "lib2:{rename}", "rename"; "lib3"
    ( $( $($fmt_arg:tt),* );* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::rustc-link-lib=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

#[macro_export]
macro_rules! warning {
    ($($fmt_arg:tt)*) => {
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::warning=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    };
}

#[macro_export]
macro_rules! error {
    ($($fmt_arg:tt)*) => {
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                write!(out, "cargo::error=").expect("Unable to write to CARGO_BUILD_OUT");
                writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    };
}
