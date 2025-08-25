use super::CARGO_BUILD_OUT;
use std::io::Write;
use std::path::Path;

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
/// If the build script inherently does not need to re-run under any circumstance, then emitting
/// `cargo::rerun-if-changed=build.rs` is a simple way to prevent it from being re-run (otherwise,
/// the default if no `rerun-if` instructions are emitted is to scan the entire package directory
/// for changes). Cargo automatically handles whether or not the script itself needs to be
/// recompiled, and of course the script will be re-run after it has been recompiled. Otherwise,
/// specifying build.rs is redundant and unnecessary.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
pub fn rerun_if_changed(file_paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for file_path in file_paths {
        let file_path = file_path.as_ref().display();
        write!(&CARGO_BUILD_OUT, "cargo::rerun-if-changed={file_path}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rerun-if-env-changed={env_var}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
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
pub fn rustc_link_arg<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-cdylib={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-bin={bin}={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-bins={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-tests={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-examples={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-arg-benches={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
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
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
pub fn rustc_link_lib<'a>(libs: impl IntoIterator<Item = &'a str>) {
    for lib in libs {
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-lib={lib}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
    }
}

/// Adds to the library search path
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
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
pub fn rustc_link_search(paths: impl IntoIterator<Item = impl AsRef<Path>>) {
    for path in paths {
        let path = path.as_ref().display();
        write!(&CARGO_BUILD_OUT, "cargo::rustc-link-search={path}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
    }
}

/// Passes certain flags to the compiler.
///
/// The `rustc-flags` instruction tells Cargo to pass the given space-separated flags to the compiler.
/// This only allows the `-l` and `-L` flags, and is equivalent to using [`rustc_link_lib`] and [`rustc_link_search`].
///
/// ```rust
/// cargo_build::rustc_flags(["-l ffi -l ncursesw -l c++ -l z"]);
///
/// cargo_build::rustc_flags([
///     "-l ffi",
///     "-l ncursesw",
///     "-l stdc++",
///     "-l z"
/// ]);
/// ```
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags>
pub fn rustc_flags<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        write!(&CARGO_BUILD_OUT, "cargo::rustc-flags={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
    }
}

/// Sets compile-time `cfg` settings.
///
/// To register `cfg` options see [`rustc_check_cfg`].
///
/// The `rustc-cfg` instruction tells Cargo to pass the given value to the
/// [`--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-cfg) to the compiler.
/// This may be used for compile-time detection of features to enable
/// [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html).
///
/// ### Custom cfgs must either be expected using the [`rustc_check_cfg`] instruction
/// or usage will need to allow the unexpected_cfgs lint to avoid
/// [unexpected cfgs](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unexpected-cfgs) warnings.
///
/// Note that this does not affect Cargo’s dependency resolution. This cannot be used to enable an optional
/// dependency, or enable other Cargo features.
///
/// Be aware that [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
/// use the form `feature="foo"`. `cfg` values passed with this flag are not restricted
/// to that form, and may provide just a single identifier, or any arbitrary key/value pair. For example, emitting
/// `cargo::rustc-cfg=abc` will then allow code to use `#[cfg(abc)]` (note the lack of `feature=`). Or an arbitrary
/// key/value pair may be used with an `=` symbol like `cargo::rustc-cfg=my_component="foo"`. The key should be a Rust
/// identifier, the value should be a string.
///
/// #### Example: Set `cfg` option when environment variable is present:
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg(["cuda"]);
///
/// if std::env::var("CUDA_PATH").is_ok() {
///     cargo_build::rustc_cfg(["cuda"]);
/// }
/// // main.rs
/// #[cfg(cuda)]
/// mod cuda;
/// ```
pub fn rustc_cfg<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        write!(&CARGO_BUILD_OUT, "cargo::rustc-flags={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
    }
}

/// Register custom `cfg`s as expected for compile-time checking of configs.
///
/// To set `cfg` options see [`rustc_cfg`].
///
/// Add to the list of expected config names and values that is used when checking the reachable cfg
/// expressions with the
/// [unexpected_cfgs](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unexpected-cfgs) lint.
///
/// The syntax of `CHECK_CFG` mirrors the rustc
/// [`--check-cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-check-cfg), see
/// Checking conditional configurations for more details.
///
/// #### Example: Allow custom `cfg` option, then set it when environment variable is present:
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg(["cuda"]);
///
/// if std::env::var("CUDA_PATH").is_ok() {
///     cargo_build::rustc_cfg(["cuda"]);
/// }
/// // main.rs
/// #[cfg(cuda)]
/// mod cuda;
/// ```
///
/// #### Example: Declare all `cfg` options and select 1.
/// TODO: Check if this works and HOW.
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg(["cfg(api_v1, api_v2)"]);
/// 
/// cargo_build::rustc_cfg("cfg(api_v1)");
/// 
/// // main.rs
/// #[cfg(api_v1)]
/// fn get_cities() -> Vec<String> { todo!() }
/// #[cfg(api_v2)]
/// fn get_cities() -> Vec<String> { todo!() }
/// ```
/// 
/// #### Example: Declare custom `cfg` option with .
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg(["cfg(api_v1, api_v2)"]);
/// 
/// cargo_build::rustc_cfg("cfg(api_v1)");
/// 
/// // main.rs
/// #[cfg(api_v1)]
/// fn get_cities() -> Vec<String> { todo!() }
/// #[cfg(api_v2)]
/// fn get_cities() -> Vec<String> { todo!() }
/// ```
pub fn rustc_check_cfg<'a>(flags: impl IntoIterator<Item = &'a str>) {
    for flag in flags {
        write!(&CARGO_BUILD_OUT, "cargo::rustc-flags={flag}")
            .expect("Unable to write to cargo_build::CARGO_BUILD_OUT");
    }
}
