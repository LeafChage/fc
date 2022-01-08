use super::ppu::PPU;
use crate::binary;

pub const PPUCTRL_INDEX: u16 = 0x2000;
pub const PPUMASK_INDEX: u16 = 0x2001;
pub const PPUSTATUS_INDEX: u16 = 0x2002;
pub const OAMADDR_INDEX: u16 = 0x2003;
pub const OAMDATA_INDEX: u16 = 0x2004;
pub const PPUSCROLL_INDEX: u16 = 0x2005;
pub const PPUADDR_INDEX: u16 = 0x2006;
pub const PPUDATA_INDEX: u16 = 0x2007;

pub type PPUADDRRegister = (u8, u8);

#[derive(PartialEq, Eq, Debug, Default)]
pub struct PPUDATARegister {
    pub cached: Vec<(u16, u8)>,
    pub data: u8,
}

impl PPUDATARegister {
    pub fn clear(&mut self) {
        self.cached = vec![];
    }
}

/// CPUのMemoryMapに存在している
/// CPUはPPUのVRAMに直接アクセスできないので、ここを通してアクセスする。
#[allow(non_snake_case)]
#[derive(PartialEq, Eq, Debug, Default)]
pub struct IORegister {
    /// 0x2000 	PPUCTRL 	W 	コントロールレジスタ1
    pub PPUCTRL: u8,
    /// 0x2001 	PPUMASK 	W 	コントロールレジスタ2
    pub PPUMASK: u8,
    /// 0x2002 	PPUSTATUS 	R 	PPUステータス
    pub PPUSTATUS: u8,
    /// 0x2003 	OAMADDR 	W 	スプライトメモリデータ
    pub OAMADDR: u8,
    /// 0x2004 	OAMDATA 	RW 	デシマルモード
    pub OAMDATA: u8,
    /// 0x2005 	PPUSCROLL 	W 	背景スクロールオフセット
    pub PPUSCROLL: u8,
    /// 0x2006 	PPUADDR 	W 	PPUメモリアドレス
    /// $2007を経由してPPUメモリへ書き込む16ビットアドレスを指定する。
    /// 上位8ビット、下位8ビットの順に書き込む。
    pub PPUADDR: PPUADDRRegister,
    /// 0x2007 	PPUDATA 	RW 	PPUメモリデータ
    /// $2006によって指定されたPPUメモリアドレスへデータを書き込む。
    /// 書き込む度にメモリアドレスはインクリメント($2000のビット2によって+=1、+=32)する
    pub PPUDATA: PPUDATARegister,
}

/// reference: http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#instruction
impl IORegister {
    pub fn read(&self, i: u16) -> u8 {
        match i {
            PPUCTRL_INDEX => panic!("PPUCTRL writeonly"),
            PPUMASK_INDEX => panic!("PPUMASK writeonly"),
            PPUSTATUS_INDEX => self.PPUSTATUS,
            OAMADDR_INDEX => panic!("OAMADDR writeonly"),
            OAMDATA_INDEX => self.OAMDATA,
            PPUSCROLL_INDEX => panic!("PPUSCROLL writeonly"),
            PPUADDR_INDEX => panic!("PPUADDR writeonly"),
            PPUDATA_INDEX => self.PPUDATA.data,
            _ => panic!("index out of range"),
        }
    }

    pub fn write(&mut self, i: u16, d: u8) {
        match i {
            PPUCTRL_INDEX => {
                self.PPUCTRL = d;
            }
            PPUMASK_INDEX => {
                self.PPUMASK = d;
            }
            PPUSTATUS_INDEX => panic!("PPUSTATUS readonly"),
            OAMADDR_INDEX => {
                self.OAMADDR = d;
            }
            OAMDATA_INDEX => {
                self.OAMDATA = d;
            }
            PPUSCROLL_INDEX => {
                self.PPUSCROLL = d;
            }
            PPUADDR_INDEX => {
                let (upper, lower) = self.PPUADDR;
                self.PPUADDR = (lower, d);
            }
            PPUDATA_INDEX => {
                let (upper, lower) = self.PPUADDR;
                let addr = binary::u8u8_to_u16(upper, lower);
                // println!("write memory {} {}", addr, d);
                self.PPUDATA.cached.push((addr, d));
                self.PPUDATA.data = d;
                self.PPUADDR = binary::u16_to_u8u8(addr + 1);
            }
            _ => panic!("index out of range"),
        }
    }
}

impl std::fmt::Display for IORegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PPUCTRL: {:#x}, \nPPUMASK: {:#x}, \nPPUSTATUS: {:#x}, \nOAMADDR: {:#x}, \nOAMDATA: {:#x}, \nPPUSCROLL: {:#x}, \nPPUADDR: ({:?}, {:?}), \nPPUDATA: {:#x}\ncached: {:?}",
            self.PPUCTRL,
            self.PPUMASK,
            self.PPUSTATUS,
            self.OAMADDR,
            self.OAMDATA,
            self.PPUSCROLL,
            self.PPUADDR.0,
            self.PPUADDR.1,
            self.PPUDATA.data,
            self.PPUDATA.cached,
            )
    }
}
