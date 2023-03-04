// cuadra
//
//! Layouts.
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

pub(crate) use clamper::Clamper;
pub use position::Position;
pub use size::Size;
pub use zone::Zone;
