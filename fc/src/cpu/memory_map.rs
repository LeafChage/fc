use crate::binary::DisplayBinary;
use crate::ppu::io_register::IORegister;
use std::vec::*;

const WRAM_RANGE: std::ops::Range<usize> = 0x0000..0x0800;
const WRAM_MIRROR_RANGE: std::ops::Range<usize> = 0x0800..0x2000;
const PPU_REGISTER_RANGE: std::ops::Range<usize> = 0x2000..0x2008;
const PPU_MIRROR_RANGE: std::ops::Range<usize> = 0x2008..0x4000;
const APU_RANGE: std::ops::Range<usize> = 0x4000..0x4020;
const ROM_RANGE: std::ops::Range<usize> = 0x4020..0x6000;
const RAM_RANGE: std::ops::Range<usize> = 0x6000..0x8000;
const PRG_ROM_RANGE: std::ops::Range<usize> = 0x8000..0x10000;

#[derive(PartialEq, Eq, Debug)]
pub struct MemoryMap {
    /// スタックポインタ
    /// スタックはWRAMの256が使える
    /// 上位アドレスは0x01に固定
    /// 0x0100～0x01FF
    pub wram: [u8; WRAM_RANGE.end - WRAM_RANGE.start],
    pub wram_mirror: [u8; WRAM_MIRROR_RANGE.end - WRAM_MIRROR_RANGE.start],
    pub ppu: IORegister,
    pub ppu_mirror: [u8; PPU_MIRROR_RANGE.end - PPU_MIRROR_RANGE.start],
    pub apu: [u8; APU_RANGE.end - APU_RANGE.start],
    pub rom: [u8; ROM_RANGE.end - ROM_RANGE.start],
    pub ram: [u8; RAM_RANGE.end - RAM_RANGE.start],
    pub prg_rom: [u8; PRG_ROM_RANGE.end - PRG_ROM_RANGE.start],
}

impl MemoryMap {
    pub fn new(program: Vec<u8>) -> Self {
        let mut m = MemoryMap {
            wram: [0u8; WRAM_RANGE.end - WRAM_RANGE.start],
            wram_mirror: [0u8; WRAM_MIRROR_RANGE.end - WRAM_MIRROR_RANGE.start],
            ppu: IORegister::default(),
            ppu_mirror: [0u8; PPU_MIRROR_RANGE.end - PPU_MIRROR_RANGE.start],
            apu: [0u8; APU_RANGE.end - APU_RANGE.start],
            rom: [0u8; ROM_RANGE.end - ROM_RANGE.start],
            ram: [0u8; RAM_RANGE.end - RAM_RANGE.start],
            prg_rom: [0u8; PRG_ROM_RANGE.end - PRG_ROM_RANGE.start],
        };
        for i in 0..program.len() {
            m.prg_rom[i] = program[i];
        }
        m
    }

    pub fn read(&self, p: u16) -> u8 {
        let p = p as usize;
        if WRAM_RANGE.contains(&p) {
            self.wram[p - WRAM_RANGE.start]
        } else if WRAM_MIRROR_RANGE.contains(&p) {
            self.wram_mirror[p - WRAM_MIRROR_RANGE.start]
        } else if PPU_REGISTER_RANGE.contains(&p) {
            self.ppu.read(p as u16)
        } else if PPU_MIRROR_RANGE.contains(&p) {
            self.ppu_mirror[p - PPU_MIRROR_RANGE.start]
        } else if APU_RANGE.contains(&p) {
            self.apu[p - APU_RANGE.start]
        } else if ROM_RANGE.contains(&p) {
            self.rom[p - ROM_RANGE.start]
        } else if RAM_RANGE.contains(&p) {
            self.ram[p - RAM_RANGE.start]
        } else if PRG_ROM_RANGE.contains(&p) {
            self.prg_rom[p - PRG_ROM_RANGE.start]
        } else {
            panic!("???")
        }
    }

    pub fn write(&mut self, p: u16, data: u8) {
        let p = p as usize;
        if WRAM_RANGE.contains(&p) {
            self.wram[p - WRAM_RANGE.start] = data;
        } else if WRAM_MIRROR_RANGE.contains(&p) {
            self.wram_mirror[p - WRAM_MIRROR_RANGE.start] = data;
        } else if PPU_REGISTER_RANGE.contains(&p) {
            self.ppu.write(p as u16, data);
        } else if PPU_MIRROR_RANGE.contains(&p) {
            self.ppu_mirror[p - PPU_MIRROR_RANGE.start] = data;
        } else if APU_RANGE.contains(&p) {
            self.apu[p - APU_RANGE.start] = data;
        } else if ROM_RANGE.contains(&p) {
            panic!("this is ROM");
            // self.rom[p - ROM_RANGE.start] = data;
        } else if RAM_RANGE.contains(&p) {
            self.ram[p - RAM_RANGE.start] = data;
        } else if PRG_ROM_RANGE.contains(&p) {
            panic!("this is ROM");
            // self.prg_rom[p - PRG_ROM_RANGE.start] = data;
        } else {
            panic!("???")
        }
    }

    pub fn stack<'b>(&'b self, sp: u16) -> DisplayBinary<'b> {
        DisplayBinary(&self.wram[(sp as usize)..0x0200])
    }

    pub fn wram<'b>(&'b self) -> DisplayBinary<'b> {
        DisplayBinary(&self.wram)
    }
}
