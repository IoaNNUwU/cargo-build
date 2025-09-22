use std::io::Write;
use std::path::{Path, PathBuf};

use super::build_out::CARGO_BUILD_OUT;

const ERR_MSG: &str = "Unable to write to CARGO_BUILD_OUT";

/// Tells Cargo to re-run the build script **ONLY** if file or directory with given name changes.
///
/// The default if no `rerun-if` instructions are emitted is to scan the entire package
/// directory for changes.
///
/// #### Example: rerun build script **ONLY** if one of specified files or folders changes:
/// ```rust
/// cargo_build::rerun_if_changed(["LICENSE.md", "README.md"]);
/// cargo_build::rerun_if_changed("docs_folder");
/// cargo_build::rerun_if_changed("src/main.c");
///
/// // `String` can be used as argument
/// let platform: String = std::env::var("OS").unwrap_or("linux".to_string());
/// let config_path: String = format!("{platform}-config.toml");
///
/// cargo_build::rerun_if_changed(config_path);
/// ```
///
/// See also [`rerun_if_changed!` macro](`crate::rerun_if_changed!`) with compile-time checked formatting
/// and variable number of arguments.
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
#[allow(private_bounds)]
pub fn rerun_if_changed<I>(file_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for file_path in file_paths.into() {
        let path = file_path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT
            .with_borrow_mut(|out| writeln!(out, "cargo::rerun-if-changed={path}").expect(ERR_MSG));
    }
}

/// Tells Cargo to re-run the build script if environment variable with the given name has changed.
///
/// ```rust
/// cargo_build::rerun_if_env_changed(["LOG", "VERBOSE"]);
/// cargo_build::rerun_if_env_changed("LOG");
///
/// // `String` can be used as argument
/// let platform: String = std::env::var("OS").unwrap_or("linux".to_string());
/// let config_path_env: String = format!("{platform}_CONFIG_PATH");
///
/// cargo_build::rerun_if_env_changed(config_path_env);
/// ```
///
/// See also [`rerun_if_env_changed!` macro](`crate::rerun_if_env_changed!`) with compile-time
/// checked formatting and variable number of arguments.
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
#[allow(private_bounds)]
pub fn rerun_if_env_changed<I>(env_vars: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for env_var in env_vars.into() {
        let env_var: &str = env_var.as_ref();

        assert!(
            !env_var.contains('\n'),
            "Env var names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rerun-if-env-changed={env_var}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for benchmarks, binaries, `cdylib` crates, examples, and tests.
///
/// - To set linker flags for specific targets see [`rustc_link_arg_benches`], [`rustc_link_arg_bins`],
///   [`rustc_link_arg_cdylib`], [`rustc_link_arg_examples`], [`rustc_link_arg_tests`].
///
/// ```rust
/// cargo_build::rustc_link_arg(["-mlongcalls", "-ffunction-sections"]);
/// cargo_build::rustc_link_arg("-Wl,--cref");
///
/// let stack_size = 8 * 1024 * 1024;
///
/// if cfg!(target_os = "windows") {
///     cargo_build::rustc_link_arg([
///         "/WX",
///         "/MANIFEST:EMBED",
///         &format!("/stack:{}", stack_size)
///     ]);
/// }
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building supported targets (benchmarks,
/// binaries, cdylib crates, examples, and tests). Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
#[allow(private_bounds)]
pub fn rustc_link_arg<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg={flag}").expect(ERR_MSG);
        });
    }
}

/// Passes custom flags to a linker for `cdylib` crates.
///
/// - To set linker flags for all supported targets see [`rustc_link_arg`].
///
/// ```rust
/// cargo_build::rustc_link_arg_cdylib([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-cdylib` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building `cdylib` crates. Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg>
#[allow(private_bounds)]
pub fn rustc_link_arg_cdylib<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg-cdylib={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for specific binary name.
///
/// - To set linker flags for all bin targets see [`rustc_link_arg_bins`].
/// - To set linker flags for all supported targets see [`rustc_link_arg`].
///
/// ```rust
/// cargo_build::rustc_link_arg_bin("server", "-Wl,--cref");
///
/// cargo_build::rustc_link_arg_bin("client", [
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-bin` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building specified binary target. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-bin-link-arg>
#[allow(private_bounds)]
pub fn rustc_link_arg_bin<I>(bin: &str, linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            let flag = flag.as_ref();

            assert!(
                !bin.contains('\n'),
                "Binary names containing newlines cannot be used in the build scripts"
            );
            assert!(
                !flag.contains('\n'),
                "Compiler flags containing newlines cannot be used in the build scripts"
            );

            writeln!(out, "cargo::rustc-link-arg-bin={bin}={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for binaries.
///
/// To set linker flags for all supported targets see [`rustc_link_arg`].
/// To set linker flags for specific binary target see [`rustc_link_arg_bin`].
///
/// ```rust
/// cargo_build::rustc_link_arg_bins([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-bins` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building binary targets. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins>
#[allow(private_bounds)]
pub fn rustc_link_arg_bins<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg-bins={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for tests.
///
/// To set linker flags for all supported targets see [`rustc_link_arg`].
///
/// ```rust
/// cargo_build::rustc_link_arg_tests([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-tests` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building tests. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests>
#[allow(private_bounds)]
pub fn rustc_link_arg_tests<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg-tests={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for examples.
///
/// To set linker flags for all supported targets see [`rustc_link_arg`].
///
/// ```rust
/// cargo_build::rustc_link_arg_examples([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-examples` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building examples. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples>
#[allow(private_bounds)]
pub fn rustc_link_arg_examples<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg-examples={flag}").expect(ERR_MSG)
        });
    }
}

