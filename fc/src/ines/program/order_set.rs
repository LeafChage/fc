use super::{Command, Mode};
use once_cell::sync::Lazy;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct OrderSet {
    pub mode: Mode,
    pub cmd: Command,
    pub length: usize,
    pub clock: usize,
}

impl OrderSet {
    pub fn new(mode: Mode, cmd: Command, length: usize, clock: usize) -> Self {
        OrderSet {
            mode,
            cmd,
            length,
            clock,
        }
    }
    pub fn none() -> Self {
        OrderSet::new(Mode::NONE, Command::NOP, 1, 1)
    }
}
impl std::fmt::Display for OrderSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.cmd, self.mode)
    }
}

/// $ - denotes a hexadecimal address
/// % - denotes a binary address
/// # - denotes an immediate value
/// #$ - denotes an immediate hex value
/// #% - denotes an immediate binary value
pub static OrderSets: Lazy<[OrderSet; 16 * 16]> = Lazy::new(|| {
    [
        // 0x00
        // Implied       BRK           $00   1   7
        // Indirect,X    ORA ($44,X)   $01   2   6
        // Zero Page     ORA $44       $05   2   3
        // Zero Page     ASL $44       $06   2   5
        // Implied       PHP           $08   1   3
        // Immediate     ORA #$44      $09   2   2
        // Accumulator   ASL A         $0A   1   2
        // Absolute      ORA $4400     $0D   3   4
        // Absolute      ASL $4400     $0E   3   6
        OrderSet::new(Mode::Implied, Command::BRK, 1, 7),
        OrderSet::new(Mode::IndirectX, Command::ORA, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPage, Command::ORA, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::ASL, 2, 5),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::PHP, 1, 3),
        OrderSet::new(Mode::Immediate, Command::ORA, 2, 2),
        OrderSet::new(Mode::Accumulator, Command::ASL, 1, 2),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::Absolute, Command::ORA, 3, 4),
        OrderSet::new(Mode::Absolute, Command::ASL, 3, 6),
        OrderSet::none(),
        // 0x10
        // Relative      BPL $44       $10   2   2 +1or2
        // Indirect,Y    ORA ($44),Y   $11   2   5 +1
        // Zero Page,X   ORA $44,X     $15   2   4
        // Zero Page,X   ASL $44,X     $16   2   6
        // Implied       CLC           $18   1   2
        // Absolute,Y    ORA $4400,Y   $19   3   4 +1
        // Absolute,X    ORA $4400,X   $1D   3   4 +1
        // Absolute,X    ASL $4400,X   $1E   3   6 +1
        OrderSet::new(Mode::Relative, Command::BPL, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::ORA, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::ORA, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::ASL, 2, 6),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::CLC, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::ORA, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::ORA, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::ASL, 3, 6),
        OrderSet::none(),
        // 0x20
        // Absolute      JSR $5597     $20   3   6
        // Indirect,X    AND ($44,X)   $21   2   6
        // Zero Page     BIT $44       $24   2   3
        // Zero Page     AND $44       $25   2   3
        // Zero Page     ROL $44       $26   2   5
        // Implied       PLP           $28   1   4
        // Immediate     AND #$44      $29   2   2
        // Accumulator   ROL A         $2A   1   2
        // Absolute      BIT $4400     $2C   3   4
        // Absolute      AND $4400     $2D   3   4
        // Absolute      ROL $4400     $2E   3   6
        OrderSet::new(Mode::Absolute, Command::JSR, 3, 6),
        OrderSet::new(Mode::IndirectX, Command::AND, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPage, Command::BIT, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::AND, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::ROL, 2, 5),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::PLP, 1, 4),
        OrderSet::new(Mode::Immediate, Command::AND, 2, 2),
        OrderSet::new(Mode::Accumulator, Command::ROL, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Absolute, Command::BIT, 3, 4),
        OrderSet::new(Mode::Absolute, Command::AND, 3, 4),
        OrderSet::new(Mode::Absolute, Command::ROL, 3, 6),
        OrderSet::none(),
        // 0x30
        // Relative      BMI $44       $30   2   2 +1or2
        // Indirect,Y    AND ($44),Y   $31   2   5 +1
        // Zero Page,X   AND $44,X     $35   2   4
        // Zero Page,X   ROL $44,X     $36   2   6
        // Implied       SEC           $38   1   2
        // Absolute,Y    AND $4400,Y   $39   3   4 +1
        // Absolute,X    AND $4400,X   $3D   3   4 +1
        // Absolute,X    ROL $4400,X   $3E   3   6 +1
        OrderSet::new(Mode::Relative, Command::BMI, 2, 2),
        OrderSet::new(Mode::IndirectX, Command::AND, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::AND, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::ROL, 2, 6),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::SEC, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::AND, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::AND, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::ROL, 3, 6),
        OrderSet::none(),
        // 0x40
        // Implied       RTI           $40   1   6
        // Indirect,X    EOR ($44,X)   $41   2   6
        // Zero Page     EOR $44       $45   2   3
        // Zero Page     LSR $44       $46   2   5
        // Implied       PHA           $48   1   3
        // Immediate     EOR #$44      $49   2   2
        // Accumulator   LSR A         $4A   1   2
        // Absolute      JMP $5597     $4C   3   3
        // Absolute      EOR $4400     $4D   3   4
        // Absolute      LSR $4400     $4E   3   6
        OrderSet::new(Mode::Implied, Command::RTI, 1, 6),
        OrderSet::new(Mode::IndirectX, Command::EOR, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPage, Command::EOR, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::LSR, 2, 5),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::PHA, 1, 3),
        OrderSet::new(Mode::Immediate, Command::EOR, 2, 2),
        OrderSet::new(Mode::Accumulator, Command::LSR, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Absolute, Command::JMP, 3, 3),
        OrderSet::new(Mode::Absolute, Command::EOR, 3, 4),
        OrderSet::new(Mode::Absolute, Command::LSR, 3, 6),
        OrderSet::none(),
        // 0x50
        // Relative      BVC $44       $50   2   2 +1or2
        // Indirect,Y    EOR ($44),Y   $51   2   5 +1
        // Zero Page,X   EOR $44,X     $55   2   4
        // Zero Page,X   LSR $44,X     $56   2   6
        // Implied       CLI           $58   1   2
        // Absolute,Y    EOR $4400,Y   $59   3   4 +1
        // Absolute,X    EOR $4400,X   $5D   3   4 +1
        // Absolute,X    LSR $4400,X   $5E   3   6 +1
        OrderSet::new(Mode::Relative, Command::BVC, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::EOR, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::EOR, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::LSR, 2, 6),
        OrderSet::none(),
        OrderSet::new(Mode::Implied, Command::CLI, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::EOR, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::EOR, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::LSR, 3, 6),
        OrderSet::none(),
        // 0x60
        // Implied       RTS           $60   1   6
        // Indirect,X    ADC ($44,X)   $61   2   6
        // Zero Page     ADC $44       $65   2   3
        // Zero Page     ROR $44       $66   2   5
        // Implied       PLA           $68   1   4
        // Immediate     ADC #$44      $69   2   2
        // Accumulator   ROR A         $6A   1   2
        // Indirect      JMP ($5597)   $6C   3   5
        // Absolute      ADC $4400     $6D   3   4
        // Absolute      ROR $4400     $6E   3   6
        OrderSet::new(Mode::Implied, Command::RTS, 1, 6),
        OrderSet::new(Mode::IndirectX, Command::ADC, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPage, Command::ADC, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::ROR, 2, 5),
        OrderSet::none(),
        OrderSet::new(Mode::Implied, Command::PLA, 1, 4),
        OrderSet::new(Mode::Immediate, Command::ADC, 2, 2),
        OrderSet::new(Mode::Accumulator, Command::ROR, 1, 2),
        OrderSet::none(),
        OrderSet::new(Mode::Indirect, Command::JMP, 3, 5),
        OrderSet::new(Mode::Absolute, Command::ADC, 3, 4),
        OrderSet::new(Mode::Absolute, Command::ROR, 3, 6),
        OrderSet::none(),
        // 0x70
        // Relative      BVS $44       $70   2   2 +1or2
        // Indirect,Y    ADC ($44),Y   $71   2   5 +1
        // Zero Page,X   ADC $44,X     $75   2   4
        // Zero Page,X   ROR $44,X     $76   2   6
        // Implied       SEI           $78   1   2
        // Absolute,Y    ADC $4400,Y   $79   3   4 +1
        // Absolute,X    ADC $4400,X   $7D   3   4 +1
        // Absolute,X    ROR $4400,X   $7E   3   6 +1
        OrderSet::new(Mode::Relative, Command::BVS, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::ADC, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::ADC, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::ROR, 2, 6),
        OrderSet::none(),
        OrderSet::new(Mode::Implied, Command::SEI, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::ADC, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::ADC, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::ROR, 3, 6),
        OrderSet::none(),
        // 0x80
        // Indirect,X    STA ($44,X)   $81   2   6
        // Zero Page     STY $44       $84   2   3
        // Zero Page     STA $44       $85   2   3
        // Zero Page     STX $44       $86   2   3
        // Implied       DEY           $88   1   2
        // Implied       TXA           $8A   1   2
        // Absolute      STY $4400     $8C   3   4
        // Absolute      STA $4400     $8D   3   4
        // Absolute      STX $4400     $8E   3   4
        OrderSet::none(),
        OrderSet::new(Mode::IndirectX, Command::STA, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPage, Command::STY, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::STA, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::STX, 2, 3),
        OrderSet::none(),
        OrderSet::new(Mode::Implied, Command::DEY, 1, 2),
        OrderSet::none(),
        OrderSet::new(Mode::Implied, Command::TXA, 1, 2),
        OrderSet::none(),
        OrderSet::new(Mode::Absolute, Command::STY, 3, 4),
        OrderSet::new(Mode::Absolute, Command::STA, 3, 4),
        OrderSet::new(Mode::Absolute, Command::STX, 3, 4),
        OrderSet::none(),
        // 0x90
        // Relative      BCC $44       $90   2   2 +1or2
        // Indirect,Y    STA ($44),Y   $91   2   5 +1
        // Zero Page,X   STY $44,X     $94   2   4
        // Zero Page,X   STA $44,X     $95   2   4
        // Zero Page,Y   STX $44,Y     $96   2   4
        // Implied       TYA           $98   1   2
        // Absolute,Y    STA $4400,Y   $99   3   4 +1
        // Implied       TXS           $9A   1   2
        // Absolute,X    STA $4400,X   $9D   3   4 +1
        OrderSet::new(Mode::Relative, Command::BCC, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::STA, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPageX, Command::STY, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::STA, 2, 4),
        OrderSet::new(Mode::ZeroPageY, Command::STX, 2, 4),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::TYA, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::STA, 3, 4),
        OrderSet::new(Mode::Implied, Command::TXS, 1, 2),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::STA, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        // 0xa0
        // Immediate     LDY #$44      $A0   2   2
        // Indirect,X    LDA ($44,X)   $A1   2   6
        // Immediate     LDX #$44      $A2   2   2
        // Zero Page     LDY $44       $A4   2   3
        // Zero Page     LDA $44       $A5   2   3
        // Zero Page     LDX $44       $A6   2   3
        // Implied       TAY           $A8   1   2
        // Immediate     LDA #$44      $A9   2   2
        // Implied       TAX           $AA   1   2
        // Absolute      LDY $4400     $AC   3   4
        // Absolute      LDA $4400     $AD   3   4
        // Absolute      LDX $4400     $AE   3   4
        OrderSet::new(Mode::Immediate, Command::LDY, 2, 2),
        OrderSet::new(Mode::IndirectX, Command::LDA, 2, 6),
        OrderSet::new(Mode::Immediate, Command::LDX, 2, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPage, Command::LDY, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::LDA, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::LDX, 2, 3),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::TAY, 1, 2),
        OrderSet::new(Mode::Immediate, Command::LDA, 2, 2),
        OrderSet::new(Mode::Implied, Command::TAX, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Absolute, Command::LDY, 3, 4),
        OrderSet::new(Mode::Absolute, Command::LDA, 3, 4),
        OrderSet::new(Mode::Absolute, Command::LDX, 3, 4),
        OrderSet::none(),
        // 0xb0
        // Relative      BCS $44       $B0   2   2 +1or2
        // Indirect,Y    LDA ($44),Y   $B1   2   5 +1
        // Zero Page,X   LDY $44,X     $B4   2   4
        // Zero Page,X   LDA $44,X     $B5   2   4
        // Zero Page,Y   LDX $44,Y     $B6   2   4
        // Implied       CLV           $B8   1   2
        // Absolute,Y    LDA $4400,Y   $B9   3   4 +1
        // Implied       TSX           $BA   1   2
        // Absolute,X    LDY $4400,X   $BC   3   4 +1
        // Absolute,X    LDA $4400,X   $BD   3   4 +1
        // Absolute,Y    LDX $4400,Y   $BE   3   4 +1
        OrderSet::new(Mode::Relative, Command::BCS, 2, 2),
        OrderSet::new(Mode::IndirectX, Command::LDA, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPageX, Command::LDY, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::LDA, 2, 4),
        OrderSet::new(Mode::ZeroPageY, Command::LDX, 2, 4),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::CLV, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::LDA, 3, 4),
        OrderSet::new(Mode::Implied, Command::TSX, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::AbsoluteX, Command::LDY, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::LDA, 3, 4),
        OrderSet::new(Mode::AbsoluteY, Command::LDX, 3, 4),
        OrderSet::none(),
        // 0xc0
        // Immediate     CPY #$44      $C0   2   2
        // Indirect,X    CMP ($44,X)   $C1   2   6
        // Zero Page     CPY $44       $C4   2   3
        // Zero Page     CMP $44       $C5   2   3
        // Zero Page     DEC $44       $C6   2   5
        // Implied       INY           $C8   1   2
        // Immediate     CMP #$44      $C9   2   2
        // Implied       DEX           $CA   1   2
        // Absolute      CPY $4400     $CC   3   4
        // Absolute      CMP $4400     $CD   3   4
        // Absolute      DEC $4400     $CE   3   6
        OrderSet::new(Mode::Immediate, Command::CPY, 2, 2),
        OrderSet::new(Mode::IndirectX, Command::CMP, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPage, Command::CPY, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::CMP, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::DEC, 2, 5),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::INY, 1, 2),
        OrderSet::new(Mode::Immediate, Command::CMP, 2, 2),
        OrderSet::new(Mode::Implied, Command::DEX, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Absolute, Command::CPY, 3, 4),
        OrderSet::new(Mode::Absolute, Command::CMP, 3, 4),
        OrderSet::new(Mode::Absolute, Command::DEC, 3, 6),
        OrderSet::none(),
        // 0xd0
        // Relative      BNE $44       $D0   2   2 +1or2
        // Indirect,Y    CMP ($44),Y   $D1   2   5 +1
        // Zero Page,X   CMP $44,X     $D5   2   4
        // Zero Page,X   DEC $44,X     $D6   2   6
        // Implied       CLD           $D8   1   2
        // Absolute,Y    CMP $4400,Y   $D9   3   4 +1
        // Absolute,X    CMP $4400,X   $DD   3   4 +1
        // Absolute,X    DEC $4400,X   $DE   3   6 +1
        OrderSet::new(Mode::Relative, Command::BNE, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::CMP, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::CMP, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::DEC, 2, 6),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::CLD, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::CMP, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::CMP, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::DEC, 3, 6),
        OrderSet::none(),
        // 0xe0
        // Immediate     CPX #$44      $E0   2   2
        // Indirect,X    SBC ($44,X)   $E1   2   6
        // Zero Page     CPX $44       $E4   2   3
        // Zero Page     SBC $44       $E5   2   3
        // Zero Page     INC $44       $E6   2   5
        // Implied       INX           $E8   1   2
        // Immediate     SBC #$44      $E9   2   2
        // Implied       NOP           $EA   1   2
        // Absolute      CPX $4400     $EC   3   4
        // Absolute      SBC $4400     $ED   3   4
        // Absolute      INC $4400     $EE   3   6
        OrderSet::new(Mode::Immediate, Command::CPX, 2, 2),
        OrderSet::new(Mode::IndirectX, Command::SBC, 2, 6),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::new(Mode::ZeroPage, Command::CPX, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::SBC, 2, 3),
        OrderSet::new(Mode::ZeroPage, Command::INC, 2, 5),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::INX, 1, 2),
        OrderSet::new(Mode::Immediate, Command::SBC, 2, 2),
        OrderSet::new(Mode::Implied, Command::NOP, 1, 2),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Absolute, Command::CPX, 3, 4),
        OrderSet::new(Mode::Absolute, Command::SBC, 3, 4),
        OrderSet::new(Mode::Absolute, Command::INC, 3, 6),
        OrderSet::none(),
        // 0xf0
        // Relative      BEQ $44       $F0   2   2 +1or2
        // Indirect,Y    SBC ($44),Y   $F1   2   5 +1
        // Zero Page,X   SBC $44,X     $F5   2   4
        // Zero Page,X   INC $44,X     $F6   2   6
        // Implied       SED           $F8   1   2
        // Absolute,Y    SBC $4400,Y   $F9   3   4 +1
        // Absolute,X    SBC $4400,X   $FD   3   4 +1
        // Absolute,X    INC $4400,X   $FE   3   6 +1
        OrderSet::new(Mode::Relative, Command::BEQ, 2, 2),
        OrderSet::new(Mode::IndirectY, Command::SBC, 2, 5),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::ZeroPageX, Command::SBC, 2, 4),
        OrderSet::new(Mode::ZeroPageX, Command::INC, 2, 6),
        OrderSet::none(),
        //
        OrderSet::new(Mode::Implied, Command::SED, 1, 2),
        OrderSet::new(Mode::AbsoluteY, Command::SBC, 3, 4),
        OrderSet::none(),
        OrderSet::none(),
        //
        OrderSet::none(),
        OrderSet::new(Mode::AbsoluteX, Command::SBC, 3, 4),
        OrderSet::new(Mode::AbsoluteX, Command::INC, 3, 6),
        OrderSet::none(),
    ]
});
