use std::convert::TryFrom;

pub fn upper_only(v: u16) -> u8 {
    (v >> 8) as u8
}

pub fn lower_only(v: u16) -> u8 {
    (v & 0x00ff) as u8
}

pub fn u8u8_to_u16(upper: u8, lower: u8) -> u16 {
    let upper = upper as u16;
    let lower = lower as u16;
    (upper << 8) + lower
}

pub fn u16_to_u8u8(v: u16) -> (u8, u8) {
    let upper = v >> 8;
    let lower = v & 0x00ff;
    (upper as u8, lower as u8)
}

#[test]
fn it_upper_only() {
    assert_eq!(upper_only(0xff00u16), 0xffu8);
    assert_eq!(upper_only(0xffffu16), 0xffu8);
}
#[test]
fn it_lower_only() {
    assert_eq!(lower_only(0xff00u16), 0x00u8);
    assert_eq!(lower_only(0xffffu16), 0xffu8);
}

#[test]
fn it_u8u8_to_u16() {
    assert_eq!(
        u8u8_to_u16(0b1100_0000u8, 0b0000_1111u8),
        0b1100_0000_0000_1111u16
    )
}

#[test]
fn it_u16_to_u8u8() {
    assert_eq!(
        u16_to_u8u8(0b1100_0000_0000_1111u16),
        (0b1100_0000u8, 0b0000_1111u8)
    )
}

#[test]
/// 仕様確認
fn it_i8_to_u8_with_try_form() {
    assert_eq!(i8::try_from(0b0000_0001u8), Ok(1));
    assert_ne!(i8::try_from(0b1000_0001u8), Ok(-1));
    assert_eq!(i8::try_from(0b0111_1111u8), Ok(127));
    assert_ne!(i8::try_from(0b1000_0000u8), Ok(-128));
}

#[test]
/// 仕様確認
fn it_shift_u8() {
    assert_eq!(0b1000_0000u8 << 1, 0b0000_0000u8);
}
