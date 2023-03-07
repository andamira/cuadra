// cuadra
//
//! ## Layout
//!
//! Several primitives are defined for `Position`, `Size` and their combination
//! as a `Zone`.
//!
//! Types have 8, 16, 32 and 64 bit sizes implementations, and they are limited
//! to values of just half the range of an equivalent bit-size signed integer,
//! leaving 1/4th of the range for positive values.
//!
//! Positions can have negative values, while Sizes can only be positive.
//!
//! For example, a [`Position8`] can hold values between `-64,-64` and `63,63`,
//! while a [`Size16`] can have values of between `1,1` and `16_383,16_383`.
//!
//! The non-negative range from 0 to the clamped maximum is always 1/4th of the
//! primitive's maximum.
//! This allows to have a safety margin around which allows to move the biggest
//! size to the furthest position, and still be able to represent all its points
//! with the same inner bit-size.
//!
//! The following diagram shows the example range of values of a [`Position8`],
//! around the inner centered square, with the outer safety margin around it,
//! and the maximum positive clamped size filled with `+`.
//! ```diagram
//!       -128,127       0,127     127,127
//!           .-----|-----|-----|-----·
//!           |                       |
//!           |  -64,63       63,63   |
//!           |     .-----------.     --
//!           |     |     |+++++|     |
//!           |     |     |+++++|     |
//!   -128,0 --     |----0,0----|     -- 127,0
//!           |     |     |     |     |
//!           |     |     |     |     |
//!           |     .-----------.     --
//!           |  -64,-64      63,-64  |
//!           |                       |
//!           .-----|-----|-----|-----.
//!       -128,-128      0,-128    127,-128
//! ```
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests;

mod clamper;
mod position;
mod size;
mod zone;

pub use clamper::{Clamper16, Clamper32, Clamper64, Clamper8};
pub use position::{Position16, Position32, Position64, Position8};
pub use size::{Size16, Size32, Size64, Size8};
pub use zone::{Zone16, Zone32, Zone64, Zone8};
