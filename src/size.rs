// cuadra::size
//
//!
//

use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

macro_rules! size {
    // $i: inner primitive
    // $b: bit size
    ( $($i:ty, $b:expr),+ ) => {
        $( size![single: $i, $b]; )+
    };

    (single: $i:ty, $b:literal) => { paste::paste! {
        use super::[<Clamper$b>] as [<C$b>];

        #[doc = "A 2D size using a positive clamped [`" $i "`]."]
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<Size$b>] {
            w: $i,
            h: $i,
        }

        impl fmt::Debug for [<Size$b>] {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {{ w: {}, h: {} }}", stringify!([<Size$b>]), self.w, self.h,)
            }
        }

        impl fmt::Display for [<Size$b>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "w:{} h:{}", self.w, self.h)
            }
        }

        impl [<Size$b>] {
            /// Defines a new `Size` with the given dimensions,
            /// which has to be at least `1`.
            pub const fn new(width: $i, height: $i) -> Self {
                Self {
                    w: [<C$b>]::clamp_positive(width),
                    h: [<C$b>]::clamp_positive(height),
                }
            }

            /// Get the width.
            #[inline]
            pub const fn w(&self) -> $i {
                self.w
            }
            /// Get the height.
            #[inline]
            pub const fn h(&self) -> $i {
                self.h
            }

            /// Set the width.
            #[inline]
            pub fn set_w(&mut self, width: $i) {
                self.w = [<C$b>]::clamp_positive(width);
            }
            /// Set the height.
            #[inline]
            pub fn set_h(&mut self, height: $i) {
                self.h = [<C$b>]::clamp_positive(height);
            }

            /// Swaps the `w, h` values.
            pub fn swap(&mut self) {
                core::mem::swap(&mut self.h, &mut self.w)
            }

            #[doc = "Returns a new `Size"$b  "` with the `w, h` values swapped."]
            pub const fn swapped(&self) -> [<Size$b>] {
                Self::new(self.h, self.w)
            }
        }

        /* conversions */

        /// # conversions
        impl [<Size$b>] {
            pub const fn as_tuple(&self) -> ($i, $i) {
                (self.w, self.h)
            }
            pub const fn from_tuple(tup: ($i, $i)) -> [<Size$b>] {
                Self::new(tup.0, tup.1)
            }

            pub const fn as_tuple_i32(&self) -> (i32, i32) {
                (
                    [<C$b>]::clamp_positive_to_i32(self.w),
                    [<C$b>]::clamp_positive_to_i32(self.h),
                )
            }
            pub const fn from_tuple_i32(tup: (i32, i32)) -> [<Size$b>] {
                Self::new(
                    [<C$b>]::clamp_positive_from_i32(tup.0),
                    [<C$b>]::clamp_positive_from_i32(tup.1),
                )
            }

            pub const fn as_tuple_u32(&self) -> (u32, u32) {
                (
                    [<C$b>]::clamp_positive_to_u32(self.w),
                    [<C$b>]::clamp_positive_to_u32(self.h),
                )
            }
            pub const fn from_tuple_u32(tup: (u32, u32)) -> [<Size$b>] {
                Self::new(
                    [<C$b>]::clamp_positive_from_u32(tup.0),
                    [<C$b>]::clamp_positive_from_u32(tup.1),
                )
            }

            pub const fn as_tuple_u16(&self) -> (u16, u16) {
                (
                    [<C$b>]::clamp_positive_to_u16(self.w),
                    [<C$b>]::clamp_positive_to_u16(self.h),
                )
            }
            pub const fn from_tuple_u16(tup: (u16, u16)) -> [<Size$b>] {
                Self::new(
                    [<C$b>]::clamp_positive_from_u16(tup.0),
                    [<C$b>]::clamp_positive_from_u16(tup.1),
                )
            }

            pub const fn as_tuple_i16(&self) -> (i16, i16) {
                (
                    [<C$b>]::clamp_positive_to_i16(self.w),
                    [<C$b>]::clamp_positive_to_i16(self.h),
                )
            }
            pub const fn from_tuple_i16(tup: (i16, i16)) -> [<Size$b>] {
                Self::new(
                    [<C$b>]::clamp_positive_from_i16(tup.0),
                    [<C$b>]::clamp_positive_from_i16(tup.1),
                )
            }

            pub const fn as_tuple_usize(&self) -> (usize, usize) {
                (
                    [<C$b>]::clamp_positive_to_usize(self.w),
                    [<C$b>]::clamp_positive_to_usize(self.h),
                )
            }
            pub const fn from_tuple_usize(tup: (usize, usize)) -> [<Size$b>] {
                Self::new(
                    [<C$b>]::clamp_positive_from_usize(tup.0),
                    [<C$b>]::clamp_positive_from_usize(tup.1),
                )
            }
        }

        impl From<(i16, i16)> for [<Size$b>] {
            fn from(tup: (i16, i16)) -> [<Size$b>] {
                Self::from_tuple_i16(tup)
            }
        }
        impl From<[<Size$b>]> for (i16, i16) {
            fn from(s: [<Size$b>]) -> (i16, i16) {
                s.as_tuple_i16()
            }
        }

        impl From<(u16, u16)> for [<Size$b>] {
            fn from(tup: (u16, u16)) -> [<Size$b>] {
                Self::from_tuple_u16(tup)
            }
        }
        impl From<[<Size$b>]> for (u16, u16) {
            fn from(s: [<Size$b>]) -> (u16, u16) {
                s.as_tuple_u16()
            }
        }

        impl From<(i32, i32)> for [<Size$b>] {
            fn from(tup: (i32, i32)) -> [<Size$b>] {
                Self::from_tuple_i32(tup)
            }
        }
        impl From<[<Size$b>]> for (i32, i32) {
            fn from(s: [<Size$b>]) -> (i32, i32) {
                s.as_tuple_i32()
            }
        }

        impl From<(u32, u32)> for [<Size$b>] {
            fn from(tup: (u32, u32)) -> [<Size$b>] {
                Self::from_tuple_u32(tup)
            }
        }
        impl From<[<Size$b>]> for (u32, u32) {
            fn from(s: [<Size$b>]) -> (u32, u32) {
                s.as_tuple_u32()
            }
        }

        impl From<(usize, usize)> for [<Size$b>] {
            fn from(tup: (usize, usize)) -> [<Size$b>] {
                Self::from_tuple_usize(tup)
            }
        }
        impl From<[<Size$b>]> for (usize, usize) {
            fn from(s: [<Size$b>]) -> (usize, usize) {
                s.as_tuple_usize()
            }
        }

        /* operations with another size */

        /// # arithmetic ops.
        impl [<Size$b>] {
            #[doc = "Saturating, clamped addition of two `" [<Size$b>] "`."]
            pub const fn saturating_add(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_add(rhs.w()),
                    self.h().saturating_add(rhs.h()),
                    )
            }
            #[doc = "Wrapping, clamped addition of two `" [<Size$b>] "`."]
            pub const fn wrapping_add(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_add(rhs.w()),
                    self.h().wrapping_add(rhs.h()),
                    )
            }
            #[doc = "Checked, clamped addition of two `" [<Size$b>] "`."]
            pub fn checked_add(&self, rhs: [<Size$b>]) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_add(rhs.w())?,
                    self.h().checked_add(rhs.h())?,
                    ))
            }

            #[doc = "Saturating, clamped addition of two `" [<Size$b>] "`."]
            pub const fn saturating_sub(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_sub(rhs.w()),
                    self.h().saturating_sub(rhs.h()),
                    )
            }
            #[doc = "Wrapping, clamped addition of two `" [<Size$b>] "`."]
            pub const fn wrapping_sub(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_sub(rhs.w()),
                    self.h().wrapping_sub(rhs.h()),
                    )
            }
            #[doc = "Checked, clamped addition of two `" [<Size$b>] "`."]
            pub fn checked_sub(&self, rhs: [<Size$b>]) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_sub(rhs.w())?,
                    self.h().checked_sub(rhs.h())?,
                    ))
            }

            #[doc = "Saturating, clamped multiplication of two `" [<Size$b>] "`."]
            pub const fn saturating_mul(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_mul(rhs.w()),
                    self.h().saturating_mul(rhs.h()),
                    )
            }
            #[doc = "Wrapping, clamped multiplication of two `" [<Size$b>] "`."]
            pub const fn wrapping_mul(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_mul(rhs.w()),
                    self.h().wrapping_mul(rhs.h()),
                    )
            }
            #[doc = "Checked, clamped multiplication of two `" [<Size$b>] "`."]
            pub fn checked_mul(&self, rhs: [<Size$b>]) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_mul(rhs.w())?,
                    self.h().checked_mul(rhs.h())?,
                    ))
            }

            #[doc = "Saturating, clamped division of two `" [<Size$b>] "`."]
            pub const fn saturating_div(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_div(rhs.w()),
                    self.h().saturating_div(rhs.h()),
                    )
            }
            #[doc = "Wrapping, clamped division of two `" [<Size$b>] "`."]
            pub const fn wrapping_div(&self, rhs: [<Size$b>]) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_div(rhs.w()),
                    self.h().wrapping_div(rhs.h()),
                    )
            }
            #[doc = "Checked, clamped division of two `" [<Size$b>] "`."]
            pub fn checked_div(&self, rhs: [<Size$b>]) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_div(rhs.w())?,
                    self.h().checked_div(rhs.h())?,
                    ))
            }

            /* operations with a primitive value */

            #[doc = "Saturating, clamped addition of a `" [<Size$b>] " with a `value``."]
            pub const fn saturating_add_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_add(value),
                    self.h().saturating_add(value),
                    )
            }
            #[doc = "Wrapping, clamped addition of a `" [<Size$b>] " with a `value``."]
            pub const fn wrapping_add_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_add(value),
                    self.h().wrapping_add(value),
                    )
            }
            #[doc = "Checked, clamped addition of a `" [<Size$b>] " with a `value``."]
            pub fn checked_add_value(&self, value: $i) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_add(value)?,
                    self.h().checked_add(value)?,
                    ))
            }

            #[doc = "Saturating, clamped substraction of a `" [<Size$b>] " with a `value``."]
            pub const fn saturating_sub_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_sub(value),
                    self.h().saturating_sub(value),
                    )
            }
            #[doc = "Wrapping, clamped substraction of a `" [<Size$b>] " with a `value``."]
            pub const fn wrapping_sub_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_sub(value),
                    self.h().wrapping_sub(value),
                    )
            }
            #[doc = "Checked, clamped substraction of a `" [<Size$b>] " with a `value``."]
            pub fn checked_sub_value(&self, value: $i) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_sub(value)?,
                    self.h().checked_sub(value)?,
                    ))
            }

            #[doc = "Saturating, clamped multiplication of a `" [<Size$b>] " with a `value``."]
            pub const fn saturating_mul_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_mul(value),
                    self.h().saturating_mul(value),
                    )
            }
            #[doc = "Wrapping, clamped multiplication of a `" [<Size$b>] " with a `value``."]
            pub const fn wrapping_mul_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_mul(value),
                    self.h().wrapping_mul(value),
                    )
            }
            #[doc = "Checked, clamped multiplication of a `" [<Size$b>] " with a `value``."]
            pub fn checked_mul_value(&self, value: $i) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_mul(value)?,
                    self.h().checked_mul(value)?,
                    ))
            }

            #[doc = "Saturating, clamped division of a `" [<Size$b>] " with a `value``."]
            pub const fn saturating_div_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().saturating_div(value),
                    self.h().saturating_div(value),
                    )
            }
            #[doc = "Wrapping, clamped division of a `" [<Size$b>] " with a `value``."]
            pub const fn wrapping_div_value(&self, value: $i) -> [<Size$b>] {
                Self::new(
                    self.w().wrapping_div(value),
                    self.h().wrapping_div(value),
                    )
            }
            #[doc = "Checked, clamped division of a `" [<Size$b>] " with a `value``."]
            pub fn checked_div_value(&self, value: $i) -> Option<[<Size$b>]> {
                Some(Self::new(
                    self.w().checked_div(value)?,
                    self.h().checked_div(value)?,
                    ))
            }
        }

        /* impl ops */

        impl Add for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }
        }
        impl Sub for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }
        }
        impl Mul for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped multiplication.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.saturating_mul(rhs)
            }
        }
        impl Div for [<Size$b>] {
            type Output = Self;

            #[inline]
            /// Saturating, clamped division.
            fn div(self, rhs: Self) -> Self {
                self.saturating_div(rhs)
            }
        }

        impl Add<$i> for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn add(self, rhs: $i) -> Self {
                self.saturating_add_value(rhs)
            }
        }
        impl Sub<$i> for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped addition.
            #[inline]
            fn sub(self, rhs: $i) -> Self {
                self.saturating_sub_value(rhs)
            }
        }
        impl Mul<$i> for [<Size$b>] {
            type Output = Self;

            /// Saturating, clamped multiplication.
            #[inline]
            fn mul(self, rhs: $i) -> Self {
                self.saturating_mul_value(rhs)
            }
        }
        impl Div<$i> for [<Size$b>] {
            type Output = Self;

            #[inline]
            /// Saturating, clamped division.
            fn div(self, rhs: $i) -> Self {
                self.saturating_div_value(rhs)
            }
        }

    }};
}
size![i8, 8, i16, 16, i32, 32, i64, 64];
