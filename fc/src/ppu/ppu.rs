use super::io_register::IORegister;
use super::memory_map::{MemoryMap, NAME_LENGTH};
use crate::display::Display;
use crate::ines::sprite::Sprite;

/// CPUのMemoryMap上にあるIORegister
/// カセットのキャラクターROM
/// PPUのVRAMと連携して動いている
#[derive(PartialEq, Eq, Debug)]
pub struct PPU<'a> {
    characters: &'a [Sprite],
    memory: MemoryMap,
    frame: usize,
}

impl<'a> PPU<'a> {
    pub fn new(sprites: &'a [Sprite]) -> Self {
        PPU {
            characters: sprites,
            memory: MemoryMap::new(),
            frame: Default::default(),
        }
    }

    /// registerの内容を反映
    pub fn refresh(&mut self, register: &mut IORegister) {
        for (addr, d) in register.PPUDATA.cached.iter() {
            dbg!((*addr, *d));
            self.memory.write(*addr, *d);
        }
        register.PPUDATA.clear();
    }

    /// 描画
    pub fn draw(&mut self) {
        let mut table: [[&Sprite; NAME_LENGTH]; 4] = [[&self.characters[0]; NAME_LENGTH]; 4];
        let mut name_index = 0;
        for names in [
            self.memory.name0,
            self.memory.name1,
            self.memory.name2,
            self.memory.name3,
        ] {
            let mut i = 0;
            for sprites_index in names {
                table[name_index][i] = &self.characters[sprites_index as usize];
                i += 1;
            }
            name_index += 1;
        }
        Display::draw(table[0], &format!("./tmp/{}.png", self.frame));
        self.frame += 1;
    }
}
