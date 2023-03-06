// cuadra::clamper
//
//!
//

#![allow(dead_code)]

/// Generates constant saturating casting functions between primitives.
macro_rules! scast {
    (all_orig: $($orig:ty),+) => {
        $( scast![all_dest: $orig; i8, u8, i16, u16, i32, u32, i64, u64, usize, isize]; )+
    };

    (all_dest: $orig:ty; $($dest:ty),+) => {
        $( scast![single: $orig, $dest]; )+
    };

    (single: $orig:ty, $dest:ty) => {
        paste::paste! {
            #[doc = "Returns a saturating casted value from " $orig " to " $dest "."]
            #[inline]
            const fn [<cast_$orig _$dest>](orig: $orig) -> $dest {
                let casted = orig as $dest;
                // if casted != orig it can be either 0 or -1.
                let overflow = orig != casted as $orig;
                if overflow { if casted == 0 { $dest::MIN } else { $dest::MAX } } else { casted }
            }
        }
    };
}
scast![all_orig: i8, u8, i16, u16, i32, u32, i64, u64, usize, isize];

/// Generates size-specific Clamper implementations.
macro_rules! macro_clamper {
    ($($ip:ty, $b:literal),+) => {
        $( macro_clamper![single: $ip, $b]; )+
    };

    // $ip: inner primitive
    // $b: bit size
    (single: $ip:ty, $b:literal) => {
        paste::paste! {
            #[doc = "Clamps distances to half the extent of an [`" $ip "`]."]
            ///
            /// This is to ensure a safety margin where big sizes can be
            /// positioned at high positions without becoming distorted.
            pub struct [<Clamper$b>];

            impl [<Clamper$b>] {
                /// Minimum value for any layout dimension.
                pub const MIN: $ip = $ip::MIN / 2;

                /// Maximum value for any layout dimension.
                pub const MAX: $ip = $ip::MAX / 2;

                /* clamping from the same primitive */

                #[doc = "Clamps [`" $ip "`] distance to [`MIN`][Self::MIN]`..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp(d: $ip) -> $ip {
                    if d < Self::MIN {
                        Self::MIN
                    } else if d > Self::MAX {
                        Self::MAX
                    } else {
                        d
                    }
                }

                #[doc = "Clamps [`" $ip "`] distance to `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_non_negative(d: $ip) -> $ip {
                    if d < 0 {
                        0
                    } else if d > Self::MAX {
                        Self::MAX
                    } else {
                        d
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive(d: $ip) -> $ip {
                    if d < 1 {
                        1
                    } else if d > Self::MAX {
                        Self::MAX
                    } else {
                        d
                    }
                }

                /* from/to i32*/

                #[doc = "Clamps [`i32`] distance to [`" $ip "`] [`MIN`][Self::MIN]`..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_from_i32(d: i32) -> $ip {
                    [<cast_i32_$ip>](d)
                }
                #[doc = "Clamps [`i32`] distance to [`" $ip "`] `0..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_non_negative_from_i32(d: i32) -> $ip {
                    if d < 0 {
                        0
                    } else {
                        [<cast_i32_$ip>](d)
                    }
                }
                #[doc = "Clamps [`i32`] distance to [`" $ip "`] `1..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_positive_from_i32(d: i32) -> $ip {
                    if d < 1 {
                        1
                    } else {
                        [<cast_i32_$ip>](d)
                    }
                }

                //

                #[doc = "Clamps [`" $ip "`] distance to [`i32`] [`MIN`][Self::MIN]`..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_to_i32(d: $ip) -> i32 {
                    if d < [<cast_i32_ $ip>](i32::MIN) {
                        i32::MIN
                    } else if d > [<cast_i32_ $ip>](i32::MAX) {
                        i32::MAX
                    } else {
                        [<cast_$ip _i32>](d)
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to [`i32`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_non_negative_to_i32(d: $ip) -> i32 {
                    if d < 0 {
                        0
                    } else if d > [<cast_i32_ $ip>](i32::MAX) {
                        i32::MAX
                    } else {
                        [<cast_$ip _i32>](d)
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to [`i32`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_to_i32(d: $ip) -> i32 {
                    if d < 1 {
                        1
                    } else if d > [<cast_i32_ $ip>](i32::MAX) {
                        i32::MAX
                    } else {
                        [<cast_$ip _i32>](d)
                    }
                }

                /* from/to u32 (e.g. notcurses, tiny-skia)*/

                #[doc = "Clamps an [`u32`] distance to [`" $ip "`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_from_u32(d: u32) -> $ip {
                    if d > [<cast_$ip _u32>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_u32_$ip>](d)
                    }
                }
                #[doc = "Clamps [`u32`] distance to [`" $ip "`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_from_u32(d: u32) -> $ip {
                    if d == 0 {
                        1
                    } else if d > [<cast_$ip _u32>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_u32_$ip>](d)
                    }
                }

                //

                #[doc = "Clamps [`" $ip "`] distance to [`u32`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_to_u32(d: $ip) -> u32 {
                    [<cast _$ip _u32>](Self::clamp_non_negative(d))
                }
                #[doc = "Clamps [`" $ip "`] distance to [`u32`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_to_u32(d: $ip) -> u32 {
                    [<cast _$ip _u32>](Self::clamp_positive(d))
                }

                /* from/to u16 (e.g. crossterm) */

                #[doc = "Clamps [`u16`] distance to [`" $ip "`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_from_u16(d: u16) -> $ip {
                    if d > [<cast_$ip _u16>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_u16_$ip>](d)
                    }
                }
                #[doc = "Clamps [`u16`] distance to [`" $ip "`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_from_u16(d: u16) -> $ip {
                    if d == 0 {
                        1
                    } else if d > [<cast_$ip _u16>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_u16_$ip>](d)
                    }
                }

                //

                #[doc = "Clamps [`" $ip "`] distance to [`u16`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_to_u16(d: $ip) -> u16 {
                    if d < 0 {
                        0
                    } else if d > [<cast_u16_ $ip>](u16::MAX) {
                        u16::MAX
                    } else {
                        [<cast_$ip _u16>](d)
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to [`u16`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_to_u16(d: $ip) -> u16 {
                    if d < 1 {
                        1
                    } else if d > [<cast_u16_ $ip>](u16::MAX) {
                        u16::MAX
                    } else {
                        [<cast_$ip _u16>](d)
                    }
                }

                /* from/to i16 (e.g. sdl) */

                #[doc = "Clamps [`i16`] distance to [`" $ip "`] [`MIN`][Self::MIN]`..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_from_i16(d: i16) -> $ip {
                    [<cast_i16_$ip>](d)
                }
                #[doc = "Clamps [`i16`] distance to [`" $ip "`] `0..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_non_negative_from_i16(d: i16) -> $ip {
                    if d < 0 {
                        0
                    } else {
                        [<cast_i16_$ip>](d)
                    }
                }
                #[doc = "Clamps [`i16`] distance to [`" $ip "`] `1..`[`MAX`][Self::MAX]."]
                #[inline(always)]
                pub const fn clamp_positive_from_i16(d: i16) -> $ip {
                    if d < 1 {
                        1
                    } else {
                        [<cast_i16_$ip>](d)
                    }
                }

                //

                #[doc = "Clamps [`" $ip "`] distance to [`i16`] [`MIN`][Self::MIN]`..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_to_i16(d: $ip) -> i16 {
                    if d < [<cast_i16_ $ip>](i16::MIN) {
                        i16::MIN
                    } else if d > [<cast_i16_ $ip>](i16::MAX) {
                        i16::MAX
                    } else {
                        [<cast_$ip _i16>](d)
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to [`i16`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_non_negative_to_i16(d: $ip) -> i16 {
                    if d < 0 {
                        0
                    } else if d > [<cast_i16_ $ip>](i16::MAX) {
                        i16::MAX
                    } else {
                        [<cast_$ip _i16>](d)
                    }
                }
                #[doc = "Clamps [`" $ip "`] distance to [`i16`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_to_i16(d: $ip) -> i16 {
                    if d < 1 {
                        1
                    } else if d > [<cast_i16_ $ip>](i16::MAX) {
                        i16::MAX
                    } else {
                        [<cast_$ip _i16>](d)
                    }
                }

                /* from/to usize (e.g. machine word size & slice len)*/

                #[doc = "Clamps [`usize`] distance to [`" $ip "`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_from_usize(d: usize) -> $ip {
                    if d > [<cast_$ip _usize>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_usize_$ip>](d)
                    }
                }
                #[doc = "Clamps [`usize`] distance to [`" $ip "`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_from_usize(d: usize) -> $ip {
                    if d == 0 {
                        1
                    } else if d > [<cast_$ip _usize>](Self::MAX) {
                        Self::MAX
                    } else {
                        [<cast_usize_$ip>](d)
                    }
                }

                //

                #[doc = "Clamps [`" $ip "`] distance to [`usize`] `0..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_to_usize(d: $ip) -> usize {
                    [<cast_$ip _usize>](Self::clamp_non_negative(d))
                }
                #[doc = "Clamps [`" $ip "`] distance to [`usize`] `1..`[`MAX`][Self::MAX]."]
                #[inline]
                pub const fn clamp_positive_to_usize(d: $ip) -> usize {
                    [<cast_$ip _usize>](Self::clamp_positive(d))
                }
            }
        }
    };
}

