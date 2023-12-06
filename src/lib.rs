//! This crate implements index types on top of [`num::NonZero*`](https://doc.rust-lang.org/core/num/index.html) and friends.
//! An `Option<Index*>` has the same memory size as `*`, thanks to the "[null pointer optimization](https://doc.rust-lang.org/std/option/#representation)".

#![no_std]

#[macro_use]
mod internal_macros;

mod non_max;

mod index;
pub use index::*;
