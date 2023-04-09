// cuadra::size
//
//!
//

use core::fmt;

macro_rules! size {
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
        }

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
    }};
}
size![i8, 8, i16, 16, i32, 32, i64, 64];