macro_clamper![i8, 8, i16, 16, i32, 32, i64, 64];
// macro_clamper![i16, 16];
// macro_clamper![i32, 32];
// macro_clamper![i64, 64];

#[cfg(test)]
mod tests {
    use super::*;

    // MAYBE
    // use core::cmp::min;
    // use az::SaturatingAs;

    macro_rules! basics {
        ($c:ident, $i:ident, $min:literal, $max:literal) => {
            assert_eq![$c::MIN, $min];
            assert_eq![$c::MAX, $max];

            assert_eq![$c::MIN, $c::clamp($i::MIN)];
            assert_eq![$c::MAX, $c::clamp($i::MAX)];

            assert_eq![0, $c::clamp_non_negative($i::MIN)];
            assert_eq![$c::MAX, $c::clamp_non_negative($i::MAX)];

            assert_eq![1, $c::clamp_positive($i::MIN)];
            assert_eq![$c::MAX, $c::clamp_positive($i::MAX)];

            assert_eq![$c::MAX * 2, $i::MAX - 1];
        };
    }

    #[test]
    #[rustfmt::skip]
    fn clamp_basics() {
        basics![Clamper8, i8, -64, 63];
        basics![Clamper16, i16, -16_384, 16_383];
        basics![Clamper32, i32, -1_073_741_824, 1_073_741_823];
        basics![Clamper64, i64, -4_611_686_018_427_387_904, 4_611_686_018_427_387_903];
    }