/// Passes custom flags to a linker for benches.
///
/// To set linker flags for all supported targets see [`rustc_link_arg`].
///
/// ```rust
/// cargo_build::rustc_link_arg_benches([
///         "-mlongcalls",
///         "-ffunction-sections",
///         "-Wl,--cref",
/// ]);
/// ```
///
/// See also [`rustc_link_arg!` macro](`crate::rustc_link_arg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// The `rustc-link-arg-benches` instruction tells Cargo to pass the
/// [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg)
/// to the compiler, but only when building benches. Its usage is highly platform
/// specific. It is useful to set the shared library version or linker script.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches>
#[allow(private_bounds)]
pub fn rustc_link_arg_benches<I>(linker_flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in linker_flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Compiler flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-arg-benches={flag}").expect(ERR_MSG)
        });
    }
}

/// Adds a library to link.
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
/// See also [`rustc_link_lib!` macro](`crate::rustc_link_lib!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
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
/// Linking modifiers (`[:MODIFIERS]`) are:
/// - `-whole-archive`(default), `+whole-archive`.
/// - `+bundle`(default), `-bundle`.
/// - `-verbatim`(default), `+verbatim`.
///
/// See more specific [`rustc_link_lib_dylib`], [`rustc_link_lib_static`], [`rustc_link_lib_framework`].
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[allow(private_bounds)]
pub fn rustc_link_lib<I>(lib_names: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for lib in lib_names.into() {
        let lib = lib.as_ref();

        assert!(
            !lib.contains('\n'),
            "Library names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT
            .with_borrow_mut(|out| writeln!(out, "cargo::rustc-link-lib={lib}").expect(ERR_MSG));
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `dylib=`.
///
/// ```rust
/// cargo_build::rustc_link_lib_dylib(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_dylib(":+whole-archive=mylib:renamed_lib");
/// ```
///
/// See also [`rustc_link_lib!` macro](`crate::rustc_link_lib!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[allow(private_bounds)]
pub fn rustc_link_lib_dylib<I>(lib_names: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for lib in lib_names.into() {
        let lib = lib.as_ref();

        assert!(
            !lib.contains('\n'),
            "Library names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-lib=dylib={lib}").expect(ERR_MSG)
        });
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `static=`.
///
/// ```rust
/// cargo_build::rustc_link_lib_static(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_static(":+whole-archive=mylib:renamed_lib");
/// ```
///
/// See also [`rustc_link_lib!` macro](`crate::rustc_link_lib!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[allow(private_bounds)]
pub fn rustc_link_lib_static<I>(lib_names: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for lib in lib_names.into() {
        let lib = lib.as_ref();

        assert!(
            !lib.contains('\n'),
            "Library names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-lib=static={lib}").expect(ERR_MSG)
        });
    }
}

/// [`rustc_link_lib`] alternative that automatically passes `framework=`.
///
/// ```rust
/// cargo_build::rustc_link_lib_framework(["nghttp2", "libssl", "libcrypto"]);
///
/// cargo_build::rustc_link_lib_framework(":+whole-archive=mylib:renamed_lib");
/// ```
///
/// See also [`rustc_link_lib!` macro](`crate::rustc_link_lib!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
#[allow(private_bounds)]
pub fn rustc_link_lib_framework<I>(lib_names: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for lib in lib_names.into() {
        let lib = lib.as_ref();

        assert!(
            !lib.contains('\n'),
            "Library names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-lib=framework={lib}").expect(ERR_MSG)
        });
    }
}

