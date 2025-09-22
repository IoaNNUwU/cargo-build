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
/// let ext = ".toml";
/// let lang = "rust";
///
/// // Each line follows `format!` macro syntax.
/// cargo_build::rerun_if_changed!(
///     "cfg{}", ext;
///     "lib-{}", lang;
/// );
///
/// // Format runtime variables
/// let host = std::env::var("OS").unwrap_or("linux".to_string());
/// cargo_build::rerun_if_changed!("{host}");
/// ```
///
/// - See [`rerun_if_changed` function](`crate::functions::rerun_if_changed`) if you dont need
///   strings interpolation and are ok with using `rerun_if_changed(["file1", "file2"])` syntax.
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
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rerun-if-changed=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
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
///   need strings interpolation and are ok with using `rerun_if_env_changed(["ENV1", "ENV2"])` syntax.
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
    ( $( $( $fmt_arg:tt ),* );* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rerun-if-env-changed=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};
}

/// Passes custom flags to a linker for specified `target`. If no target was specified, passes flags to all
/// supported targets, such as `becnches`, `bins`, `cdylib` crates, `examples`, `tests`.
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
///   need strings interpolation and are ok with using `rustc_link_arg(["arg1", "arg2"])` syntax.
///
/// Use this macro to set linker flags for specific targets by providing optional `target`:
///
/// ```rust
/// cargo_build::rustc_link_arg!(benches: "-mlongcalls"; "-ffunction-sections");
/// cargo_build::rustc_link_arg!(bins: "-mlongcalls"; "-ffunction-sections");
/// cargo_build::rustc_link_arg!(cdylib: "-mlongcalls"; "-ffunction-sections");
/// cargo_build::rustc_link_arg!(examples: "-mlongcalls"; "-ffunction-sections");
/// cargo_build::rustc_link_arg!(tests: "-mlongcalls"; "-ffunction-sections");
///
/// ```
///
/// ```rust
/// cargo_build::rustc_link_arg!(bin "server": "-mlongcalls"; "-ffunction-sections");
///
/// cargo_build::rustc_link_arg!(
///     bin "client":
///       "-mlongcalls";
///       "-ffunction-sections";
///       "-Wl,--cref";
///       "stack-size={}", { 8 * 1024 * 1024 };
/// );
/// ```
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
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( benches: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-benches=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( bins: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-bins=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( bin $bin_name:tt : $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-bin=").expect("Unable to write to CARGO_BUILD_OUT");
                    write!(out, "{}=", $bin_name).expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( cdylib: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-cdylib=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( examples: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-examples=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( tests: $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-arg-tests=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};
}

/// Adds a library to link.
///
/// ```rust
/// cargo_build::rustc_link_lib!("nghttp2"; "libssl"; "libcrypto");
///
/// cargo_build::rustc_link_lib!(static: "+whole-archive" = "nghttp2"; "libssl");
/// cargo_build::rustc_link_lib!(dylib: "+verbatim", "+bundle" = "nghttp2"; "libssl");
/// cargo_build::rustc_link_lib!(framework = "nghttp2"; "libssl");
///
/// cargo_build::rustc_link_lib!(
///     static: "+whole-archive", "+verbatim", "+bundle" =
///         "nghttp2";
///         "libssl";
///         "libcrypto";
///         "mylib:{}", "renamed_lib";
/// );
/// ```
///
/// Supports variable number of arguments. They are separated by `;` to allow interpolation
/// using `format!` macro syntax for each argument. Format string and arguments are separated by `,`.
///
/// - See [`rustc_link_lib` function](`crate::functions::rustc_link_lib`) if you dont
///   need strings interpolation and are ok with using `rustc_link_lib(["lib1", "lib2"])` syntax.
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
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[macro_export]
macro_rules! rustc_link_lib {

    () => {};

    ( static $(: $mod1:tt $(, $mod_rem:tt )* )? = $( $($fmt_arg:tt),* );*) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let closure = move || {
                let mut out = out.borrow_mut();
                write!(out, "cargo::rustc-link-lib=static").expect("Unable to write to CARGO_BUILD_OUT");
                $(
                    write!(out, ":").expect("Unable to write to CARGO_BUILD_OUT");
                    write!(out, $mod1).expect("Unable to write to CARGO_BUILD_OUT");
                    $(
                        write!(out, ",{}", $mod_rem).expect("Unable to write to CARGO_BUILD_OUT");
                    )*
                )?
                write!(out, "=").expect("Unable to write to CARGO_BUILD_OUT");
            };

            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    closure();
                    writeln!(out.borrow_mut(), $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( dylib $(: $mod1:tt $(, $mod_rem:tt )* )? = $( $($fmt_arg:tt),* );*) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let closure = move || {
                let mut out = out.borrow_mut();
                write!(out, "cargo::rustc-link-lib=dylib").expect("Unable to write to CARGO_BUILD_OUT");
                $(
                    write!(out, ":").expect("Unable to write to CARGO_BUILD_OUT");
                    write!(out, $mod1).expect("Unable to write to CARGO_BUILD_OUT");
                    $(
                        write!(out, ",{}", $mod_rem).expect("Unable to write to CARGO_BUILD_OUT");
                    )*
                )?
                write!(out, "=").expect("Unable to write to CARGO_BUILD_OUT");
            };

            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    closure();
                    writeln!(out.borrow_mut(), $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( framework $(: $mod1:tt $(, $mod_rem:tt )* )? = $( $($fmt_arg:tt),* );*) => {{
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let closure = move || {
                let mut out = out.borrow_mut();
                write!(out, "cargo::rustc-link-lib=framework").expect("Unable to write to CARGO_BUILD_OUT");
                $(
                    write!(out, ":").expect("Unable to write to CARGO_BUILD_OUT");
                    write!(out, $mod1).expect("Unable to write to CARGO_BUILD_OUT");
                    $(
                        write!(out, ",{}", $mod_rem).expect("Unable to write to CARGO_BUILD_OUT");
                    )*
                )?
                write!(out, "=").expect("Unable to write to CARGO_BUILD_OUT");
            };

            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    closure();
                    writeln!(out.borrow_mut(), $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( $( $($fmt_arg:tt),* );* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-lib=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};
}

/// Adds a directory to the library search path.
///
/// ```rust
/// cargo_build::rustc_link_search!("libs");
///
/// cargo_build::rustc_link_search!(native="libs");
/// cargo_build::rustc_link_search!(framework="mac_os_libs"; "more_mac_os_libs"; );
/// ```
///
/// The `rustc-link-search` instruction tells Cargo to pass the
/// [`-L` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// to the compiler to add a directory to the library search path.
///
/// The kind of search path can optionally be specified with the form `-L KIND=PATH`.
///
/// The optional `KIND` may be one of `dependency`, `crate`, `native`, `framework`, or `all` (the default).
/// See the [rustc book](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// for more detail.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[macro_export]
macro_rules! rustc_link_search {
    () => {};

    ( framework = $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=framework=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( framework : $($($fmt_arg:tt),*);* ) => {{
        $crate::rustc_link_search!(framework = $($($fmt_arg),*);*)
    }};

    ( native = $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=native=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( native : $($($fmt_arg:tt),*);* ) => {{
        $crate::rustc_link_search!(native = $($($fmt_arg),*);*)
    }};

    ( crate = $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=crate=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( crate : $($($fmt_arg:tt),*);* ) => {{
        $crate::rustc_link_search!(crate = $($($fmt_arg),*);*)
    }};

    ( dependency = $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=dependency=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( dependency : $($($fmt_arg:tt),*);* ) => {{
        $crate::rustc_link_search!(dependency = $($($fmt_arg),*);*)
    }};

    ( all = $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=all=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};

    ( all : $($($fmt_arg:tt),*);* ) => {{
        $crate::rustc_link_search!(all = $($($fmt_arg),*);*)
    }};

    ( $($($fmt_arg:tt),*);* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                if const { !matches!(stringify!($($fmt_arg),*).as_bytes(), b"") } {
                    write!(out, "cargo::rustc-link-search=").expect("Unable to write to CARGO_BUILD_OUT");
                    writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
                }
            )*
        });
    }};
}

/// Enables custom compile-time `cfg` settings.
///
/// #### Register all `cfg` options with [`rustc_check_cfg`] to avoid `unexpected_cfgs` warnings.
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg!("custom_cfg");
///
/// cargo_build::rustc_cfg!("custom_cfg");
///
/// // main.rs
/// #[cfg(custom_cfg)]
/// mod optional_mod;
///
/// ```
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg!("api_version": "1", "2", "3");
///
/// cargo_build::rustc_cfg!("api_version" = "1");
///
/// // main.rs
/// #[cfg(api_version="1")]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_version="2")]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
///
/// The `rustc-cfg` instruction tells Cargo to pass the given value to the
/// [`--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-cfg) to the compiler.
/// This may be used for compile-time detection of features to enable
/// [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html).
///
/// Custom cfgs must either be defined using the [`rustc_check_cfg!`] instruction
/// or usage will need to allow `the unexpected_cfgs lint` to avoid
/// [unexpected cfgs](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unexpected-cfgs) warnings.
///
/// Note that this does not affect Cargo’s dependency resolution. This cannot be used to enable an optional
/// dependency, or enable other Cargo features.
///
/// Be aware that [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
/// use the form `feature="foo"` and `#[cfg(feature = "foo")]` in code. `cfg` values passed with this flag are
/// not restricted to that form, and may provide just a single identifier, or any arbitrary key/value pair.
///
/// For example, using `cargo_build::rustc_cfg!("abc")` will then allow code to use `#[cfg(abc)]` (note the lack
/// of `feature=`). Or an arbitrary key/value pair may be used like
/// `cargo_build::rustc_cfg!("my_component" = "foo")` which enables `#[cfg(my_component="foo")]` code blocks.
/// The key should be a Rust identifier, the value should be a string.
///
/// See [`rustc_check_cfg!`] for more information on custom `cfg`s definitions.
///
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg>
#[macro_export]
macro_rules! rustc_cfg {
    () => {};
    ( $cfg_name:tt ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-cfg={}", $cfg_name)
                .expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};
    ( $cfg_name:tt = $cfg_value:tt ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-cfg={}=\"{}\"", $cfg_name, $cfg_value)
                .expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};
}

/// Define expected `cfg` names and values. Those names are used when checking the *reachable* `cfg` expressions
/// with the `unexpected_cfgs` lint.
///
/// #### Note that this function only *defines* expected config names. See [`rustc_cfg!`] to set `cfg` option during `build.rs` run.
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg!("custom_cfg");
///
/// cargo_build::rustc_cfg!("custom_cfg");
///
/// // main.rs
/// #[cfg(custom_cfg)]
/// mod optional_mod;
///
/// ```
/// ```
/// // build.rs
/// cargo_build::rustc_check_cfg!("api_version": "1", "2", "3"); // or
/// cargo_build::rustc_check_cfg!("api_version": ["1", "2", "3"]);
///
/// cargo_build::rustc_cfg!("api_version" = "1");
///
/// // main.rs
/// #[cfg(api_version="1")]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_version="2")]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
///
/// Note that all possible cfgs should be defined, regardless of which cfgs are currently enabled. This includes
/// all possible values of a given `cfg` name.
///
/// It is recommended to group the [`rustc_check_cfg!`] and [`rustc_cfg!`] functions as closely
/// as possible in order to avoid typos, missing check-cfg, stale cfgs..
///
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--check-cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-check-cfg).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-check-cfg>
#[macro_export]
macro_rules! rustc_check_cfg {
    () => {};

    ( $cfg_name:tt ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-check-cfg=cfg({})", $cfg_name).expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};

    ( $cfg_name:tt : [ $( $cfg_value:tt ),+ ]) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            write!(out, "cargo::rustc-check-cfg=cfg({}, values(", $cfg_name).expect("Unable to write to CARGO_BUILD_OUT");
            let mut values = String::new();
            use std::fmt::Write;
            $(
                write!(values, "\"{}\", ", $cfg_value).expect("Unable to write to CARGO_BUILD_OUT");
            )+
            values.pop();
            values.pop();
            writeln!(out, "{}))", &values).expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};

    ( $cfg_name:tt : $( $cfg_value:tt ),+ ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            write!(out, "cargo::rustc-check-cfg=cfg({}, values(", $cfg_name).expect("Unable to write to CARGO_BUILD_OUT");
            let mut values = String::new();
            use std::fmt::Write;
            $(
                write!(values, "\"{}\", ", $cfg_value).expect("Unable to write to CARGO_BUILD_OUT");
            )+
            values.pop();
            values.pop();
            writeln!(out, "{}))", &values).expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};

    ( $( $cfg_name:tt ),* ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            $(
                writeln!(out, "cargo::rustc-check-cfg=cfg({})", $cfg_name).expect("Unable to write to CARGO_BUILD_OUT");
            )*
        });
    }};
}

/// Sets an environment variable.
///
/// #### Example: Automatically insert env variable during compile time.
/// ```ignore
/// // build.rs
/// use std::process::Command;
///
/// let com_out = Command::new("git").args(["rev-parse", "HEAD"]).output().unwrap();
/// let git_hash = String::from_utf8(com_out.stdout).unwrap();
///
/// cargo_build::rustc_env("GIT_HASH", &git_hash);
///
/// // main.rs
/// const EMBEDDED_GIT_HASH: &str = env!("GIT_HASH");
/// ```
///
/// The `rustc-env` instruction tells Cargo to set the given environment variable when
/// compiling the package. The value can be then retrieved by the
/// [`env!` macro](https://doc.rust-lang.org/std/macro.env.html) in the compiled crate.
/// This is useful for embedding additional metadata in crate’s code, such as the hash
/// of git HEAD or the unique identifier of a continuous integration server.
///
/// See also the [environment variables automatically included by Cargo](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).
///
/// Note: These environment variables are also set when running an executable with `cargo run`
/// or `cargo test`. However, this usage is discouraged since it ties the executable to Cargo’s
/// execution environment. Normally, these environment variables should only be checked at
/// compile-time with the `env!` macro.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env>
#[macro_export]
macro_rules! rustc_env {
    () => {};

    ( $env_name:tt = $env_value:tt ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-env={}={}", $env_name, $env_value)
                .expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};
}

/// Displays a warning on the terminal.
///  
/// ```rust
/// let err = "Unable to find a file";
/// cargo_build::warning!("Warning during build: {}", err);
/// ```
///
/// The `warning` instruction tells Cargo to display a warning after the build script has finished running. Warnings are
/// only shown for `path` dependencies (that is, those you’re working on locally), so for example warnings printed out in
/// [crates.io](https://crates.io/) crates are not emitted by default, unless the build fails. The `-vv` "very verbose"
/// flag may be used to have Cargo display warnings for all crates.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning>
#[macro_export]
macro_rules! warning {
    ( $($fmt_arg:tt),* ) => {
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            write!(out, "cargo::warning=").expect("Unable to write to CARGO_BUILD_OUT");
            writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
        });
    };
}

/// Displays an error on the terminal.
///
/// #### This error fails the build even if all the other steps finished successfully.
///
/// ```rust
/// let err = "Unable to find a file";
/// cargo_build::error!("Fatal error during build: {}", err);
/// ```
///
/// The error instruction tells Cargo to display an error after the build script has finished running, and then fail the build.
///
/// Note: Build script libraries should carefully consider if they want to use `cargo::error` versus returning a `Result`.
/// It may be better to return a `Result`, and allow the caller to decide if the error is fatal or not. The caller can then
/// decide whether or not to display the `Err` variant using `cargo::error`.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-error>
#[macro_export]
macro_rules! error {
    ( $($fmt_arg:tt),* ) => {
        $crate::build_out::CARGO_BUILD_OUT.with(|out| {
            let mut out = out.borrow_mut();
            write!(out, "cargo::error=").expect("Unable to write to CARGO_BUILD_OUT");
            writeln!(out, $($fmt_arg),*).expect("Unable to write to CARGO_BUILD_OUT");
        });
    };
}

/// Metadata, used by links scripts.
///
/// The `package.links` key may be set in the `Cargo.toml` manifest to declare that the package links with the given native
/// library. The purpose of this manifest key is to give Cargo an understanding about the set of native dependencies that a
/// package has, as well as providing a principled system of passing metadata between package build scripts.
///
/// ```toml
/// // Cargo.toml
/// [package]
/// ..
/// links = "foo"
/// ```
/// ```rust
/// // build.rs
/// cargo_build::metadata("LINKAGE", "static");
/// cargo_build::rustc_link_search_native(["libs"]);
/// cargo_build::rustc_link_lib_static(["foo"]);
/// ```
///
/// This manifest states that the package links to the `libfoo` native library. When using the `links` key, the package must
/// have a build script, and the build script should use the [`rustc_link_lib`] instruction to link the library.
///
/// Primarily, Cargo requires that there is at most one package per `links` value. In other words, it is forbidden to have two
/// packages link to the same native library. This helps prevent duplicate symbols between crates. Note, however, that there
/// are [conventions in place](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages) to alleviate this.
///
/// Build scripts can generate an arbitrary set of metadata in the form of key-value pairs.
/// This metadata is set with the [`metadata`] instruction.
///
/// The metadata is passed to the build scripts of **dependent** packages. For example, if the package `foo` depends on `bar`, which links
/// `baz`, then if `bar` generates `key=value` as part of its build script metadata, then the build script of `foo` will have the environment
/// variables `DEP_BAZ_KEY=value` (note that the value of the `links` key is used). See the
/// [Using another `sys` crate](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#using-another-sys-crate)
/// for an example of how this can be used.
///
/// Note that metadata is only passed to immediate dependents, not transitive dependents.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key>
#[macro_export]
macro_rules! metadata {
    () => {};
    ( $meta_key:tt = $meta_value:tt ) => {{
        $crate::build_out::CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::metadata={}={}", $meta_key, $meta_value)
                .expect("Unable to write to CARGO_BUILD_OUT");
        });
    }};
}
