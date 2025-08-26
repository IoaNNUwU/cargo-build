//! ## Cargo build
//!
//! ##### `cargo-build` is a wrapper around cargo instructions accesible in `build.rs`
//!
//! <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
//!
//! Those instructions are usually implemented by `println!("cargo::")` call. This crate
//! provides easy-to-use wrapper-functions around those instructions.
//! 
//! #### Main benefit that it is harder to make typos with cargo commands and there is no need to repeat `println!("cargo::")` for each command.
//!
//! #### With `cargo-build`:
//! ```rust
//! cargo_build::rustc_link_arg_bin("server", ["-Wl", "--cref"]);
//! cargo_build::rustc_link_arg_bin("client", [
//!         "-mlongcalls",
//!         "-ffunction-sections",
//!         "-Wl,--cref",
//! ]);
//! ```
//! #### Without `cargo-build`:
//! ```rust
//! println!("cargo::rustc-link-arg-bin=server=-Wl");
//! println!("cargo::rustc-link-arg-bin=server=--cref");
//! println!("cargo::rustc-link-arg-bin=client=-mlongcalls");
//! println!("cargo::rustc-link-arg-bin=client=-ffunction-sections");
//! println!("cargo::rustc-link-arg-bin=client=-Wl,--cref");
//! ```
//! 
//! #### With `cargo-build`:
//! ```rust
//! cargo_build::rustc_check_cfg("cuda", []);
//! cargo_build::rustc_cfg("cuda");
//! 
//! use cargo_build::StrExtCfg;
//! 
//! cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);
//! cargo_build::rustc_cfg("api_version".value("1"));
//! ```
//! #### Without `cargo-build`:
//! ```rust
//! // Note the inconstancy of `cfg`.
//! println!("cargo::rustc-check-cfg=cfg(cuda)");
//! println!("cargo::rustc-cfg=cuda");
//! // Note the need for escape sequences
//! println!("cargo::rustc-check-cfg=cfg(api_version, values(\"1\", \"2\", \"3\"))");
//! println!("cargo::rustc-cfg=api_version-\"1\"");
//! ```
//!
//! Note: The order of instructions in the build script may affect the order of arguments that
//! cargo passes to rustc. In turn, the order of arguments passed to rustc may affect the
//! order of arguments passed to the linker. Therefore, you will want to pay attention to
//! the order of the build script’s instructions. For example, if object `foo` needs to link
//! against library `bar`, you may want to make sure that library `bar`'s `rustc-link-lib`
//! instruction appears after instructions to link object `foo`.
//!
//! #### Why functions and not macros?
//!
//! - Faster compile times and easier code

#[cfg(feature="macros")]
mod macros;
// pub use macros::*; is not needed because #[macro_export] exports them from crate root

mod functions;
pub use functions::*;

pub mod cargo_build_out;

#[cfg(test)]
mod functions_test;

#[cfg(test)]
mod macros_test;