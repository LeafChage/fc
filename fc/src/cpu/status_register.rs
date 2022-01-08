const N_FLAG: u8 = 0b1000_0000;
const V_FLAG: u8 = 0b0100_0000;
const R_FLAG: u8 = 0b0010_0000;
const B_FLAG: u8 = 0b0001_0000;
const D_FLAG: u8 = 0b0000_1000;
const I_FLAG: u8 = 0b0000_0100;
const Z_FLAG: u8 = 0b0000_0010;
const C_FLAG: u8 = 0b0000_0001;
const DEFAULT_STATUS: u8 = 0b0011_0100;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StatusRegister(u8);
impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister(DEFAULT_STATUS)
    }
    pub fn n(&self) -> bool {
        let StatusRegister(v) = self;
        v & N_FLAG == N_FLAG
    }
    pub fn v(&self) -> bool {
        let StatusRegister(v) = self;
        v & V_FLAG == V_FLAG
    }
    pub fn r(&self) -> bool {
        let StatusRegister(v) = self;
        v & R_FLAG == R_FLAG
    }
    pub fn b(&self) -> bool {
        let StatusRegister(v) = self;
        v & B_FLAG == B_FLAG
    }
    pub fn d(&self) -> bool {
        let StatusRegister(v) = self;
        v & D_FLAG == D_FLAG
    }
    pub fn i(&self) -> bool {
        let StatusRegister(v) = self;
        v & I_FLAG == I_FLAG
    }
    pub fn z(&self) -> bool {
        let StatusRegister(v) = self;
        v & Z_FLAG == Z_FLAG
    }
    pub fn c(&self) -> bool {
        let StatusRegister(v) = self;
        v & C_FLAG == C_FLAG
    }
    /// N（Negative flag） 演算結果のビット7をストアします。 BIT命令ではメモリ値のビット7をストアします。
    pub fn set_n(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | N_FLAG } else { v & (!N_FLAG) })
    }
    /// V（oVerflow flag） 演算によって$7F-$80をまたぐときにセットし、 そうでないならクリア（0をストア）します。 またBIT命令でメモリ値のビット6をストアし、CLV命令でクリアします。
    pub fn set_v(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | V_FLAG } else { v & (!V_FLAG) })
    }
    /// R (Reserved flag) 常にtrue
    pub fn set_r(&self, _: bool) -> Self {
        panic!("undefined");
    }
    /// B（Break flag） BRK命令による割り込みが発生したときにセットします。 NMIやIRQの場合はクリアします。
    pub fn set_b(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | B_FLAG } else { v & (!B_FLAG) })
    }
    /// D（Decimal flag） オリジナルの6502ではこのフラグをセットすることによって、 演算命令で10進演算が使用されます。NESでは10進演算は削除されているため、 このフラグは無視します。 ただし、SED、CLD命令によって操作は可能です。
    pub fn set_d(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | D_FLAG } else { v & (!D_FLAG) })
    }
    /// I（Interrupt flag） 割り込みが発生するとセットします。 またCLI命令でクリア、SEI命令でセットします。
    pub fn set_i(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | I_FLAG } else { v & (!I_FLAG) })
    }
    /// Z（Zero flag） 演算結果がゼロであった場合にセットし、 そうでない場合はクリアします。
    pub fn set_z(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | Z_FLAG } else { v & (!Z_FLAG) })
    }
    /// C（Carry flag） ADC命令によっビット7からの桁上げが発生したとき、 SBC、CMP、CPX、CPX命令によってビット7からの桁上げが発生しなかったときにセットします。 またASL、ROL命令ではAのビット7を、LSR、ROR命令ではAのビット0をストアします。 CLC命令でクリア、SEC命令でセットします。
    pub fn set_c(&self, on: bool) -> Self {
        let StatusRegister(v) = self;
        Self(if on { v | C_FLAG } else { v & (!C_FLAG) })
    }
}

pub fn flag_n(v: u8) -> bool {
    v & N_FLAG == N_FLAG
}
pub fn flag_z(v: u8) -> bool {
    v == 0
}

impl From<StatusRegister> for u8 {
    fn from(v: StatusRegister) -> Self {
        let StatusRegister(v) = v;
        v
    }
}

impl From<u8> for StatusRegister {
    fn from(v: u8) -> Self {
        StatusRegister(v)
    }
}

impl std::fmt::Display for StatusRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N: {}\nV: {}\nR: {}\nB: {}\nD: {}\nI: {}\nZ: {}\nC: {}",
            self.n(),
            self.v(),
            self.r(),
            self.b(),
            self.d(),
            self.i(),
            self.z(),
            self.c(),
        )
    }
}

#[test]
fn it_n() {
    assert_eq!(StatusRegister(0b1000_0000).n(), true);
    assert_eq!(StatusRegister(0b0000_0000).n(), false);
}
#[test]
fn it_o() {
    assert_eq!(StatusRegister(0b0100_0000).v(), true);
    assert_eq!(StatusRegister(0b0000_0000).v(), false);
}
#[test]
fn it_b() {
    assert_eq!(StatusRegister(0b0001_0000).b(), true);
    assert_eq!(StatusRegister(0b0000_0000).b(), false);
}
#[test]
fn it_d() {
    assert_eq!(StatusRegister(0b0000_1000).d(), true);
    assert_eq!(StatusRegister(0b0000_0000).d(), false);
}
#[test]
fn it_i() {
    assert_eq!(StatusRegister(0b0000_0100).i(), true);
    assert_eq!(StatusRegister(0b0000_0000).i(), false);
}
#[test]
fn it_z() {
    assert_eq!(StatusRegister(0b0000_0010).z(), true);
    assert_eq!(StatusRegister(0b0000_0000).z(), false);
}
#[test]
fn it_c() {
    assert_eq!(StatusRegister(0b0000_0001).c(), true);
    assert_eq!(StatusRegister(0b0000_0000).c(), false);
}

// #[test]
// fn it_flag_n() {
//     assert_eq!(0.flag_n(0b1000_0000), N_FLAG);
//     assert_eq!(0.flag_n(0b0100_0000), 0);
//     assert_eq!(0.flag_n(0b0000_0000), 0);
// }
// #[test]
// fn it_flag_o() {}
// #[test]
// fn it_flag_b() {}
// #[test]
// fn it_flag_d() {}
// #[test]
// fn it_flag_i() {}
// #[test]
// fn it_flag_z() {
//     assert_eq!(0b0000_0000.flag_z(0), Z_FLAG);
//     assert_eq!(Z_FLAG.flag_z(0), Z_FLAG);
//     assert_eq!(0b0000_0000.flag_z(0b0100_0000), 0);
//     assert_eq!(Z_FLAG.flag_z(0b1111_1111), 0);
// }
// #[test]
// fn it_flag_c() {}
