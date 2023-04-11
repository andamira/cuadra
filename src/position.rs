// cuadra::position
//
//!
//

use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

macro_rules! position {
    // $i: inner primitive
    // $b: bit size
    ( $($i:ty, $b:expr),+ ) => {
        $( position![single: $i, $b]; )+
    };

    (single: $i:ty, $b:literal) => { paste::paste! {
        use super::[<Clamper$b>] as [<C$b>];

        #[doc = "A 2D position using a clamped [`" $i "`]."]
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<Position$b>] {
            x: $i,
            y: $i,
        }

        impl fmt::Debug for [<Position$b>] {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {{ x: {}, y: {} }}", stringify!([<Position$b>]), self.x, self.y,)
            }
        }

        impl fmt::Display for [<Position$b>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "x:{} y:{}", self.x, self.y)
            }
        }

        impl [<Position$b>] {
            /// Defines a new `Position` with the given dimensions.
            pub const fn new(x: $i, y: $i) -> Self {
                Self {
                    x: [<C$b>]::clamp(x),
                    y: [<C$b>]::clamp(y),
                }
            }

            /// Get x.
            #[inline]
            pub const fn x(&self) -> $i {
                self.x
            }
            /// Get y.
            #[inline]
            pub const fn y(&self) -> $i {
                self.y
            }

            /// Set x.
            #[inline]
            pub fn set_x(&mut self, x: $i) {
                self.x = [<C$b>]::clamp(x);
            }
            /// Set y.
            #[inline]
            pub fn set_y(&mut self, y: $i) {
                self.y = [<C$b>]::clamp(y);
            }

            /// Swaps the `x, y` values.
            pub fn swap(&mut self) {
                core::mem::swap(&mut self.y, &mut self.x)
            }

            #[doc = "Returns a new `Position"$b  "` with the `x, y` values swapped."]
            pub const fn swapped(&self) -> [<Position$b>] {
                Self::new(self.y, self.x)
            }
        }

        /* conversions */

        /// # conversions
        impl [<Position$b>] {
            pub const fn as_tuple(&self) -> ($i, $i) {
                (self.x, self.y)
            }
            pub const fn from_tuple(tup: ($i, $i)) -> [<Position$b>] {
                Self::new(tup.0, tup.1)
            }

            pub const fn as_tuple_i32(&self) -> (i32, i32) {
                (
                    [<C$b>]::clamp_to_i32(self.x),
                    [<C$b>]::clamp_to_i32(self.y),
                )
            }
            pub const fn from_tuple_i32(tup: (i32, i32)) -> [<Position$b>] {
                Self::new(
                    [<C$b>]::clamp_from_i32(tup.0),
                    [<C$b>]::clamp_from_i32(tup.1),
                )
            }

            pub const fn as_tuple_u32(&self) -> (u32, u32) {
                (
                    [<C$b>]::clamp_to_u32(self.x),
                    [<C$b>]::clamp_to_u32(self.y),
                )
            }
            pub const fn from_tuple_u32(tup: (u32, u32)) -> [<Position$b>] {
                Self::new(
                    [<C$b>]::clamp_from_u32(tup.0),
                    [<C$b>]::clamp_from_u32(tup.1),
                )
            }

            pub const fn as_tuple_u16(&self) -> (u16, u16) {
                (
                    [<C$b>]::clamp_to_u16(self.x),
                    [<C$b>]::clamp_to_u16(self.y),
                )
            }
            pub const fn from_tuple_u16(tup: (u16, u16)) -> [<Position$b>] {
                Self::new(
                    [<C$b>]::clamp_from_u16(tup.0),
                    [<C$b>]::clamp_from_u16(tup.1),
                )
            }

            pub const fn as_tuple_i16(&self) -> (i16, i16) {
                (
                    [<C$b>]::clamp_to_i16(self.x),
                    [<C$b>]::clamp_to_i16(self.y),
                )
            }
            pub const fn from_tuple_i16(tup: (i16, i16)) -> [<Position$b>] {
                Self::new(
                    [<C$b>]::clamp_from_i16(tup.0),
                    [<C$b>]::clamp_from_i16(tup.1),
                )
            }

            pub const fn as_tuple_usize(&self) -> (usize, usize) {
                (
                    [<C$b>]::clamp_to_usize(self.x),
                    [<C$b>]::clamp_to_usize(self.y),
                )
            }
            pub const fn from_tuple_usize(tup: (usize, usize)) -> [<Position$b>] {
                Self::new(
                    [<C$b>]::clamp_from_usize(tup.0),
                    [<C$b>]::clamp_from_usize(tup.1),
                )
            }
        }

        impl From<(i16, i16)> for [<Position$b>] {
            fn from(tup: (i16, i16)) -> [<Position$b>] {
                Self::from_tuple_i16(tup)
            }
        }
        impl From<[<Position$b>]> for (i16, i16) {
            fn from(s: [<Position$b>]) -> (i16, i16) {
                s.as_tuple_i16()
            }
        }

        impl From<(u16, u16)> for [<Position$b>] {
            fn from(tup: (u16, u16)) -> [<Position$b>] {
                Self::from_tuple_u16(tup)
            }
        }
        impl From<[<Position$b>]> for (u16, u16) {
            fn from(s: [<Position$b>]) -> (u16, u16) {
                s.as_tuple_u16()
            }
        }

        impl From<(i32, i32)> for [<Position$b>] {
            fn from(tup: (i32, i32)) -> [<Position$b>] {
                Self::from_tuple_i32(tup)
            }
        }
        impl From<[<Position$b>]> for (i32, i32) {
            fn from(p: [<Position$b>]) -> (i32, i32) {
                p.as_tuple_i32()
            }
        }

        impl From<(u32, u32)> for [<Position$b>] {
            fn from(tup: (u32, u32)) -> [<Position$b>] {
                Self::from_tuple_u32(tup)
            }
        }
        impl From<[<Position$b>]> for (u32, u32) {
            fn from(p: [<Position$b>]) -> (u32, u32) {
                p.as_tuple_u32()
            }
        }

        impl From<(usize, usize)> for [<Position$b>] {
            fn from(tup: (usize, usize)) -> [<Position$b>] {
                Self::from_tuple_usize(tup)
            }
        }
        impl From<[<Position$b>]> for (usize, usize) {
            fn from(p: [<Position$b>]) -> (usize, usize) {
                p.as_tuple_usize()
            }
        }

        /* operations with another position */

        /// # arithmetic ops.
        impl [<Position$b>] {
            #[doc = "Saturating, clamped addition of two `" [<Position$b>] "`."]
            pub const fn saturating_add(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_add(rhs.x()),
                    self.y().saturating_add(rhs.y()),
                    )
            }
            #[doc = "Wrapping, clamped addition of two `" [<Position$b>] "`."]
            pub const fn wrapping_add(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_add(rhs.x()),
                    self.y().wrapping_add(rhs.y()),
                    )
            }
            #[doc = "Checked, clamped addition of two `" [<Position$b>] "`."]
            pub fn checked_add(&self, rhs: [<Position$b>]) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_add(rhs.x())?,
                    self.y().checked_add(rhs.y())?,
                    ))
            }

            #[doc = "Saturating, clamped addition of two `" [<Position$b>] "`."]
            pub const fn saturating_sub(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_sub(rhs.x()),
                    self.y().saturating_sub(rhs.y()),
                    )
            }
            #[doc = "Wrapping, clamped addition of two `" [<Position$b>] "`."]
            pub const fn wrapping_sub(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_sub(rhs.x()),
                    self.y().wrapping_sub(rhs.y()),
                    )
            }
            #[doc = "Checked, clamped addition of two `" [<Position$b>] "`."]
            pub fn checked_sub(&self, rhs: [<Position$b>]) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_sub(rhs.x())?,
                    self.y().checked_sub(rhs.y())?,
                    ))
            }

            #[doc = "Saturating, clamped multiplication of two `" [<Position$b>] "`."]
            pub const fn saturating_mul(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_mul(rhs.x()),
                    self.y().saturating_mul(rhs.y()),
                    )
            }
            #[doc = "Wrapping, clamped multiplication of two `" [<Position$b>] "`."]
            pub const fn wrapping_mul(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_mul(rhs.x()),
                    self.y().wrapping_mul(rhs.y()),
                    )
            }
            #[doc = "Checked, clamped multiplication of two `" [<Position$b>] "`."]
            pub fn checked_mul(&self, rhs: [<Position$b>]) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_mul(rhs.x())?,
                    self.y().checked_mul(rhs.y())?,
                    ))
            }

            #[doc = "Saturating, clamped division of two `" [<Position$b>] "`."]
            pub const fn saturating_div(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_div(rhs.x()),
                    self.y().saturating_div(rhs.y()),
                    )
            }
            #[doc = "Wrapping, clamped division of two `" [<Position$b>] "`."]
            pub const fn wrapping_div(&self, rhs: [<Position$b>]) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_div(rhs.x()),
                    self.y().wrapping_div(rhs.y()),
                    )
            }
            #[doc = "Checked, clamped division of two `" [<Position$b>] "`."]
            pub fn checked_div(&self, rhs: [<Position$b>]) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_div(rhs.x())?,
                    self.y().checked_div(rhs.y())?,
                    ))
            }

            /* operations with a primitive value */

            #[doc = "Saturating, clamped addition of a `" [<Position$b>] " with a `value``."]
            pub const fn saturating_add_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_add(value),
                    self.y().saturating_add(value),
                    )
            }
            #[doc = "Wrapping, clamped addition of a `" [<Position$b>] " with a `value``."]
            pub const fn wrapping_add_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_add(value),
                    self.y().wrapping_add(value),
                    )
            }
            #[doc = "Checked, clamped addition of a `" [<Position$b>] " with a `value``."]
            pub fn checked_add_value(&self, value: $i) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_add(value)?,
                    self.y().checked_add(value)?,
                    ))
            }

            #[doc = "Saturating, clamped substraction of a `" [<Position$b>] " with a `value``."]
            pub const fn saturating_sub_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_sub(value),
                    self.y().saturating_sub(value),
                    )
            }
            #[doc = "Wrapping, clamped substraction of a `" [<Position$b>] " with a `value``."]
            pub const fn wrapping_sub_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_sub(value),
                    self.y().wrapping_sub(value),
                    )
            }
            #[doc = "Checked, clamped substraction of a `" [<Position$b>] " with a `value``."]
            pub fn checked_sub_value(&self, value: $i) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_sub(value)?,
                    self.y().checked_sub(value)?,
                    ))
            }

            #[doc = "Saturating, clamped multiplication of a `" [<Position$b>] " with a `value``."]
            pub const fn saturating_mul_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_mul(value),
                    self.y().saturating_mul(value),
                    )
            }
            #[doc = "Wrapping, clamped multiplication of a `" [<Position$b>] " with a `value``."]
            pub const fn wrapping_mul_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_mul(value),
                    self.y().wrapping_mul(value),
                    )
            }
            #[doc = "Checked, clamped multiplication of a `" [<Position$b>] " with a `value``."]
            pub fn checked_mul_value(&self, value: $i) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_mul(value)?,
                    self.y().checked_mul(value)?,
                    ))
            }

            #[doc = "Saturating, clamped division of a `" [<Position$b>] " with a `value``."]
            pub const fn saturating_div_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().saturating_div(value),
                    self.y().saturating_div(value),
                    )
            }
            #[doc = "Wrapping, clamped division of a `" [<Position$b>] " with a `value``."]
            pub const fn wrapping_div_value(&self, value: $i) -> [<Position$b>] {
                Self::new(
                    self.x().wrapping_div(value),
                    self.y().wrapping_div(value),
                    )
            }
            #[doc = "Checked, clamped division of a `" [<Position$b>] " with a `value``."]
            pub fn checked_div_value(&self, value: $i) -> Option<[<Position$b>]> {
                Some(Self::new(
                    self.x().checked_div(value)?,
                    self.y().checked_div(value)?,
                    ))
            }
        }

        /* impl ops */

        impl Add for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }
        }
        impl Sub for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }
        }
        impl Mul for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped multiplication.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.saturating_mul(rhs)
            }
        }
        impl Div for [<Position$b>] {
            type Output = Self;

            #[inline]
            /// Saturating, clamped division.
            fn div(self, rhs: Self) -> Self {
                self.saturating_div(rhs)
            }
        }

        impl Add<$i> for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn add(self, rhs: $i) -> Self {
                self.saturating_add_value(rhs)
            }
        }
        impl Sub<$i> for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn sub(self, rhs: $i) -> Self {
                self.saturating_sub_value(rhs)
            }
        }
        impl Mul<$i> for [<Position$b>] {
            type Output = Self;

            /// Saturating, clamped multiplication.
            #[inline]
            fn mul(self, rhs: $i) -> Self {
                self.saturating_mul_value(rhs)
            }
        }
        impl Div<$i> for [<Position$b>] {
            type Output = Self;

            #[inline]
            /// Saturating, clamped division.
            fn div(self, rhs: $i) -> Self {
                self.saturating_div_value(rhs)
            }
        }
    }};
}
position![i8, 8, i16, 16, i32, 32, i64, 64];
