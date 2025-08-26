use super::cargo_build_out::CARGO_BUILD_OUT;
use std::io::Write;
use std::path::Path;

const ERR_MSG: &str = "Unable to write to cargo_build::CARGO_BUILD_OUT";

/// Tells Cargo to re-run the build script if file with given name changes.
///
/// ```rust
/// cargo_build::rerun_if_changed(["LICENSE.md", "README.md"]);
/// cargo_build::rerun_if_changed(["docs_folder"]);
///
/// cargo_build::rerun_if_changed(["src/main.rs"]);
/// ```
///
/// The `rerun-if-changed` instruction tells Cargo to re-run the build script if the file at
/// the given path has changed. Currently, Cargo only uses the filesystem last-modified “mtime”
/// timestamp to determine if the file has changed. It compares against an internal cached
/// timestamp of when the build script last ran.
///
/// If the path points to a directory, it will scan the entire directory for any modifications.
///
/// If the build script inherently does not need to re-run under any circumstance, then using
/// `cargo_build::rerun_if_changed("build.rs")` is a simple way to prevent it from being re-run
/// (otherwise, the default if no `rerun-if` instructions are emitted is to scan the entire
/// package directory for changes). Cargo automatically handles whether or not the script itself
/// needs to be recompiled, and of course the script will be re-run after it has been recompiled.
/// Otherwise, specifying build.rs is redundant and unnecessary.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
pub fn rerun_if_changed(file_paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for file_path in file_paths {
        let file_path = file_path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rerun-if-changed={file_path}").expect(ERR_MSG)
        });
    }
}

/// Tells Cargo to re-run the build script if environment variable with the given name has changed.
///
/// ```rust
/// cargo_build::rerun_if_env_changed(["LOG", "VERBOSE"]);
/// ```
///
/// Note that the environment variables here are intended for global environment variables like
/// CC and such, it is not possible to use this for environment variables like TARGET that Cargo
/// sets for build scripts. The environment variables in use are those received by cargo
/// invocations, not those received by the executable of the build script.
///
/// As of 1.46, using `env!` and `option_env!` in source code will automatically detect changes
/// and trigger rebuilds. `rerun-if-env-changed` is no longer needed for variables already
/// referenced by these macros.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
pub fn rerun_if_env_changed<'a>(env_vars: impl IntoIterator<Item = &'a str>) {
    for env_var in env_vars {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rerun-if-env-changed={env_var}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for benchmarks, binaries, `cdylib` crates, examples, and tests.
///
/// The `rustc-link-arg` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building supported targets (benchmarks,
/// binaries, cdylib crates, examples, and tests). Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// ### Examples
///
/// ```rust
/// cargo_build::rustc_link_arg(["-mlongcalls", "-ffunction-sections"]);
/// cargo_build::rustc_link_arg(["-Wl,--cref"]);
///
/// if cfg!(target_os = "windows") {
///     cargo_build::rustc_link_arg([
///         "/WX",
///         "/MANIFEST:EMBED",
///         &format!("/stack:{}", 8 * 1024 * 1024)
///     ]);
/// }
/// ```
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
pub fn rustc_link_arg<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT
            .with(|out| writeln!(out.borrow_mut(), "cargo::rustc-link-arg={flag}").expect(ERR_MSG));
    }
}

