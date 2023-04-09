// cuadra::position
//
///
//
use core::fmt;

macro_rules! position {
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
            /// Defines a new `Position` with the given dimensions,
            /// which has to be at least `1`.
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
        }

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
            fn from(s: [<Position$b>]) -> (i32, i32) {
                s.as_tuple_i32()
            }
        }

        impl From<(u32, u32)> for [<Position$b>] {
            fn from(tup: (u32, u32)) -> [<Position$b>] {
                Self::from_tuple_u32(tup)
            }
        }
        impl From<[<Position$b>]> for (u32, u32) {
            fn from(s: [<Position$b>]) -> (u32, u32) {
                s.as_tuple_u32()
            }
        }

        impl From<(usize, usize)> for [<Position$b>] {
            fn from(tup: (usize, usize)) -> [<Position$b>] {
                Self::from_tuple_usize(tup)
            }
        }
        impl From<[<Position$b>]> for (usize, usize) {
            fn from(s: [<Position$b>]) -> (usize, usize) {
                s.as_tuple_usize()
            }
        }
    }};
}
position![i8, 8, i16, 16, i32, 32, i64, 64];
