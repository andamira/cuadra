// cuadra::position
//
///
//
use core::fmt;

use super::Clamper as C;

/* definitions */

/// A 2D position.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position {{ x: {}, y: {} }}", self.x, self.y,)
    }
}

impl Position {
    /// Defines a new `Position` with the given coordinates.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x: C::clamp(x),
            y: C::clamp(y),
        }
    }

    /// Get x.
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }
    /// Get y.
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Set x.
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.x = C::clamp(x);
    }
    /// Set y.
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.y = C::clamp(y);
    }
}

impl Position {
    pub const fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub const fn from_tuple(tup: (i32, i32)) -> Position {
        Self::new(C::clamp(tup.0), C::clamp(tup.1))
    }

    pub const fn as_tuple_u32(&self) -> (u32, u32) {
        (C::clamp_to_u32(self.x), C::clamp_to_u32(self.y))
    }
    pub const fn from_tuple_u32(tup: (u32, u32)) -> Position {
        Self::new(C::clamp_from_u32(tup.0), C::clamp_from_u32(tup.1))
    }

    pub const fn as_tuple_u16(&self) -> (u16, u16) {
        (
            C::clamp_positive_to_u16(self.y),
            C::clamp_positive_to_u16(self.y),
        )
    }
    pub const fn from_tuple_u16(tup: (u16, u16)) -> Position {
        Self::new(
            C::clamp_positive_from_u16(tup.0),
            C::clamp_positive_from_u16(tup.1),
        )
    }

    pub const fn as_tuple_i16(&self) -> (i16, i16) {
        (C::clamp_to_i16(self.x), C::clamp_to_i16(self.y))
    }
    pub const fn from_tuple_i16(tup: (i16, i16)) -> Position {
        Self::new(C::clamp_from_i16(tup.0), C::clamp_from_i16(tup.1))
    }

    pub const fn as_tuple_usize(&self) -> (usize, usize) {
        (
            C::clamp_positive_to_usize(self.x),
            C::clamp_positive_to_usize(self.y),
        )
    }
    pub const fn from_tuple_usize(tup: (usize, usize)) -> Position {
        Self::new(
            C::clamp_positive_from_usize(tup.0),
            C::clamp_positive_from_usize(tup.1),
        )
    }
}

impl From<(i32, i32)> for Position {
    fn from(tup: (i32, i32)) -> Position {
        Self::from_tuple(tup)
    }
}
impl From<Position> for (i32, i32) {
    fn from(p: Position) -> (i32, i32) {
        p.as_tuple()
    }
}

impl From<(i16, i16)> for Position {
    fn from(tup: (i16, i16)) -> Position {
        Self::from_tuple_i16(tup)
    }
}
impl From<Position> for (i16, i16) {
    fn from(s: Position) -> (i16, i16) {
        s.as_tuple_i16()
    }
}

impl From<(u16, u16)> for Position {
    fn from(tup: (u16, u16)) -> Position {
        Self::from_tuple_u16(tup)
    }
}
impl From<Position> for (u16, u16) {
    fn from(s: Position) -> (u16, u16) {
        s.as_tuple_u16()
    }
}

impl From<(u32, u32)> for Position {
    fn from(tup: (u32, u32)) -> Position {
        Self::from_tuple_u32(tup)
    }
}
impl From<Position> for (u32, u32) {
    fn from(s: Position) -> (u32, u32) {
        s.as_tuple_u32()
    }
}

impl From<(usize, usize)> for Position {
    fn from(tup: (usize, usize)) -> Position {
        Self::from_tuple_usize(tup)
    }
}
impl From<Position> for (usize, usize) {
    fn from(s: Position) -> (usize, usize) {
        s.as_tuple_usize()
    }
}