/// Passes custom flags to a linker for `cdylib` crates.
///
/// The `rustc-link-arg-cdylib` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building `cdylib` crates. Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg>
///
/// ### Examples
///
/// ```rust
/// cargo_build::rustc_link_arg_cdylib([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
pub fn rustc_link_arg_cdylib<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-cdylib={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for specific binary name.
///
/// ```rust
/// cargo_build::rustc_link_arg_bin("server", ["-Wl,--cref"]);
///
/// cargo_build::rustc_link_arg_bin("client", [
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// The `rustc-link-arg-bin` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building specified binary target. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all binaries see [`rustc_link_arg_bins`]
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-bin-link-arg>
pub fn rustc_link_arg_bin<'b, 'f>(bin: &'b str, flags: impl IntoIterator<Item = &'f str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-bin={bin}={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for binaries.
///
/// ```rust
/// cargo_build::rustc_link_arg_bins([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// The `rustc-link-arg-bins` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building binary targets. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins>
pub fn rustc_link_arg_bins<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-bins={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for tests.
///
/// ```rust
/// cargo_build::rustc_link_arg_tests([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// The `rustc-link-arg-tests` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building tests. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests>
pub fn rustc_link_arg_tests<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-tests={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for examples.
///
/// ```rust
/// cargo_build::rustc_link_arg_examples([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// The `rustc-link-arg-examples` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building examples. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples>
pub fn rustc_link_arg_examples<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-examples={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for benches.
///
/// ```rust
/// cargo_build::rustc_link_arg_benches([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// The `rustc-link-arg-benches` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building benches. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// If you want to pass flags for all supported targets see [`rustc_link_arg`]
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches>
pub fn rustc_link_arg_benches<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-arg-benches={flag}").expect(ERR_MSG)
        });
    }
}

/// Adds a library to link
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
/// ```rust
/// cargo_build::rustc_link_lib(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib([
///     "nghttp2",
///     "static=libssl",
///     "dylib=libcrypto",
///     "static:+whole-archive=mylib:renamed_lib",
/// ]);
/// ```
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
pub fn rustc_link_lib<'a>(libs: impl IntoIterator<Item = &'a str>) {
    for lib in libs {
        CARGO_BUILD_OUT
            .with(|out| writeln!(out.borrow_mut(), "cargo::rustc-link-lib={lib}").expect(ERR_MSG));
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `dylib=`
///
/// ```rust
/// cargo_build::rustc_link_lib_dylib(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_dylib([
///     ":+whole-archive=mylib:renamed_lib",
/// ]);
/// ```
pub fn rustc_link_lib_dylib<'a>(libs: impl IntoIterator<Item = &'a str>) {
    for lib in libs {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-lib=dylib={lib}").expect(ERR_MSG)
        });
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `static=`
///
/// ```rust
/// cargo_build::rustc_link_lib_static(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_static([
///     ":+whole-archive=mylib:renamed_lib",
/// ]);
/// ```
pub fn rustc_link_lib_static<'a>(libs: impl IntoIterator<Item = &'a str>) {
    for lib in libs {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-lib=static={lib}").expect(ERR_MSG)
        });
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `framework=`
///
/// ```rust
/// cargo_build::rustc_link_lib_framework(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_framework([
///     ":+whole-archive=mylib:renamed_lib",
/// ]);
/// ```
pub fn rustc_link_lib_framework<'a>(libs: impl IntoIterator<Item = &'a str>) {
    for lib in libs {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-lib=framework={lib}").expect(ERR_MSG)
        });
    }
}

/// Adds a directory to the library search path
///
/// The `rustc-link-lib` instruction tells Cargo to pass the
/// [`-L` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// to the compiler to add a directory to the library search path.
///
/// The kind of search path can optionally be specified with the form `-L KIND=PATH`.
///
/// The optional `KIND` may be one of `dependency`, `crate`, `native`, `framework`, or `all`.
/// See the [rustc book](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// for more detail.
///
/// ```rust
/// cargo_build::rustc_link_search(["libs"]);
///
/// cargo_build::rustc_link_search([
///     "native=libs",
///     "framework=mac_os_libs"
/// ]);
/// ```
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
pub fn rustc_link_search(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(
                out.borrow_mut(),
                "cargo::rustc-link-search={}",
                path.as_ref().display()
            )
            .expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `native=`
///
/// ```rust
/// cargo_build::rustc_link_search_native(["libs", "vendor", "api"]);
/// ```
pub fn rustc_link_search_native(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-search=native={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `dependency=`
///
/// ```rust
/// cargo_build::rustc_link_search_dependency(["libs", "vendor", "api"]);
/// ```
pub fn rustc_link_search_dependency(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(
                out.borrow_mut(),
                "cargo::rustc-link-search=dependency={path}"
            )
            .expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `crate=`
///
/// ```rust
/// cargo_build::rustc_link_search_crate(["libs", "vendor", "api"]);
/// ```
pub fn rustc_link_search_crate(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-search=crate={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `framework=`
///
/// ```rust
/// cargo_build::rustc_link_search_framework(["libs", "vendor", "api"]);
/// ```
pub fn rustc_link_search_framework(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(
                out.borrow_mut(),
                "cargo::rustc-link-search=framework={path}"
            )
            .expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `all=`
///
/// ```rust
/// cargo_build::rustc_link_search_all(["libs", "vendor", "api"]);
/// ```
pub fn rustc_link_search_all(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-link-search=all={path}").expect(ERR_MSG)
        });
    }
}

/// Passes certain flags to the compiler.
///
/// The `rustc-flags` instruction tells Cargo to pass the given space-separated flags to the compiler.
/// This only allows the `-l` and `-L` flags.
///
/// This function is is equivalent to using [`rustc_link_lib`] and [`rustc_link_search`].
///
/// ```rust
/// cargo_build::rustc_flags(["-L libs -L common_libs"]);
///
/// cargo_build::rustc_flags([
///     "-l ffi",
///     "-l ncursesw",
///     "-l stdc++",
///     "-l z"
/// ]);
/// ```
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags>
pub fn rustc_flags<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-flags={flag}").expect(ERR_MSG);
        });
    }
}

