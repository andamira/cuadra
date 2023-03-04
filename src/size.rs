// revela::layout::size
//
//!
//

use core::fmt;

use super::Clamper as C;

/// A 2D size.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    w: i32,
    h: i32,
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Size {{ x: {}, y: {} }}", self.w, self.h,)
    }
}

impl Size {
    /// Defines a new `Size` with the given dimensions,
    /// which has to be at least `1`.
    pub const fn new(width: i32, height: i32) -> Self {
        Self {
            w: C::clamp_positive(width),
            h: C::clamp_positive(height),
        }
    }

    /// Get the width.
    #[inline]
    pub const fn w(&self) -> i32 {
        self.w
    }
    /// Get the height.
    #[inline]
    pub const fn h(&self) -> i32 {
        self.h
    }

    /// Set the width.
    #[inline]
    pub fn set_w(&mut self, width: i32) {
        self.w = C::clamp(width);
    }
    /// Set the height.
    #[inline]
    pub fn set_h(&mut self, height: i32) {
        self.h = C::clamp(height);
    }
}

impl Size {
    pub const fn as_tuple(&self) -> (i32, i32) {
        (self.w, self.h)
    }
    pub const fn from_tuple(tup: (i32, i32)) -> Size {
        Self::new(tup.0, tup.1)
    }

    pub const fn as_tuple_u32(&self) -> (u32, u32) {
        (
            C::clamp_positive_to_u32(self.w),
            C::clamp_positive_to_u32(self.h),
        )
    }
    pub const fn from_tuple_u32(tup: (u32, u32)) -> Size {
        Self::new(
            C::clamp_positive_from_u32(tup.0),
            C::clamp_positive_from_u32(tup.1),
        )
    }

    pub const fn as_tuple_u16(&self) -> (u16, u16) {
        (
            C::clamp_positive_to_u16(self.w),
            C::clamp_positive_to_u16(self.h),
        )
    }
    pub const fn from_tuple_u16(tup: (u16, u16)) -> Size {
        Self::new(
            C::clamp_positive_from_u16(tup.0),
            C::clamp_positive_from_u16(tup.1),
        )
    }

    pub const fn as_tuple_i16(&self) -> (i16, i16) {
        (
            C::clamp_positive_to_i16(self.w),
            C::clamp_positive_to_i16(self.h),
        )
    }
    pub const fn from_tuple_i16(tup: (i16, i16)) -> Size {
        Self::new(
            C::clamp_positive_from_i16(tup.0),
            C::clamp_positive_from_i16(tup.1),
        )
    }

    pub const fn as_tuple_usize(&self) -> (usize, usize) {
        (
            C::clamp_positive_to_usize(self.w),
            C::clamp_positive_to_usize(self.h),
        )
    }
    pub const fn from_tuple_usize(tup: (usize, usize)) -> Size {
        Self::new(
            C::clamp_positive_from_usize(tup.0),
            C::clamp_positive_from_usize(tup.1),
        )
    }
}

impl From<(i32, i32)> for Size {
    fn from(tup: (i32, i32)) -> Size {
        Self::from_tuple(tup)
    }
}
impl From<Size> for (i32, i32) {
    fn from(s: Size) -> (i32, i32) {
        s.as_tuple()
    }
}

impl From<(i16, i16)> for Size {
    fn from(tup: (i16, i16)) -> Size {
        Self::from_tuple_i16(tup)
    }
}
impl From<Size> for (i16, i16) {
    fn from(s: Size) -> (i16, i16) {
        s.as_tuple_i16()
    }
}

impl From<(u16, u16)> for Size {
    fn from(tup: (u16, u16)) -> Size {
        Self::from_tuple_u16(tup)
    }
}
impl From<Size> for (u16, u16) {
    fn from(s: Size) -> (u16, u16) {
        s.as_tuple_u16()
    }
}

impl From<(u32, u32)> for Size {
    fn from(tup: (u32, u32)) -> Size {
        Self::from_tuple_u32(tup)
    }
}
impl From<Size> for (u32, u32) {
    fn from(s: Size) -> (u32, u32) {
        s.as_tuple_u32()
    }
}

impl From<(usize, usize)> for Size {
    fn from(tup: (usize, usize)) -> Size {
        Self::from_tuple_usize(tup)
    }
}
impl From<Size> for (usize, usize) {
    fn from(s: Size) -> (usize, usize) {
        s.as_tuple_usize()
    }
}
