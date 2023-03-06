// cuadra
//
//! Layouts.
//
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

mod clamper;
mod position;
mod size;
mod zone;

pub use clamper::{Clamper16, Clamper32, Clamper64, Clamper8};

pub use position::{Position16, Position32, Position64, Position8};
pub use size::{Size16, Size32, Size64, Size8};
pub use zone::{Zone16, Zone32, Zone64, Zone8};
