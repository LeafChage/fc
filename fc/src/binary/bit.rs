const BYTE_UNIT_BIT_COUNT: usize = 8;
const MASKS: &'static [u8; BYTE_UNIT_BIT_COUNT] = &[
    0b0111_1111,
    0b1011_1111,
    0b1101_1111,
    0b1110_1111,
    0b1111_0111,
    0b1111_1011,
    0b1111_1101,
    0b1111_1110,
];
const FULL_MASK: u8 = 0b1111_1111;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Bit(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

impl std::convert::From<Bit> for Vec<u8> {
    fn from(v: Bit) -> Self {
        let Bit(a1, a2, a3, a4, a5, a6, a7, a8) = v;
        vec![a1, a2, a3, a4, a5, a6, a7, a8]
    }
}

impl std::convert::From<u8> for Bit {
    fn from(v: u8) -> Self {
        Bit(
            bool_to_bit(MASKS[0] | v == FULL_MASK),
            bool_to_bit(MASKS[1] | v == FULL_MASK),
            bool_to_bit(MASKS[2] | v == FULL_MASK),
            bool_to_bit(MASKS[3] | v == FULL_MASK),
            bool_to_bit(MASKS[4] | v == FULL_MASK),
            bool_to_bit(MASKS[5] | v == FULL_MASK),
            bool_to_bit(MASKS[6] | v == FULL_MASK),
            bool_to_bit(MASKS[7] | v == FULL_MASK),
        )
    }
}

impl std::ops::Add for Bit {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let Bit(a1, a2, a3, a4, a5, a6, a7, a8) = self;
        let Bit(b1, b2, b3, b4, b5, b6, b7, b8) = other;
        Bit(
            a1 + b1,
            a2 + b2,
            a3 + b3,
            a4 + b4,
            a5 + b5,
            a6 + b6,
            a7 + b7,
            a8 + b8,
        )
    }
}

impl std::fmt::Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bit(a1, a2, a3, a4, a5, a6, a7, a8) = self;
        write!(f, "{}{}{}{}{}{}{}{}", a1, a2, a3, a4, a5, a6, a7, a8)
    }
}

fn bool_to_bit(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

#[test]
fn it_byte_to_bit() {
    assert_eq!(Bit::from(0b0101_0101), Bit(0, 1, 0, 1, 0, 1, 0, 1))
}