    // macro_rules! clamp_u32 {
    //     ($($c:ident, $i:ty),+) => { $( clamp_u32![single: $c, $i]; )+ };
    //     (single: $c:ident, $i:ty) => {
    //         assert_eq![0, $c::clamp_from_u32(0)];
    //         assert_eq![10, $c::clamp_from_u32(10)];
    //         assert_eq![min($c::MAX, u32::MAX.saturating_as::<$i>()), $c::clamp_from_u32(u32::MAX)];
    //     };
    // }
    // #[test]
    // fn clamp_u32() {
    //     clamp_u32![Clamper8, i8, Clamper16, i16, Clamper32, i32, Clamper64, i64];
    // }

    #[test]
    fn clamping_32() {
        use super::Clamper32 as C;

        /* i32 */

        assert_eq![99, C::clamp(99)];
        assert_eq![-99, C::clamp(-99)];
        assert_eq![C::MAX, C::clamp(i32::MAX)];
        assert_eq![C::MIN, C::clamp(i32::MIN)];
        //
        assert_eq![C::MAX, C::clamp_non_negative(i32::MAX)];
        assert_eq![0, C::clamp_non_negative(i32::MIN)];
        //
        assert_eq![C::MAX, C::clamp_positive(i32::MAX)];
        assert_eq![1, C::clamp_positive(i32::MIN)];

        /* from/to u32 */

        assert_eq![C::MAX, C::clamp_from_u32(u32::MAX)];
        assert_eq![0, C::clamp_from_u32(u32::MIN)];
        assert_eq![C::MAX, C::clamp_positive_from_u32(u32::MAX)];
        assert_eq![1, C::clamp_positive_from_u32(u32::MIN)];
        //
        assert_eq![C::MAX as u32, C::clamp_to_u32(i32::MAX)];
        assert_eq![0_u32, C::clamp_to_u32(i32::MIN)];
        assert_eq![C::MAX as u32, C::clamp_positive_to_u32(i32::MAX)];
        assert_eq![1_u32, C::clamp_positive_to_u32(i32::MIN)];

        /* from/to i16 */

        assert_eq![i16::MAX as i32, C::clamp_from_i16(i16::MAX)];
        assert_eq![i16::MIN as i32, C::clamp_from_i16(i16::MIN)];
        assert_eq![i16::MAX as i32, C::clamp_non_negative_from_i16(i16::MAX)];
        assert_eq![0_i32, C::clamp_non_negative_from_i16(i16::MIN)];
        assert_eq![i16::MAX as i32, C::clamp_positive_from_i16(i16::MAX)];
        assert_eq![1_i32, C::clamp_positive_from_i16(i16::MIN)];
        //
        assert_eq![i16::MAX, C::clamp_to_i16(i32::MAX)];
        assert_eq![i16::MIN, C::clamp_to_i16(i32::MIN)];
        assert_eq![i16::MAX, C::clamp_non_negative_to_i16(i32::MAX)];
        assert_eq![0_i16, C::clamp_non_negative_to_i16(i32::MIN)];
        assert_eq![i16::MAX, C::clamp_positive_to_i16(i32::MAX)];
        assert_eq![1_i16, C::clamp_positive_to_i16(i32::MIN)];

        /* from/to usize */
        assert_eq![C::MAX, C::clamp_from_usize(usize::MAX)];
        assert_eq![0, C::clamp_from_usize(usize::MIN)];
        assert_eq![C::MAX, C::clamp_positive_from_usize(usize::MAX)];
        assert_eq![1, C::clamp_positive_from_usize(usize::MIN)];
        //
        assert_eq![C::MAX as usize, C::clamp_to_usize(i32::MAX)];
        assert_eq![0_usize, C::clamp_to_usize(i32::MIN)];
        assert_eq![C::MAX as usize, C::clamp_positive_to_usize(i32::MAX)];
        assert_eq![1_usize, C::clamp_positive_to_usize(i32::MIN)];
    }
}
