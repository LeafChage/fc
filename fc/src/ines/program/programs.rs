use super::program::Program;

#[derive(PartialEq, Eq, Debug)]
pub struct Programs(Vec<Program>);

impl std::fmt::Display for Programs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Programs(ps) = self;
        Ok(for v in ps.iter() {
            write!(f, "{}\n", v)?;
        })
    }
}

impl Programs {
    pub fn parse(raw: &Vec<u8>) -> Self {
        let mut programs = Vec::new();

        let mut point = 0;
        let raw = &raw[..];
        while point < raw.len() {
            let program = Program::parse(&raw, point as u16);
            point += program.orderset.length as usize;
            programs.push(program);
        }

        Programs(programs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;
    use bytes::Buf;

    #[test]
    fn it_buf_size_after_get() {
        let base = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut buf = &base[..];
        let a = buf.get_u8();
        assert_eq!(a, 0);
        assert_eq!(base, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let _ = buf.get_u16();
        assert_eq!(buf, [3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn it_parse_program_rom() {
        let mut contents =
            io::read_to_binary("../docs/demo/sample1.nes").expect("Don't warry, it is debug");

        let program_rom = crate::ines::parser(&mut contents).unwrap().program_rom_data;
        let p = Programs::parse(&program_rom);
        println!("{}", p);
    }
}