/// Adds a directory to the library search path.
///
/// ```rust
/// cargo_build::rustc_link_search("libs");
///
/// cargo_build::rustc_link_search([
///     "native=libs",
///     "framework=mac_os_libs"
/// ]);
/// ```
///
/// The `rustc-link-search` instruction tells Cargo to pass the
/// [`-L` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// to the compiler to add a directory to the library search path.
///
/// The kind of search path can optionally be specified with the form `-L KIND=PATH`.
///
/// The optional `KIND` may be one of `dependency`, `crate`, `native`, `framework`, or `all`.
/// See the [rustc book](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path)
/// for more detail.
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// See more specific [`rustc_link_search_dependency`], [`rustc_link_search_crate`], [`rustc_link_search_native`],
/// [`rustc_link_search_framework`], [`rustc_link_search_all`].
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search={}", path).expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `native=`.
///
/// ```rust
/// cargo_build::rustc_link_search_native(["libs", "vendor", "api"]);
/// ```
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search_native<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search=native={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `dependency=`.
///
/// ```rust
/// cargo_build::rustc_link_search_dependency(["libs", "vendor", "api"]);
/// ```
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search_dependency<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search=dependency={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `crate=`.
///
/// ```rust
/// cargo_build::rustc_link_search_crate(["libs", "vendor", "api"]);
/// ```
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search_crate<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search=crate={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `framework=`.
///
/// ```rust
/// cargo_build::rustc_link_search_framework(["libs", "vendor", "api"]);
/// ```
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search_framework<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search=framework={path}").expect(ERR_MSG);
        });
    }
}

/// [`rustc_link_search`] alternative that automatically passes `all=`.
///
/// ```rust
/// cargo_build::rustc_link_search_all(["libs", "vendor", "api"]);
/// ```
///
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
#[allow(private_bounds)]
pub fn rustc_link_search_all<I>(lib_paths: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    for path in lib_paths.into() {
        let path = path.as_ref();

        match path.to_str() {
            Some(path) => assert!(
                !path.contains('\n'),
                "Library paths containing newlines cannot be used in the build scripts"
            ),
            None => {}
        }
        let path = path.display();

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-link-search=all={path}").expect(ERR_MSG)
        });
    }
}

/// Passes certain flags to the compiler.
///
/// #### This only allows the `-l` and `-L` flags.
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
/// See also [`rustc_link_search!` macro](`crate::rustc_link_search!`) and
/// [`rustc_link_lib!` macro](`crate::rustc_link_lib!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags>
#[allow(private_bounds)]
pub fn rustc_flags<I>(flags: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for flag in flags.into() {
        let flag = flag.as_ref();

        assert!(
            !flag.contains('\n'),
            "Rustc flags containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-flags={flag}").expect(ERR_MSG);
        });
    }
}

/// Enables custom compile-time `cfg` settings.
///
/// #### Register all `cfg` options with [`rustc_check_cfg`] to avoid `unexpected_cfgs` warnings.
///
/// - Allows `str`/`String` as argument for zero-variant `cfg`s.
/// - Allows `(&str, &str)` as argument for `cfg`s with variants.
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs("custom_cfg");
///
/// cargo_build::rustc_cfg("custom_cfg");
///
/// // main.rs
/// #[cfg(custom_cfg)]
/// mod optional_mod;
///
/// ```
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// // Use pair (&str, &str) as argument to set `cfg` variant
/// // - Note double parenthesis (( ))
/// cargo_build::rustc_cfg(("api_version", "1"));
///
/// // main.rs
/// #[cfg(api_version="1")]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_version="2")]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
///
/// See also [`rustc_cfg!` macro](`crate::rustc_cfg!`) with improved syntax.
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
/// `cargo_build::rustc_cfg(("my_component", "foo"))` which enables `#[cfg(my_component="foo")]` code blocks.
/// The key should be a Rust identifier, the value should be a string.
///
/// See [`rustc_check_cfg`] for more information on custom `cfg`s definitions.
///
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg>
#[allow(private_bounds)]
pub fn rustc_cfg<'a>(cfg: impl Into<RustcCfg<'a>>) {
    let RustcCfg { name, value } = cfg.into();

    assert!(
        !name.contains('\n'),
        "Cfg names containing newlines cannot be used in the build scripts"
    );

    CARGO_BUILD_OUT.with_borrow_mut(|out| match value {
        None => writeln!(out, "cargo::rustc-cfg={name}").expect(ERR_MSG),
        Some(value) => {
            assert!(
                !value.contains('\n'),
                "Cfg values containing newlines cannot be used in the build scripts"
            );
            writeln!(out, "cargo::rustc-cfg={name}=\"{value}\"").expect(ERR_MSG);
        }
    });
}

