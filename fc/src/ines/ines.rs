use super::header::{INESHeader, INES_HEADER_SIZE};
use super::sprite::Sprite;
use std::convert::TryInto;
use std::io::Result;

/// reference: https://wiki.nesdev.org/w/index.php/INES
/// An iNES file consists of the following sections, in order:
/// Header (16 bytes)
/// Trainer, if present (0 or 512 bytes)
/// PRG ROM data (16384 * x bytes)
/// CHR ROM data, if present (8192 * y bytes)
/// PlayChoice INST-ROM, if present (0 or 8192 bytes)
/// PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images for details)
/// Some ROM-Images additionally contain a 128-byte (or sometimes 127-byte) title at the end of the file.
#[derive(Eq, PartialEq, Debug)]
pub struct INES {
    pub header: INESHeader,
    pub trainer: Vec<u8>,
    pub program_rom_data: Vec<u8>,
    pub character_rom_data: Vec<u8>,
}

const INES_PROGRAM_UNIT_SIZE: usize = 0x4000;
const INES_CHARACTER_UNIT_SIZE: usize = 0x2000;

pub fn parser(buf: &mut Vec<u8>) -> Result<INES> {
    let header_buf: &[u8; INES_HEADER_SIZE] = &buf[0..INES_HEADER_SIZE].try_into().unwrap();
    let h = INESHeader::parser(header_buf)?;

    let program_data_size_offset = INES_HEADER_SIZE;
    let program_data_size = INES_PROGRAM_UNIT_SIZE * h.program_rom_size as usize;
    let character_data_size_offset = INES_HEADER_SIZE + program_data_size as usize;
    let character_data_size = INES_CHARACTER_UNIT_SIZE * h.character_rom_size as usize;

    let program_rom_buff = &buf[program_data_size_offset..character_data_size_offset];
    let character_rom_buff =
        &buf[character_data_size_offset..(character_data_size_offset + character_data_size)];

    Ok(INES {
        header: h,
        trainer: vec![],
        program_rom_data: program_rom_buff.to_vec(),
        character_rom_data: character_rom_buff.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;

    #[test]
    fn it_parser() {
        let mut contents =
            io::read_to_binary("../docs/demo/sample1.nes").expect("Don't warry, it is debug");

        println!("{:?}", parser(&mut contents));
    }
}
