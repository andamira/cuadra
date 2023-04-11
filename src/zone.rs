// cuadra::zone
//
//!
//

use core::fmt;

macro_rules! zone {
    ( $($i:ty, $b:expr),+ ) => {
        $( zone![single: $i, $b]; )+
    };

    (single: $i:ty, $b:literal) => { paste::paste! {
        use super::{[<Position$b>], [<Size$b>]};

        #[doc = "A 2D zone combines a [`" [<Position$b>] "`] with a [`" [<Size$b>] "`]."]
        #[derive(Clone, Copy, Default, PartialEq, Eq)]
        pub struct [<Zone$b>] {
            pub p: [<Position$b>],
            pub s: [<Size$b>],
        }

        impl fmt::Debug for [<Zone$b>] {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[<Zone$b>] {{ p: {:?}, s: {:?} }}", self.p, self.s,)
            }
        }

        impl fmt::Display for [<Zone$b>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.p, self.s)
            }
        }

        impl [<Zone$b>] {
            /// Returns a new zone from the provided position and size.
            pub const fn new(position: [<Position$b>], size: [<Size$b>]) -> Self {
                [<Zone$b>] {
                    p: position,
                    s: size,
                }
            }

            /// Returns a new zone from the provided position and size raw components.
            pub const fn new_raw(x: $i, y: $i, width: $i, height: $i) -> Self {
                Self::new([<Position$b>]::new(x, y), [<Size$b>]::new(width, height))
            }

            /// Gets a tuple with the position and size.
            pub const fn position_size(&self) -> ([<Position$b>], [<Size$b>]) {
                (self.p, self.s)
            }

            /// Get the position.
            pub const fn position(&self) -> [<Position$b>] {
                self.p
            }
            /// Chain-set the position.
            pub fn set_position(mut self, position: impl Into<[<Position$b>]>) {
                self.p = position.into();
            }

            /// Get the size.
            pub const fn size(&self) -> [<Size$b>] {
                self.s
            }
            /// Set the size.
            pub fn set_size(mut self, size: impl Into<[<Size$b>]>) {
                self.s = size.into();
            }

            /// Get the `x` position.
            pub const fn x(&self) -> $i {
                self.p.x()
            }
            /// Set the `x` position.
            pub fn set_x(&mut self, x: $i) {
                self.p.set_x(x)
            }

            /// Get the `y` position.
            pub const fn y(&self) -> $i {
                self.p.y()
            }
            /// Set the `y` position.
            pub fn set_y(&mut self, y: $i) {
                self.p.set_y(y)
            }

            /// Get the `width`.
            pub const fn w(&self) -> $i {
                self.s.w()
            }
            /// Set the `width`.
            pub fn set_w(&mut self, width: $i) {
                self.s.set_w(width)
            }

            /// Get the `height`.
            pub const fn h(&self) -> $i {
                self.s.h()
            }
            /// Set the `height`.
            pub fn set_h(&mut self, height: $i) {
                self.s.set_h(height)
            }
        }

        /// # conversions
        impl [<Zone$b>] {
            /// Returns a tuple with the `(x, y, width, height)` components.
            pub const fn as_tuple(&self) -> ($i, $i, $i, $i) {
                (self.p.x(), self.p.y(), self.s.w(), self.s.h())
            }
            /// Creates a zone from a tuple with `(x, y, width, height)` components.
            pub const fn from_tuple(tup: ($i, $i, $i, $i)) -> [<Zone$b>] {
                Self::new_raw(tup.0, tup.1, tup.2, tup.3)
            }

            pub const fn as_tuple_i32(&self) -> (i32, i32, i32, i32) {
                let p = self.p.as_tuple_i32();
                let s = self.s.as_tuple_i32();
                (p.0, p.1, s.0, s.1)
            }
            pub const fn from_tuple_i32(tup: (i32, i32, i32, i32)) -> [<Zone$b>] {
                Self::new(
                    [<Position$b>]::from_tuple_i32((tup.0, tup.1)),
                    [<Size$b>]::from_tuple_i32((tup.2, tup.3)),
                )
            }

            pub const fn as_tuple_u32(&self) -> (u32, u32, u32, u32) {
                let p = self.p.as_tuple_u32();
                let s = self.s.as_tuple_u32();
                (p.0, p.1, s.0, s.1)
            }
            pub const fn from_tuple_u32(tup: (u32, u32, u32, u32)) -> [<Zone$b>] {
                Self::new(
                    [<Position$b>]::from_tuple_u32((tup.0, tup.1)),
                    [<Size$b>]::from_tuple_u32((tup.2, tup.3)),
                )
            }

            pub const fn as_tuple_u16(&self) -> (u16, u16, u16, u16) {
                let p = self.p.as_tuple_u16();
                let s = self.s.as_tuple_u16();
                (p.0, p.1, s.0, s.1)
            }
            pub const fn from_tuple_u16(tup: (u16, u16, u16, u16)) -> [<Zone$b>] {
                Self::new(
                    [<Position$b>]::from_tuple_u16((tup.0, tup.1)),
                    [<Size$b>]::from_tuple_u16((tup.2, tup.3)),
                )
            }

            pub const fn as_tuple_i16(&self) -> (i16, i16, i16, i16) {
                let p = self.p.as_tuple_i16();
                let s = self.s.as_tuple_i16();
                (p.0, p.1, s.0, s.1)
            }
            pub const fn from_tuple_i16(tup: (i16, i16, i16, i16)) -> [<Zone$b>] {
                Self::new(
                    [<Position$b>]::from_tuple_i16((tup.0, tup.1)),
                    [<Size$b>]::from_tuple_i16((tup.2, tup.3)),
                )
            }

            pub const fn as_tuple_usize(&self) -> (usize, usize, usize, usize) {
                let p = self.p.as_tuple_usize();
                let s = self.s.as_tuple_usize();
                (p.0, p.1, s.0, s.1)
            }
            pub const fn from_tuple_usize(tup: (usize, usize, usize, usize)) -> [<Zone$b>] {
                Self::new(
                    [<Position$b>]::from_tuple_usize((tup.0, tup.1)),
                    [<Size$b>]::from_tuple_usize((tup.2, tup.3)),
                )
            }
        }

        impl From<(i32, i32, i32, i32)> for [<Zone$b>] {
            fn from(tup: (i32, i32, i32, i32)) -> [<Zone$b>] {
                Self::from_tuple_i32(tup)
            }
        }
        impl From<[<Zone$b>]> for (i32, i32, i32, i32) {
            fn from(z: [<Zone$b>]) -> (i32, i32, i32, i32) {
                z.as_tuple_i32()
            }
        }

        impl From<(i16, i16, i16, i16)> for [<Zone$b>] {
            fn from(tup: (i16, i16, i16, i16)) -> [<Zone$b>] {
                Self::from_tuple_i16(tup)
            }
        }
        impl From<[<Zone$b>]> for (i16, i16, i16, i16) {
            fn from(z: [<Zone$b>]) -> (i16, i16, i16, i16) {
                z.as_tuple_i16()
            }
        }

        impl From<(u16, u16, u16, u16)> for [<Zone$b>] {
            fn from(tup: (u16, u16, u16, u16)) -> [<Zone$b>] {
                Self::from_tuple_u16(tup)
            }
        }
        impl From<[<Zone$b>]> for (u16, u16, u16, u16) {
            fn from(z: [<Zone$b>]) -> (u16, u16, u16, u16) {
                z.as_tuple_u16()
            }
        }

        impl From<(u32, u32, u32, u32)> for [<Zone$b>] {
            fn from(tup: (u32, u32, u32, u32)) -> [<Zone$b>] {
                Self::from_tuple_u32(tup)
            }
        }
        impl From<[<Zone$b>]> for (u32, u32, u32, u32) {
            fn from(z: [<Zone$b>]) -> (u32, u32, u32, u32) {
                z.as_tuple_u32()
            }
        }

        impl From<(usize, usize, usize, usize)> for [<Zone$b>] {
            fn from(tup: (usize, usize, usize, usize)) -> [<Zone$b>] {
                Self::from_tuple_usize(tup)
            }
        }
        impl From<[<Zone$b>]> for (usize, usize, usize, usize) {
            fn from(z: [<Zone$b>]) -> (usize, usize, usize, usize) {
                z.as_tuple_usize()
            }
        }
    }};
}
zone![i8, 8, i16, 16, i32, 32, i64, 64];
