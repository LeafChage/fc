use crate::binary;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operand {
    Data(u8),
    Addr(u16),
    None,
}

impl Operand {
    pub fn addr(&self) -> u16 {
        if let Operand::Addr(addr) = self {
            addr.clone()
        } else {
            panic!("unexpected");
        }
    }

    pub fn addrs(&self) -> (u8, u8) {
        if let Operand::Addr(addr) = self {
            binary::u16_to_u8u8(addr.clone())
        } else {
            panic!("unexpected");
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            Operand::Data(v) => write!(f, "{}", v)?,
            Operand::Addr(a) => write!(f, "{:#x}", a)?,
            Operand::None => write!(f, "-")?,
        })
    }
}
