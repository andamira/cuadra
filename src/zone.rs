// revela::layout::zone
//
//!
//

use core::fmt;

use super::{Position, Size};

/// A 2D zone combines a [`Position`] and a [`Size`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Zone {
    pub p: Position,
    pub s: Size,
}

impl fmt::Debug for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zone {{ p: {:?}, s: {:?} }}", self.p, self.s,)
    }
}

impl Zone {
    /// Returns a new zone from the provided position and size.
    pub const fn new(position: Position, size: Size) -> Self {
        Zone {
            p: position,
            s: size,
        }
    }

    /// Returns a new zone from the provided position and size raw components.
    pub const fn new_raw(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new(Position::new(x, y), Size::new(width, height))
    }

    /// Gets a tuple with the position and size.
    pub const fn position_size(&self) -> (Position, Size) {
        (self.p, self.s)
    }

    /// Get the position.
    pub const fn position(&self) -> Position {
        self.p
    }
    /// Mutate the position.
    pub fn mut_position(&mut self, position: Position) {
        self.p = position;
    }
    /// Chain-set the position.
    pub fn set_position(mut self, position: impl Into<Position>) {
        self.p = position.into();
    }

    /// Get the size.
    pub const fn size(&self) -> Size {
        self.s
    }
    /// Mutate the size.
    pub fn mut_size(mut self, size: Size) {
        self.s = size;
    }
    /// Set the size.
    pub fn set_size(mut self, size: impl Into<Size>) {
        self.s = size.into();
    }

    /// Get the `x` position.
    pub const fn x(&self) -> i32 {
        self.p.x()
    }
    /// Set the `x` position.
    pub fn set_x(&mut self, x: i32) {
        self.p.set_x(x)
    }

    /// Get the `y` position.
    pub const fn y(&self) -> i32 {
        self.p.y()
    }
    /// Set the `y` position.
    pub fn set_y(&mut self, y: i32) {
        self.p.set_y(y)
    }

    /// Get the `width`.
    pub const fn w(&self) -> i32 {
        self.s.w()
    }
    /// Set the `width`.
    pub fn set_w(&mut self, width: i32) {
        self.s.set_w(width)
    }

    /// Get the `height`.
    pub const fn h(&self) -> i32 {
        self.s.h()
    }
    /// Set the `height`.
    pub fn set_h(&mut self, height: i32) {
        self.s.set_h(height)
    }
}

impl Zone {
    /// Returns a tuple with the `(x, y, width, height)` components.
    pub const fn as_tuple(&self) -> (i32, i32, i32, i32) {
        (self.p.x(), self.p.y(), self.s.w(), self.s.h())
    }
    /// Creates a zone from a tuple with `(x, y, width, height)` components.
    pub const fn from_tuple(tup: (i32, i32, i32, i32)) -> Zone {
        Self::new_raw(tup.0, tup.1, tup.2, tup.3)
    }

    pub const fn as_tuple_u32(&self) -> (u32, u32, u32, u32) {
        let p = self.p.as_tuple_u32();
        let s = self.s.as_tuple_u32();
        (p.0, p.1, s.0, s.1)
    }
    pub const fn from_tuple_u32(tup: (u32, u32, u32, u32)) -> Zone {
        Self::new(
            Position::from_tuple_u32((tup.0, tup.1)),
            Size::from_tuple_u32((tup.2, tup.3)),
        )
    }

    pub const fn as_tuple_u16(&self) -> (u16, u16, u16, u16) {
        let p = self.p.as_tuple_u16();
        let s = self.s.as_tuple_u16();
        (p.0, p.1, s.0, s.1)
    }
    pub const fn from_tuple_u16(tup: (u16, u16, u16, u16)) -> Zone {
        Self::new(
            Position::from_tuple_u16((tup.0, tup.1)),
            Size::from_tuple_u16((tup.2, tup.3)),
        )
    }

    pub const fn as_tuple_i16(&self) -> (i16, i16, i16, i16) {
        let p = self.p.as_tuple_i16();
        let s = self.s.as_tuple_i16();
        (p.0, p.1, s.0, s.1)
    }
    pub const fn from_tuple_i16(tup: (i16, i16, i16, i16)) -> Zone {
        Self::new(
            Position::from_tuple_i16((tup.0, tup.1)),
            Size::from_tuple_i16((tup.2, tup.3)),
        )
    }

    pub const fn as_tuple_usize(&self) -> (usize, usize, usize, usize) {
        let p = self.p.as_tuple_usize();
        let s = self.s.as_tuple_usize();
        (p.0, p.1, s.0, s.1)
    }
    pub const fn from_tuple_usize(tup: (usize, usize, usize, usize)) -> Zone {
        Self::new(
            Position::from_tuple_usize((tup.0, tup.1)),
            Size::from_tuple_usize((tup.2, tup.3)),
        )
    }
}

mod conversions {
    use super::Zone;

    impl From<(i32, i32, i32, i32)> for Zone {
        fn from(tup: (i32, i32, i32, i32)) -> Zone {
            Self::from_tuple(tup)
        }
    }
    impl From<Zone> for (i32, i32, i32, i32) {
        fn from(p: Zone) -> (i32, i32, i32, i32) {
            p.as_tuple()
        }
    }

    impl From<(i16, i16, i16, i16)> for Zone {
        fn from(tup: (i16, i16, i16, i16)) -> Zone {
            Self::from_tuple_i16(tup)
        }
    }
    impl From<Zone> for (i16, i16, i16, i16) {
        fn from(s: Zone) -> (i16, i16, i16, i16) {
            s.as_tuple_i16()
        }
    }

    impl From<(u16, u16, u16, u16)> for Zone {
        fn from(tup: (u16, u16, u16, u16)) -> Zone {
            Self::from_tuple_u16(tup)
        }
    }
    impl From<Zone> for (u16, u16, u16, u16) {
        fn from(s: Zone) -> (u16, u16, u16, u16) {
            s.as_tuple_u16()
        }
    }

    impl From<(u32, u32, u32, u32)> for Zone {
        fn from(tup: (u32, u32, u32, u32)) -> Zone {
            Self::from_tuple_u32(tup)
        }
    }
    impl From<Zone> for (u32, u32, u32, u32) {
        fn from(s: Zone) -> (u32, u32, u32, u32) {
            s.as_tuple_u32()
        }
    }

    impl From<(usize, usize, usize, usize)> for Zone {
        fn from(tup: (usize, usize, usize, usize)) -> Zone {
            Self::from_tuple_usize(tup)
        }
    }
    impl From<Zone> for (usize, usize, usize, usize) {
        fn from(s: Zone) -> (usize, usize, usize, usize) {
            s.as_tuple_usize()
        }
    }
}
