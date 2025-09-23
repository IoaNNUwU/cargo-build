#![doc = include_str!("../README.md")]

#[cfg(feature = "macros")]
mod macros;
// pub use macros::*; no need because #[macro_export] exports them from crate root

mod functions;
pub use functions::*;

pub mod build_out;

#[cfg(test)]
mod functions_test;

#[cfg(test)]
#[cfg(feature = "macros")]
mod macros_test;
