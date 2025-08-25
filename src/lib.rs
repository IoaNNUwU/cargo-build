//! ## Cargo build
//!
//! ##### `cargo-build` is a wrapper around cargo instructions accesible in `build.rs`
//!
//! <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
//!
//! Those instructions are usually implemented by `println!("cargo::")` call. This crate
//! provides easy-to-use wrapper-functions around those instructions.
//!
//! ```rust
//! // With cargo-build
//! cargo_build::rustc_link_arg([
//!     "-mlongcalls",
//!     "-ffunction-sections",
//!     "-Wl,--cref"
//! ]);
//! // Without cargo-build
//! println!("cargo::rustc-link-arg=-mlongcalls");
//! println!("cargo::rustc-link-arg=-ffunction-sections");
//! println!("cargo::rustc-link-arg=-Wl,--cref");
//! ```
//!
//! `warning!` & `error!` macros simplify debugging:
//! - Those macros (and corresponding `println!("cargo::")` instructions) are the only way to print to stdout during `build.rs` execution.
//!
//! ```rust
//! let args: Vec<String> = std::env::args().collect();
//! cargo_build::warning!("Args = {:?}", args);
//!
//! #[cfg(target_os="macos")]
//! cargo_build::error!("Fatal error: {}", "Unsupported platform");
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
//!
//! Most functions in this crate take `IntoIterator<Item = &str>` as argument, except for
//! `warning` and `error` macros, which have the same signature as `println!` macro to support
//! compile-time checks for arguments count.

#[cfg(feature = "macros")]
mod macros;
// pub use macros::*; is not needed because #[macro_export] exports them from crate root

#[cfg(feature = "functions")]
mod functions;
#[cfg(feature = "functions")]
pub use functions::*;

mod cargo_build_out;
pub use cargo_build_out::{CargoBuildOut, CARGO_BUILD_OUT};

#[cfg(test)]
mod test;
