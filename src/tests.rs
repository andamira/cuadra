// cuadra::tests
//

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
