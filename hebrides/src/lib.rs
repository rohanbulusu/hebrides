//! A general mathematics library.
//! 
//! `Real` and `Complex` are Rust implementations of their mathematical
//! counterparts, and `Vector` and `Matrix` form the basis of the crate's
//! linear algebra systems.

#![deny(rust_2018_idioms, missing_docs)]

pub mod elem;
pub mod linal;

pub use elem::*;
pub use linal::*;