/// Enables custom compile-time `cfg` settings.
///
/// #### Register `cfg` options with [`rustc_check_cfg`] to avoid `unexpected_cfgs` warnings.
///
/// Import [`RustcCfgValue`] trait to be able to call [`RustcCfgValue::value`] on [`str`]:
///
/// ```rust
/// // build.rs
/// use cargo_build::StrExtRustcCfg;
///
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
/// cargo_build::rustc_cfg("api_version".value("1"));
/// ```
///
/// The `rustc-cfg` instruction tells Cargo to pass the given value to the
/// [`--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-cfg) to the compiler.
/// This may be used for compile-time detection of features to enable
/// [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html).
///
/// Custom cfgs must either be defined using the [`rustc_check_cfg`] instruction
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
/// For example, using `cargo_build::rustc_cfgs(["abc"])` will then allow code to use `#[cfg(abc)]` (note the lack
/// of `feature=`). Or an arbitrary key/value pair may be used like
/// `cargo_build::rustc_cfg("my_component".value("foo"))` which enables `#[cfg(my_component="foo")]` code blocks.
/// The key should be a Rust identifier, the value should be a string.
///
/// #### Example: Set `cfg` option when environment variable is present:
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs(["cuda"]);
///
/// if std::env::var("CUDA_PATH").is_ok() {
///     cargo_build::rustc_cfg("cuda");
/// }
/// // main.rs
/// #[cfg(cuda)]
/// mod cuda;
/// ```
///
/// #### Example: Set custom `cfg` options:
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs(["api_v1", "api_v2"]);
/// cargo_build::rustc_cfg("api_v1");
///
/// // main.rs
/// #[cfg(api_v1)]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_v2)]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
///
/// #### Example: Set range of custom `cfg` options:
/// ```rust
/// // build.rs
/// use cargo_build::StrExtRustcCfg;
///
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
/// cargo_build::rustc_cfg("api_version".value("1"));
/// ```
///
/// #### Example: Explicit `cfg` argument:
/// ```
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// use cargo_build::cfg;
///
/// cargo_build::rustc_cfg(cfg("api_version", Some("1")));
/// cargo_build::rustc_cfg(cfg("custom_cfg", None));
/// ```
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg>
pub fn rustc_cfg<'a>(cfg: impl Into<RustcCfg<'a>>) {
    let RustcCfg { name, value } = cfg.into();
    CARGO_BUILD_OUT.with(|out| match value {
        None => writeln!(out.borrow_mut(), "cargo::rustc-cfg={name}").expect(ERR_MSG),
        Some(value) => {
            writeln!(out.borrow_mut(), "cargo::rustc-cfg={name}=\"{value}\"").expect(ERR_MSG)
        }
    });
}

/// Helper struct for [`rustc_cfg`] function.
///
/// Import [`RustcCfgValue`] trait to be able to call [`RustcCfgValue::value`] on [`str`]:
///
/// ```rust
/// // build.rs
/// use cargo_build::StrExtRustcCfg;
///
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// cargo_build::rustc_cfg("api_version".value("1"));
///
/// // Be more explicit
/// use cargo_build::RustcCfg;
///
/// let cfg: RustcCfg = "api_version".value("1");
/// let cfg = RustcCfg::new("api_version", Some("1"));
/// ```
pub struct RustcCfg<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> RustcCfg<'a> {
    pub fn new(name: &'a str, value: Option<&'a str>) -> Self {
        Self { name, value }
    }
}

impl<'a> From<&'a str> for RustcCfg<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            name: value,
            value: None,
        }
    }
}

/// Helper function that allows explicit [`RustcCfg`] creation.
pub fn cfg<'a>(name: &'a str, value: Option<&'a str>) -> RustcCfg<'a> {
    RustcCfg { name, value }
}

/// Helper trait for [`rustc_cfg`] function.
///
/// Import this trait to be able to call [`RustcCfgValue::value`] on [`str`]:
///
/// ```rust
/// // build.rs
/// use cargo_build::StrExtRustcCfg;
///
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// cargo_build::rustc_cfg("api_version".value("1"));
/// ```
pub trait StrExtRustcCfg<'a> {
    fn value(&'a self, value: &'a str) -> RustcCfg<'a>;
}

impl<'a> StrExtRustcCfg<'a> for &'a str {
    fn value(&'a self, value: &'a str) -> RustcCfg<'a> {
        let mut cfg: RustcCfg = (*self).into();
        cfg.value = Some(value);
        cfg
    }
}

