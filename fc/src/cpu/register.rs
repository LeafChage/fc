use super::status_register::StatusRegister;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    /// アキュームレータ
    pub A: u8,
    /// インデックスレジスタ
    pub X: u8,
    /// インデックスレジスタ
    pub Y: u8,
    /// プログラムカウンタ
    pub PC: u16,
    /// スタックポインタ
    /// スタックはWRAMの256が使える
    /// 上位アドレスは0x01に固定
    /// 0x0100～0x01FF
    pub SP: u16,
    /// プロセッサステータスレジスタ
    pub P: StatusRegister,
}

/// reference: http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#instruction
impl Register {
    pub fn new() -> Self {
        Register {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            PC: 0x00,
            SP: 0x01ff,
            P: StatusRegister::new(),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {:#x},  X: {:#x}, Y: {:#x}, PC: {:#x}, SP: {:#x}, \nP: NVRBDIZC: {}{}{}{}{}{}{}{}",
            self.A,
            self.X,
            self.Y,
            self.PC,
            self.SP,
            if self.P.n() { 1 } else { 0 },
            if self.P.v() { 1 } else { 0 },
            if self.P.r() { 1 } else { 0 },
            if self.P.b() { 1 } else { 0 },
            if self.P.d() { 1 } else { 0 },
            if self.P.i() { 1 } else { 0 },
            if self.P.z() { 1 } else { 0 },
            if self.P.c() { 1 } else { 0 },
        )
    }
}