/// Helper struct for [`rustc_cfg`] argument.
///
/// - Implements `From<&str>` for zero-variant `cfg`s
/// - Implements `From<(&str, &str)>` for `cfg`s with variants
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs(["custom_cfg"]);
/// cargo_build::rustc_cfg("custom_cfg");
///
/// // main.rs
/// #[cfg(custom_cfg)]
/// mod optional_mod;
///
/// // build.rs
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// // Use pair (&str, &str) as argument to set `cfg` variant
/// cargo_build::rustc_cfg(("api_version", "1"));
///
/// // main.rs
/// #[cfg(api_version="1")]
/// fn get_users() -> Vec<String> { todo!() }
/// #[cfg(api_version="2")]
/// fn get_users() -> Vec<String> { todo!() }
/// ```
struct RustcCfg<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> From<&'a str> for RustcCfg<'a> {
    fn from(name: &'a str) -> Self {
        Self { name, value: None }
    }
}

impl<'a> From<(&'a str, &'a str)> for RustcCfg<'a> {
    fn from((name, value): (&'a str, &'a str)) -> Self {
        Self {
            name,
            value: Some(value),
        }
    }
}

impl<'a> From<(&'a str,)> for RustcCfg<'a> {
    fn from(name: (&'a str,)) -> Self {
        Self {
            name: name.0,
            value: None,
        }
    }
}

/// Define expected `cfg` names and values. Those names are used when checking the *reachable* `cfg` expressions
/// with the `unexpected_cfgs` lint.
///
/// #### Note that this function only *defines* expected config names. See [`rustc_cfg`] to set `cfg` option during `build.rs` run.
///
/// - see [`rustc_check_cfgs`] to register `cfg`s options without variants.
///
/// ```rust
/// // build.rs
/// cargo_build::rustc_check_cfgs("custom_cfg");
///
/// cargo_build::rustc_cfg("custom_cfg");
///
/// // main.rs
/// #[cfg(custom_cfg)]
/// mod optional_mod;
///
/// ```
/// ```
/// // build.rs
/// cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
///
/// cargo_build::rustc_cfg(("api_version", "1"));
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
/// It is recommended to group the [`rustc_check_cfg`] and [`rustc_cfg`] functions as closely
/// as possible in order to avoid typos, missing check-cfg, stale cfgs..
///
/// See also [`rustc_check_cfg!` macro](`crate::rustc_check_cfg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
///
/// See also:
/// - [Conditional compilation example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#conditional-compilation).
/// - [Syntax of rustc `--check-cfg` flag](https://doc.rust-lang.org/rustc/command-line-arguments.html#option-check-cfg).
/// - [Checking conditional configurations](https://doc.rust-lang.org/rustc/check-cfg.html).
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-check-cfg>
#[allow(private_bounds)]
pub fn rustc_check_cfg<I>(name: &str, values: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    assert!(
        !name.contains('\n'),
        "Cfg names containing newlines cannot be used in the build scripts"
    );

    let values: String = values
        .into()
        .into_iter()
        .map(|value| {
            let value = value.as_ref();
            assert!(
                !value.contains('\n'),
                "Cfg values containing newlines cannot be used in the build scripts"
            );
            format!("\"{}\"", value)
        })
        .collect::<Vec<String>>()
        .join(", ");

    CARGO_BUILD_OUT.with_borrow_mut(|out| {
        if values.is_empty() {
            writeln!(out, "cargo::rustc-check-cfg=cfg({name})").expect(ERR_MSG);
        } else {
            writeln!(out, "cargo::rustc-check-cfg=cfg({name}, values({values}))").expect(ERR_MSG);
        }
    });
}

/// Define expected config names. Those names are used when checking the *reachable* cfg expressions
/// with the `unexpected_cfgs` lint.
///
/// This function is [`rustc_check_cfg`] alternative with multiple arguments.
///
/// ```rust
/// cargo_build::rustc_check_cfgs(["api_v1", "api_v2"]);
/// cargo_build::rustc_cfg("api_v1");
/// ```
///
/// See also [`rustc_check_cfg!` macro](`crate::rustc_check_cfg!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
#[allow(private_bounds)]
pub fn rustc_check_cfgs<I>(cfg_names: impl Into<VarArg<I>>)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for name in cfg_names.into() {
        let name = name.as_ref();

        assert!(
            !name.contains('\n'),
            "Cfg names containing newlines cannot be used in the build scripts"
        );

        CARGO_BUILD_OUT.with_borrow_mut(|out| {
            writeln!(out, "cargo::rustc-check-cfg=cfg({name})").expect(ERR_MSG);
        });
    }
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
/// See also [`rustc_env!` macro](`crate::rustc_env!`) with improved syntax.
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
pub fn rustc_env(var: &str, value: &str) {
    assert!(
        !var.contains('\n'),
        "Env variables containing newlines cannot be used in the build scripts"
    );
    assert!(
        !value.contains('\n'),
        "Env variable values containing newlines cannot be used in the build scripts"
    );

    CARGO_BUILD_OUT.with_borrow_mut(|out| {
        writeln!(out, "cargo::rustc-env={var}={value}").expect(ERR_MSG);
    });
}