/// Define expected `cfg` names and values. Those names are used when checking the *reachable* `cfg` expressions
/// with the `unexpected_cfgs` lint.
///
/// Note that this function only *defines* expected config names. See [`rustc_cfg`] to set
/// `cfg` option during `build.rs` run.
///
/// - see [`rustc_check_cfgs`] to register multiple `cfg` names without values.
///
/// Note that all possible cfgs should be defined, regardless of which cfgs are currently enabled. This includes
/// all possible values of a given `cfg` name.
///
/// It is recommended to group the [`rustc_check_cfg`] and [`rustc_cfg`] functions as closely
/// as possible in order to avoid typos, missing check-cfg, stale cfgs..
///
/// #### Example: Register `cfg` option without values, then enable it using `rustc_cfg`.
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg("cuda", []); // [] indicates no values
/// cargo_build::rustc_check_cfgs(["cuda"]);
///
/// cargo_build::rustc_cfg("cuda");
///
/// // main.rs
/// #[cfg(cuda)]
/// mod cuda;
/// ```
///
/// #### Example: Register multiple related `cfg` options without values:
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs(["api_v1", "api_v2"]);
///
/// cargo_build::rustc_cfg("api_v1");
///
/// // main.rs
/// #[cfg(api_v1)]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_v2)]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
///
/// #### Example: Register range of custom `cfg` options:
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// use cargo_build::StrExtRustcCfg;
/// cargo_build::rustc_cfg("api_version".value("1"));
///
/// // main.rs
/// #[cfg(api_version="1")]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_version="2")]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--check-cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-check-cfg).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-check-cfg>
pub fn rustc_check_cfg<'a>(name: &'a str, values: impl IntoIterator<Item = &'a str>) {

    let values: String = values
        .into_iter()
        .map(|e| format!("\"{e}\""))
        .collect::<Vec<String>>()
        .join(", ");

    CARGO_BUILD_OUT.with(|out| {
        if values.is_empty() {
            writeln!(out.borrow_mut(), "cargo::rustc-check-cfg=cfg({name})").expect(ERR_MSG);
        } else {
            writeln!(
                out.borrow_mut(),
                "cargo::rustc-check-cfg=cfg({name}, values({values}))"
            )
            .expect(ERR_MSG);
        }
    });
}

/// Register expected config names and values. Those names are used when checking the *reachable* cfg expressions
/// with the `unexpected_cfgs` lint.
///
/// This function is [`rustc_check_cfg`] alternative with multiple arguments.
pub fn rustc_check_cfgs<'a>(names: impl IntoIterator<Item = &'a str>) {
    for name in names {
        CARGO_BUILD_OUT.with(|out| {
            writeln!(out.borrow_mut(), "cargo::rustc-check-cfg=cfg({name})").expect(ERR_MSG);
        });
    }
}

/// Sets an environment variable
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
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env>
pub fn rustc_env(var: &str, value: &str) {
    CARGO_BUILD_OUT.with(|out| {
        writeln!(out.borrow_mut(), "cargo::rustc-env={var}={value}").expect(ERR_MSG);
    });
}

/// Displays an error on the terminal.
///
/// The error instruction tells Cargo to display an error after the build script has finished running, and then fail the build.
///
/// Note: Build script libraries should carefully consider if they want to use `cargo::error` versus returning a `Result`.
/// It may be better to return a `Result`, and allow the caller to decide if the error is fatal or not. The caller can then
/// decide whether or not to display the `Err` variant using `cargo::error`.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-error>
pub fn error(msg: &str) {
    CARGO_BUILD_OUT.with(|out| {
        writeln!(out.borrow_mut(), "cargo::error={msg}").expect(ERR_MSG);
    });
}

/// Displays a warning on the terminal.
///
/// The `warning` instruction tells Cargo to display a warning after the build script has finished running. Warnings are
/// only shown for `path` dependencies (that is, those you’re working on locally), so for example warnings printed out in
/// [crates.io](https://crates.io/) crates are not emitted by default, unless the build fails. The `-vv` "very verbose"
/// flag may be used to have Cargo display warnings for all crates.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning>
pub fn warning(msg: &str) {
    CARGO_BUILD_OUT.with(|out| {
        writeln!(out.borrow_mut(), "cargo::warning={msg}").expect(ERR_MSG);
    });
}

/// Metadata, used by links scripts.
///
/// The `package.links` key may be set in the `Cargo.toml` manifest to declare that the package links with the given native
/// library. The purpose of this manifest key is to give Cargo an understanding about the set of native dependencies that a
/// package has, as well as providing a principled system of passing metadata between package build scripts.
///
/// ```toml
/// [package]
/// # ...
/// links = "foo"
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
/// ```ignore
/// // Cargo.toml
/// [package]
/// ..
/// links = "tiledb"
///
/// // build.rs
/// cargo_build::metadata("LINKAGE", "static");
/// cargo_build::rustc_link_search_native("libs");
/// cargo_build::rustc_link_lib_static("tiledb_static");
/// ```
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key>
pub fn metadata(key: &str, value: &str) {
    CARGO_BUILD_OUT.with(|out| {
        writeln!(out.borrow_mut(), "cargo::metadata={key}={value}").expect(ERR_MSG);
    });
}
