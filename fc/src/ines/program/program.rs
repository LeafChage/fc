use super::addressing::Addresing;
use super::operand::Operand;
use super::{Mode, OrderSet, OrderSets};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Program {
    /// debug用のIndex
    order_set_index: u8,
    pub orderset: OrderSet,
    pub operand: Operand,
    pub addr: u16,
}
impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@{:#06x} #{:#04x} => {} {}",
            self.addr, self.order_set_index, self.orderset, self.operand
        )
    }
}

impl Program {
    pub fn parse(rom: &[u8], point: u16) -> Self {
        let point_usize = point as usize;
        let order_set_index = rom[point_usize];
        let orderset = OrderSets[order_set_index as usize];
        let operand = Self::operand(&rom, point_usize, orderset.mode);
        Program {
            order_set_index,
            orderset,
            operand,
            addr: point,
        }
    }

    fn operand<T>(buf: &T, addr: usize, mode: Mode) -> Operand
    where
        T: Addresing,
    {
        match mode {
            Mode::Implied => buf.implied(addr),
            Mode::Accumulator => buf.accumulator(addr),
            Mode::Immediate => buf.immediate(addr),
            Mode::ZeroPage => buf.zero_page(addr),
            Mode::ZeroPageX => buf.indexed_zero_page(addr),
            Mode::ZeroPageY => buf.indexed_zero_page(addr),
            Mode::Absolute => buf.absolute(addr),
            Mode::AbsoluteX => buf.absolute_indirect(addr),
            Mode::AbsoluteY => buf.absolute_indirect(addr),
            Mode::Relative => buf.relative(addr),
            Mode::Indirect => buf.absolute_indirect(addr),
            Mode::IndirectX => buf.indexed_indirect(addr),
            Mode::IndirectY => buf.indirect_indexed(addr),
            Mode::NONE => buf.none(addr),
        }
    }
}
