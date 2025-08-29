//! ## Cargo build
//!
//! #### `cargo-build` is a wrapper around cargo instructions accesible in `build.rs`
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
//! #### With `cargo-build`:
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
//! #### With `cargo-build`:
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
//! #### With `cargo-build`:
//! ```rust
//! cargo_build::warning("Warning during compilation");
//! cargo_build::error("Fatal error during compilation");
//! 
//! let env_var = "HOST";
//! 
//! if std::env::var(env_var).is_ok() {
//!     cargo_build::warning!("Warning during compilation: {} is not set", env_var);
//!     cargo_build::error!("Unable to finish compilation: {} is not set", env_var);
//! }
//! ```
//! #### Without `cargo-build`:
//! ```rust
//! println!("cargo::warning=Warning during compilation");
//! println!("cargo::error=Fatal error during compilation");
//! 
//! let env_var = "HOST";
//! 
//! if std::env::var(env_var).is_ok() {
//!     println!("cargo::warning=Warning during compilation: {} is not set", env_var);
//!     println!("cargo::error=Unable to finish compilation: {} is not set", env_var);
//! 
//!     // or with custom function. Note the need of `format!` on call site.
//!     fn my_error(err: &str) { println!("cargo::error={}", err); }
//!     my_error(&format!("Warning during compilation: {} is not set", env_var));
//! }
//! ```
//!
//! Note: The order of instructions in the build script may affect the order of arguments that
//! cargo passes to rustc. In turn, the order of arguments passed to rustc may affect the
//! order of arguments passed to the linker. Therefore, you will want to pay attention to
//! the order of the build script’s instructions. For example, if object `foo` needs to link
//! against library `bar`, you may want to make sure that library `bar`'s `rustc-link-lib`
//! instruction appears after instructions to link object `foo`.

#[cfg(feature="macros")]
mod macros;
// pub use macros::*; is not needed because #[macro_export] exports them from crate root

mod functions;
pub use functions::*;

pub mod build_out;

#[cfg(test)]
mod functions_test;

#[cfg(test)]
#[cfg(feature="macros")]
mod macros_test;