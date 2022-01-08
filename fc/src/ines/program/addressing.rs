use super::operand::Operand;
use crate::binary;
use std::convert::TryFrom;

pub trait Addresing {
    fn accumulator(&self, addr: usize) -> Operand;
    fn immediate(&self, addr: usize) -> Operand;
    fn absolute(&self, addr: usize) -> Operand;
    fn zero_page(&self, addr: usize) -> Operand;
    fn indexed_zero_page(&self, addr: usize) -> Operand;
    fn indexed_absolute(&self, addr: usize) -> Operand;
    fn implied(&self, addr: usize) -> Operand;
    fn relative(&self, addr: usize) -> Operand;
    fn indexed_indirect(&self, addr: usize) -> Operand;
    fn indirect_indexed(&self, addr: usize) -> Operand;
    fn absolute_indirect(&self, addr: usize) -> Operand;
    fn none(&self, addr: usize) -> Operand;
}

impl Addresing for &[u8] {
    /// アキュムレータ上での実行を意味します。
    fn accumulator(&self, addr: usize) -> Operand {
        Operand::None
    }
    /// 2番目のバイトをデータそのものとして使用します。
    fn immediate(&self, addr: usize) -> Operand {
        Operand::Data(self[addr + 1])
    }

    /// 2番目のバイトを下位アドレス、
    /// 3番目のバイトを上位アドレスとして実効アドレスとします。
    fn absolute(&self, addr: usize) -> Operand {
        let lower = self[addr + 1];
        let upper = self[addr + 2];
        Operand::Addr(binary::u8u8_to_u16(upper, lower))
    }

    ///  上位アドレスとして$00、
    ///  下位アドレスとして2番目のバイトを使用し実効アドレスとします。
    fn zero_page(&self, addr: usize) -> Operand {
        Operand::Addr(binary::u8u8_to_u16(0x0000, self[addr + 1]))
    }

    /// 上位アドレスとして$00、
    /// 下位アドレスとして2番目のバイトに
    /// インデックスレジスタ（X,Y）を加算（8） した値を実効アドレスとします。
    /// インデックスレジスタX、Yそれぞれについて、
    /// Zero Page, X、Zero Page, Y のアドレッシングモードがあります。
    fn indexed_zero_page(&self, addr: usize) -> Operand {
        self.zero_page(addr)
    }

    /// 2番目のバイトを下位アドレス、3番目のバイトを上位アドレスとして、
    /// このアドレスにインデックスレジスタ（X,Y）を加算（16）したものを実効アドレスとします。
    /// インデックスレジスタX、Yそれぞれ、Absolute, X、Absolute, Y のアドレッシングモードがあります。
    fn indexed_absolute(&self, addr: usize) -> Operand {
        let lower = self[addr + 1];
        let upper = self[addr + 2];
        Operand::Addr(binary::u8u8_to_u16(upper, lower))
    }

    /// レジスタ類を操作する命令で使用され、アドレス指定はありません。
    fn implied(&self, addr: usize) -> Operand {
        Operand::None
    }

    /// 条件分岐命令で使用されます。
    /// 次の命令を示すプログラムカウンタに2番目のバイトを加算（符号拡張）した値を実効アドレスとします。
    /// オフセットとして、-128（$80）～+127（$7F）を指定できます。
    fn relative(&self, addr: usize) -> Operand {
        let v = self[addr + 1] as u16;
        let next_program = addr as u16 + 2;
        Operand::Addr(if v >= 0b1000_0000 {
            0x8000 + next_program + v - 256
        } else {
            0x8000 + next_program + v
        })
    }

    /// 上位アドレスを$00とし、
    /// また2番目のバイトにインデックスレジスタXを加算（8）した値を下位アドレスとします。
    /// このアドレスに格納されている値を実効アドレスの下位バイト、
    /// そしてその次のアドレスに格納されている値を実効アドレスの上位バイトとします。
    /// このインクリメントにおいてキャリーは無視します。
    fn indexed_indirect(&self, addr: usize) -> Operand {
        Operand::Addr(binary::u8u8_to_u16(0x0000, self[addr + 1]))
    }

    /// まず上位アドレスを$00とし、下位アドレスとして2番目のバイトを使用します。
    /// このアドレスに格納されている値を次の上位アドレス、
    /// その次のアドレスに格納されている値を次の下位アドレスとします。
    /// このときのインクリメントにおけるキャリーは無視します。
    /// 得られたアドレスにインデックスレジスタYを加算（16）したものを実効アドレスとします。
    fn indirect_indexed(&self, addr: usize) -> Operand {
        Operand::Addr(binary::u8u8_to_u16(0x0000, self[addr + 1]))
    }

    /// 2、3番目のバイトで示されるアドレスに格納されている値を実効アドレスの下位バイト、
    /// その次のアドレスに格納されている値を実効アドレスの上位バイトとします。
    /// このインクリメントで下位バイトからのキャリーは無視します。
    fn absolute_indirect(&self, addr: usize) -> Operand {
        let lower = self[addr + 1];
        let upper = self[addr + 2];
        Operand::Addr(binary::u8u8_to_u16(upper, lower))
    }

    fn none(&self, addr: usize) -> Operand {
        Operand::None
    }
}

#[test]
fn ts_accumulator() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.accumulator(0), Operand::None);
}
#[test]
fn ts_immediate() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.immediate(0), Operand::Data(1));
}
#[test]
fn ts_absolute() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.absolute(0), Operand::Addr(0x0201));
}
#[test]
fn ts_zero_page() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.zero_page(0), Operand::Addr(0x0001));
}
#[test]
fn ts_indexed_zero_page() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.zero_page(0), Operand::Addr(0x0001));
}
#[test]
fn ts_indexed_absolute() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.indexed_absolute(0), Operand::Addr(0x0201));
}
#[test]
fn ts_implied() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.implied(0), Operand::None);
}
#[test]
fn ts_relative() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.relative(0), Operand::Addr(2 + 1));
}
#[test]
fn ts_relative2() {
    let raw = &(vec![0, 0x7f, 2, 3, 4, 5][..]);
    assert_eq!(raw.relative(0), Operand::Addr(2 + 127));
}
#[test]
fn ts_relative3() {
    let raw = &(vec![0, 0xff, 2, 3, 4, 5][..]);
    assert_eq!(raw.relative(0), Operand::Addr(2 - 1));
}

#[test]
fn ts_indexed_indirect() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.indexed_indirect(0), Operand::Addr(1));
}
#[test]
fn ts_indirect_indexed() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.indirect_indexed(0), Operand::Addr(1));
}
#[test]
fn ts_absolute_indirect() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.absolute_indirect(0), Operand::Addr(0x0201));
}
#[test]
fn ts_none() {
    let raw = &(vec![0, 1, 2, 3, 4, 5][..]);
    assert_eq!(raw.none(0), Operand::None);
}
