//! ## Cargo build
//!
//! #### `cargo-build` is a wrapper around cargo instructions accesible in `build.rs`
//!
//! Add this crate as dependency
//! ```toml
//! [build-dependencies]
//! cargo-build = "0.7.2" # no macros
//! 
//! [build-dependencies]
//! cargo-build = { version = "0.7.2", features = ["macros"] }
//! ```
//! 
//! <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
//!
//! Those instructions are usually implemented by `println!("cargo::")` call. This crate
//! provides easy to use wrapper-functions around those instructions.
//! 
//! Benefits:
//! - Less code.
//! - Easier to modify later.
//! - Harder to make typos.
//!
//! #### With `cargo-build` using functions:
//! ```rust
//! cargo_build::rustc_link_arg_bin("server", "-Wl,--cref");
//! 
//! cargo_build::rustc_link_arg_bin("client", [
//!         "-mlongcalls",
//!         "-ffunction-sections",
//!         "-Wl,--cref",
//! ]);
//! ```
//! #### Without `cargo-build`:
//! ```rust
//! println!("cargo::rustc-link-arg-bin=server=-Wl,--cref");
//! println!("cargo::rustc-link-arg-bin=client=-mlongcalls");
//! println!("cargo::rustc-link-arg-bin=client=-ffunction-sections");
//! println!("cargo::rustc-link-arg-bin=client=-Wl,--cref");
//! ```
//! 
//! #### With `cargo-build` using functions:
//! ```rust
//! cargo_build::rustc_check_cfgs("cuda");
//! cargo_build::rustc_cfg("cuda");
//! 
//! cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
//! cargo_build::rustc_cfg(("api_version", "1"));
//! ```
//! #### Without `cargo-build`:
//! - Note the inconsistancy of `cfg`
//! - Note the need for escape sequences
//! ```rust
//! println!("cargo::rustc-check-cfg=cfg(cuda)");
//! println!("cargo::rustc-cfg=cuda");
//! 
//! println!("cargo::rustc-check-cfg=cfg(api_version, values(\"1\", \"2\", \"3\"))");
//! println!("cargo::rustc-cfg=api_version-\"1\"");
//! ```
//! #### Macros example (enable `features = ["macros"]` in `Cargo.toml`):
//! ```rust
//! let env_var = "HOST";
//! 
//! if std::env::var(env_var).is_ok() {
//!     cargo_build::warning!("Warning during compilation: {} is not set", env_var);
//!     cargo_build::error!("Unable to finish compilation: {} is not set", env_var);
//! }
//! 
//! cargo_build::rustc_link_arg!(cdylib: "-mlongcalls"; "-ffunction-sections");
//! 
//! cargo_build::rustc_link_arg!(
//!     bin "client":
//!       "-mlongcalls";
//!       "-ffunction-sections";
//!       "-Wl,--cref";
//!       "stack-size={}", { 8 * 1024 * 1024 };
//! );
//! 
//! cargo_build::rustc_link_lib!(
//!     static: "+whole-archive", "+verbatim", "+bundle" =
//!       "nghttp2";
//!       "libssl";
//!       "libcrypto";
//!       "mylib:{}", "renamed_lib";
//! );
//! 
//! cargo_build::rustc_check_cfg!("api_version": "1", "2", "3");
//! cargo_build::rustc_cfg!("api_version" = "1");
//! ```
//! 
//! Why use [`cargo-build`](https://crates.io/crates/cargo-build) when [`cargo emit`](https://crates.io/crates/cargo-emit) already exists:
//! - Support for modern features (such as `error`, `rustc_check_cfg`).
//! - Support for "keywords" (such as `link-lib:KIND` is not a string but defined set of values (`static`, `dylib`, `framework`)).
//! - Better syntax overall (such as `static: "lib1"; "lib2:{}", "renamed_lib2"; "lib3"` in macros - no need to declare each lib `static`).
//! - Extended examples and documentation for modern use cases.
//! - Macros are feature - library works even without macros. Enable them by using `features = ["macros"]` in `Cargo.toml`
//!
//! Note: The order of instructions in the build script may affect the order of arguments that
//! cargo passes to rustc. In turn, the order of arguments passed to rustc may affect the
//! order of arguments passed to the linker. Therefore, you will want to pay attention to
//! the order of the build script’s instructions. For example, if object `foo` needs to link
//! against library `bar`, you may want to make sure that library `bar`'s `rustc-link-lib`
//! instruction appears after instructions to link object `foo`.\
//! 
//! <https://doc.rust-lang.org/cargo/reference/build-scripts.html>

#[cfg(feature="macros")]
mod macros;
// pub use macros::*; no need because #[macro_export] exports them from crate root

mod functions;
pub use functions::*;

pub mod build_out;

#[cfg(test)]
mod functions_test;

#[cfg(test)]
#[cfg(feature="macros")]
mod macros_test;