/// Displays an error on the terminal.
///
/// #### This error fails the build even if all the other steps finished successfully.
///
/// ```rust
/// cargo_build::error("Fatal error during build process");
///
/// cargo_build::error("Fatal multi
/// line error
/// during build process");
/// ```
///
/// See [`error!` macro](`crate::error!`) with compile-time checked formatting.
///
/// The error instruction tells Cargo to display an error after the build script has finished running, and then fail the build.
///
/// Note: Build script libraries should carefully consider if they want to use `cargo::error` versus returning a `Result`.
/// It may be better to return a `Result`, and allow the caller to decide if the error is fatal or not. The caller can then
/// decide whether or not to display the `Err` variant using `cargo::error`.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-error>
pub fn error(msg: &str) {
    CARGO_BUILD_OUT.with_borrow_mut(|out| {
        for line in msg.lines() {
            writeln!(out, "cargo::error={line}").expect(ERR_MSG);
        }
    });
}

/// Displays a warning on the terminal.
///  
/// ```rust
/// cargo_build::warning("Warning during build process");
///
/// cargo_build::warning("Multi line
/// warning
/// during build process");
/// ```
///
/// See [`warning!` macro](`crate::warning!`) with compile-time checked formatting.
///
/// The `warning` instruction tells Cargo to display a warning after the build script has finished running. Warnings are
/// only shown for `path` dependencies (that is, those you’re working on locally), so for example warnings printed out in
/// [crates.io](https://crates.io/) crates are not emitted by default, unless the build fails. The `-vv` "very verbose"
/// flag may be used to have Cargo display warnings for all crates.
///
/// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning>
pub fn warning(msg: &str) {
    CARGO_BUILD_OUT.with_borrow_mut(|out| {
        for line in msg.lines() {
            writeln!(out, "cargo::warning={line}").expect(ERR_MSG);
        }
    });
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
/// See also [`metadata!` macro](`crate::metadata!`) with compile-time checked
/// formatting, variable number of arguments and improved syntax.
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
pub fn metadata(key: &str, value: &str) {
    assert!(
        !key.contains('\n'),
        "Metadata keys containing newlines cannot be used in the build scripts"
    );
    assert!(
        !value.contains('\n'),
        "Metadata values containing newlines cannot be used in the build scripts"
    );

    CARGO_BUILD_OUT.with_borrow_mut(|out| {
        writeln!(out, "cargo::metadata={key}={value}").expect(ERR_MSG);
    });
}

/// Helper struct for generic `one or many` iterator.
///
/// - Implements `From<&str>` for single argument.
/// - Implements `From<IntoIterator<&str>>` for multiple arguments.
///
/// This struct implements `IntoIterator<&str>` itself but there is no perfomance const
/// unlike using `Option<IntoIterator<&str>>` wrapper and matching it each time in [`Iterator::next`].
///
/// ```
/// cargo_build::rustc_link_lib("foo");
/// cargo_build::rustc_link_lib(["bar", "baz"]);
///
/// let api = std::env::var("API_LIB_NAME").unwrap_or("api".to_string());
/// cargo_build::rustc_link_lib(format!("{}", api));
/// ```
struct VarArg<I: IntoIterator>(I);

impl<'a> From<&'a str> for VarArg<std::iter::Once<&'a str>> {
    fn from(str: &'a str) -> Self {
        Self(std::iter::once(str))
    }
}

impl From<String> for VarArg<std::iter::Once<String>> {
    fn from(value: String) -> Self {
        Self(std::iter::once(value))
    }
}

impl From<PathBuf> for VarArg<std::iter::Once<PathBuf>> {
    fn from(value: PathBuf) -> Self {
        Self(std::iter::once(value))
    }
}

impl<I: IntoIterator> From<I> for VarArg<I> {
    fn from(into_iter: I) -> Self {
        Self(into_iter)
    }
}

impl<I: IntoIterator> IntoIterator for VarArg<I> {
    type Item = I::Item;
    type IntoIter = I::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
