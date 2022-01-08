use std::convert::TryInto;
use std::io::{Cursor, Error, Read, Result};

/// reference: https://wiki.nesdev.org/w/index.php/INES
/// 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
/// 4: Size of PRG ROM in 16 KB units
/// 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
/// 6: Flags 6 - Mapper, mirroring, battery, trainer
/// 7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
/// 8: Flags 8 - PRG-RAM size (rarely used extension)
/// 9: Flags 9 - TV system (rarely used extension)
/// 10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
/// 11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
#[derive(Eq, PartialEq, Debug)]
pub struct INESHeader {
    pub nes: [u8; 4],
    pub program_rom_size: u8,
    pub character_rom_size: u8,
    pub flag6: u8,
    pub flag7: u8,
    pub flag8: u8,
    pub flag9: u8,
    pub flag10: u8,
    pub padding: [u8; 5],
}

pub const INES_HEADER_SIZE: usize = 16;

const NES_BYTE: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // N E S SUB

impl INESHeader {
    pub fn parser(buf: &[u8; INES_HEADER_SIZE]) -> Result<INESHeader> {
        if buf[0..4] != NES_BYTE {
            return Err(Error::from(std::io::ErrorKind::NotFound));
        };

        Ok(INESHeader {
            nes: NES_BYTE,
            program_rom_size: buf[4],
            character_rom_size: buf[5],
            flag6: buf[6],
            flag7: buf[7],
            flag8: buf[8],
            flag9: buf[9],
            flag10: buf[10],
            padding: buf[11..16].try_into().expect("This is maybe always safe."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;

    #[test]
    fn it_parser() {
        let mut contents = Cursor::new(
            io::read_to_binary("../docs/demo/sample1.nes").expect("Don't warry, it is debug"),
        );

        let buf: &mut [u8; INES_HEADER_SIZE] = &mut Default::default();
        contents.read_exact(buf).unwrap();

        println!("{:?}", INESHeader::parser(buf));
    }
}
