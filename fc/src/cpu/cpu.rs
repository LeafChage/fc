use super::memory_map::MemoryMap;
use super::register::Register;
use super::status_register::{flag_n, flag_z, StatusRegister};
use crate::binary;
use crate::ines::program::{Command, Mode, Operand, Program};
use crate::ines::INES;
use crate::ppu::io_register::IORegister;

// const InitializeProgramCounter: usize = 0xFFFC;

#[derive(PartialEq, Eq, Debug)]
struct Cpu {
    pub register: Register,
    pub memory: MemoryMap,
}

impl Cpu {
    pub fn new(ines: INES) -> Self {
        let mut cpu = Cpu {
            register: Register::new(),
            memory: MemoryMap::new(ines.program_rom_data),
        };
        cpu.register.PC = 0x8000;
        cpu
    }

    pub fn run(&mut self) {
        // thread::sleep(time::Duration::from_millis(200));
        let program = self.fetch_program();
        println!("==================================");
        println!("[Program]{}", program);
        let operand = self.update_operand_with_register(&program);
        println!(
            "[Exec]{:?} {} {}",
            program.orderset.cmd, operand, program.orderset.clock,
        );
        self.exec(
            program.orderset.cmd,
            operand,
            program.orderset.clock as usize,
        );

        println!("[CPURegister]\n{}", self.register);
        println!("[PPURegister]\n{}", self.memory.ppu);
        println!("[Stack]\n{}", self.memory.stack(self.register.SP));
        // println!("[WRAM]\n{}", self.memory.wram());
    }

    fn fetch_program(&mut self) -> Program {
        let pc = self.register.PC - 0x8000;
        let program = Program::parse(&self.memory.prg_rom, pc);
        println!(
            "{:#x}, {}",
            self.register.PC,
            binary::DisplayBinary(
                &self.memory.prg_rom[pc as usize..(pc as usize + program.orderset.length as usize)]
            )
        );
        self.register.PC += program.orderset.length as u16;
        program
    }

    fn exec(&mut self, cmd: Command, operand: Operand, clock: usize) {
        match cmd {
            Command::ADC => self.adc(operand),
            Command::SBC => self.sbc(operand),
            Command::AND => self.and(operand),
            Command::ORA => self.ora(operand),
            Command::EOR => self.eor(operand),
            Command::ASL => self.asl(),
            Command::LSR => self.lsr(),
            Command::ROL => self.rol(),
            Command::ROR => self.ror(),
            Command::BCC => self.bcc(operand),
            Command::BCS => self.bcs(operand),
            Command::BEQ => self.beq(operand),
            Command::BNE => self.bne(operand),
            Command::BVC => self.bvc(operand),
            Command::BVS => self.bvs(operand),
            Command::BPL => self.bpl(operand),
            Command::BMI => self.bmi(operand),
            Command::BIT => self.bit(),
            Command::JMP => self.jmp(operand),
            Command::JSR => self.jsr(operand),
            Command::RTS => self.rts(),
            Command::BRK => self.brk(),
            Command::RTI => self.rti(),
            Command::CMP => self.cmp(operand),
            Command::CPX => self.cpx(operand),
            Command::CPY => self.cpy(operand),
            Command::INC => self.inc(operand),
            Command::DEC => self.dec(operand),
            Command::INX => self.inx(),
            Command::DEX => self.dex(),
            Command::INY => self.iny(),
            Command::DEY => self.dey(),
            Command::CLC => self.clc(),
            Command::SEC => self.sec(),
            Command::CLI => self.cli(),
            Command::SEI => self.sei(),
            Command::CLD => self.cld(),
            Command::SED => self.sed(),
            Command::CLV => self.clv(),
            Command::LDA => self.lda(operand),
            Command::LDX => self.ldx(operand),
            Command::LDY => self.ldy(operand),
            Command::STA => self.sta(operand),
            Command::STX => self.stx(operand),
            Command::STY => self.sty(operand),
            Command::TAX => self.tax(),
            Command::TXA => self.txa(),
            Command::TAY => self.tay(),
            Command::TYA => self.tya(),
            Command::TSX => self.tsx(),
            Command::TXS => self.txs(),
            Command::PHA => self.pha(),
            Command::PLA => self.pla(),
            Command::PHP => self.php(),
            Command::PLP => self.plp(),
            Command::NOP => self.nop(),
        }
    }

    fn update_operand_with_register(&mut self, program: &Program) -> Operand {
        match program.orderset.mode {
            Mode::Accumulator => Operand::None,
            Mode::Implied => Operand::None,
            Mode::Immediate => program.operand,
            Mode::ZeroPage => program.operand,
            Mode::ZeroPageX => {
                let (upper, lower) = program.operand.addrs();
                let lower = lower.wrapping_add(self.register.X as u8);
                Operand::Addr(binary::u8u8_to_u16(upper, lower))
            }
            Mode::ZeroPageY => {
                let (upper, lower) = program.operand.addrs();
                let lower = lower.wrapping_add(self.register.Y as u8);
                Operand::Addr(binary::u8u8_to_u16(upper, lower))
            }
            Mode::Absolute => program.operand,
            Mode::AbsoluteX => {
                let addr = program.operand.addr();
                Operand::Addr(addr.wrapping_add(self.register.X as u16))
            }
            Mode::AbsoluteY => {
                let addr = program.operand.addr();
                Operand::Addr(addr.wrapping_add(self.register.Y as u16))
            }
            Mode::Relative => program.operand,
            Mode::Indirect => {
                let addr = program.operand.addr();
                let lower = self.memory.read(addr);
                let upper = self.memory.read(addr + 1);
                Operand::Addr(binary::u8u8_to_u16(upper, lower))
            }
            Mode::IndirectX => {
                let (upper, lower) = program.operand.addrs();
                let addr = binary::u8u8_to_u16(upper, lower.wrapping_add(self.register.X));
                let lower = self.memory.read(addr);
                let upper = self.memory.read(addr + 1);
                Operand::Addr(binary::u8u8_to_u16(upper, lower))
            }
            Mode::IndirectY => {
                let addr = program.operand.addr();
                let upper = self.memory.read(addr);
                let lower = self.memory.read(addr + 1);
                let addr = binary::u8u8_to_u16(upper, lower).wrapping_add(self.register.Y as u16);
                Operand::Addr(addr)
            }
            Mode::NONE => program.operand,
        }
    }

    /// ADC (Add M to A with C)	A + M + C -> A
    /// flags: N V Z C
    fn adc(&mut self, operand: Operand) {
        let m = self.to_data(operand);
        dbg!(m as u16 + self.register.A as u16 + self.register.P.c() as u16);
        unimplemented!();
    }

    /// SBC (Subtract M from A with C)	A - M - not C -> A
    /// flags: N V Z C
    fn sbc(&mut self, operand: Operand) {
        let m = self.to_data(operand);
        dbg!(m as u16 - self.register.A as u16 - !self.register.P.c() as u16);
        unimplemented!();
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BCC (Branch on C clear) Cフラグがクリアされていれば分岐します。
    /// flags: none
    fn bcc(&mut self, operand: Operand) {
        if self.register.P.c() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BCS (Branch on C set) Cフラグがセットされていれば分岐します。
    /// flags: none
    fn bcs(&mut self, operand: Operand) {
        if !self.register.P.c() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BEQ (Branch on Z set (result equal)) Zフラグがセットされていれば分岐します。
    /// flags: none
    fn beq(&mut self, operand: Operand) {
        if self.register.P.z() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BNE (Branch on Z clear (result not equal)) Zフラグがクリアされていれば分岐します。
    /// flags: none
    fn bne(&mut self, operand: Operand) {
        if !self.register.P.z() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BVC (Branch on V clear) Vフラグがクリアされていれば分岐します。
    /// flags: none
    fn bvc(&mut self, operand: Operand) {
        if self.register.P.v() {
            self.register.PC = operand.addr();
        }
    }

    /// BVS (Branch on V set) Vフラグがセットされていれば分岐します。
    /// flags: none
    fn bvs(&mut self, operand: Operand) {
        if !self.register.P.v() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BPL (Branch on N clear (result plus)) Nフラグがクリアされていれば分岐します。
    /// flags: none
    fn bpl(&mut self, operand: Operand) {
        if self.register.P.n() {
            self.register.PC = operand.addr();
        }
    }

    /// 条件が成立した場合、リラティブ・アドレス指定により 分岐先のアドレスをPCへストアします。
    /// 成立しなかった場合、分岐命令の先頭アドレス+2をPCへストアします（単に次の命令アドレス）。
    /// BMI (Branch on N set (result minus)) Nフラグがセットされていれば分岐します。
    /// flags: none
    fn bmi(&mut self, operand: Operand) {
        if !self.register.P.n() {
            self.register.PC = operand.addr();
        }
    }

    fn bit(&mut self) {
        unimplemented!();
    }
    ///JMP (Jump to new location)
    /// ADDR -> PC
    /// flags: none
    fn jmp(&mut self, operand: Operand) {
        self.register.PC = operand.addr();
    }
    ///JSR (Jump to new location saving return address)
    /// ADDR -> PC
    /// サブルーチンへジャンプします。
    /// まずジャンプ先のアドレスをアドレス指定によって取得した後、
    /// PCを上位バイト、下位バイトの順にスタックへプッシュします。
    /// このときのPCはトラップの項にもあるようにJSRの最後のバイトアドレスです。
    /// 最後にジャンプ先へジャンプします。
    /// flags: none
    /// トラップ：
    /// ジャンプサブルーチン命令（JSR）によってスタックに格納する復帰アドレスは、
    /// 次の命令の一つ前のアドレス（JSRの最後のバイト）であり、
    /// リターンサブルーチン命令（RTS）によってインクリメントします。
    fn jsr(&mut self, operand: Operand) {
        // トラップ回避のために-1
        let pc = self.register.PC - 1;
        self.push_stack(binary::upper_only(pc));
        self.push_stack(binary::lower_only(pc));
        self.register.PC = operand.addr();
    }

    /// RTS (Return from Subroutine)
    /// サブルーチンから復帰します。
    /// 復帰アドレスをスタックから、下位バイト、 上位バイトの順に
    /// ポップしたのちインクリメントします。
    /// flags: none
    fn rts(&mut self) {
        let lower = self.pop_stack();
        let upper = self.pop_stack();
        self.register.PC = binary::u8u8_to_u16(upper, lower);
        self.register.PC += 1;
    }
    ///割り込みが確認された時、Iフラグがセットされていれば割り込みは無視します。
    ///Iフラグがクリアされていれば、割り込み動作を開始します。
    ///BRKでは、Bフラグをセットし、PCに1を加算します。
    ///次にPCの上位バイト、下位バイト、ステータスレジスタを順にスタックへ格納します。
    ///次にIフラグをセットし、最後にPCの下位バイトを$FFFEから、上位バイトを$FFFFからフェッチします。
    /// IRQと異なる点はBフラグとPCの扱いのみで、あとは同じです。
    /// BRKではPCに1を加算するため、BRK命令のあるアドレス+2番地がリターンアドレスとなります。
    fn brk(&mut self) {
        if self.register.P.i() {
            // 何もしない
        } else {
            // unimplemented!();
        }
    }
    fn rti(&mut self) {
        unimplemented!();
    }

    /// AND ("AND" M with A)	A and M -> A
    ///  flags: N Z
    fn and(&mut self, operand: Operand) {
        let v = self.register.A & self.to_data(operand);
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// ORA ("OR" M with A)	A or M -> A
    ///  flags: N Z
    fn ora(&mut self, operand: Operand) {
        let v = self.register.A | self.to_data(operand);
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// EOR ("Exclusive-OR" M with A)	A eor M -> A
    ///  flags: N Z
    fn eor(&mut self, operand: Operand) {
        let v = self.register.A ^ self.to_data(operand);
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }

    /// ASL (Arithmetic shift left one bit)
    /// Aを左シフト、ビット0には0
    /// C <- Aのビット7
    fn asl(&mut self) {
        let a = self.register.A;
        self.register.P = self.register.P.set_c((a & 0b1000_0000) == 0b1000_0000);
        self.register.A = a << 1;
    }
    /// LSR (Logical shift right one bit)
    /// Aを右シフト、ビット7にはpc
    /// Aのビット0 -> C
    fn lsr(&mut self) {
        let a = self.register.A;
        self.register.P = self.register.P.set_c((a & 0b0000_0001) == 0b0000_0001);
        self.register.A = a >> 1;
    }
    /// ROL (Rotate left one bit)
    /// Aを左シフト、ビット0にはC
    /// C <- Aのビット7
    fn rol(&mut self) {
        let a = self.register.A;
        let c = self.register.P.c();
        self.register.P = self.register.P.set_c((a & 0b1000_0000) == 0b1000_0000);
        self.register.A = (a << 1) & (if c { 0b0000_0001 } else { 0b0000_0000 });
        unimplemented!("なんかおかしいっぽい");
    }
    /// ROR (Rotate right one bit)
    /// Aを右シフト、ビット7にはC
    /// Aのビット0 -> C
    fn ror(&mut self) {
        let a = self.register.A;
        let c = self.register.P.c();
        self.register.P = self.register.P.set_c((a & 0b0000_0001) == 0b0000_0001);
        self.register.A = (a >> 1) & (if c { 0b1000_0000 } else { 0b0000_0000 });
        unimplemented!("なんかおかしいっぽい");
    }

    /// 演算の結果によって、フラグをセットします。
    /// Cフラグは、正かゼロのときセットし、負のときクリアします。 （単にキャリーをストア）
    /// CMP (Compare M and A)	A - M
    /// flags: N Z C
    fn cmp(&mut self, operand: Operand) {
        let v = self.register.A - self.to_data(operand);
        self.register.P = self
            .register
            .P
            .set_n(flag_n(v))
            .set_n(flag_z(v))
            .set_c(v >= 1);
        unimplemented!("calicurate");
    }
    /// 演算の結果によって、フラグをセットします。
    /// Cフラグは、正かゼロのときセットし、負のときクリアします。 （単にキャリーをストア）
    /// CPX (Compare M and X)	X - M
    /// flags: N Z C
    fn cpx(&mut self, operand: Operand) {
        let v = self.register.X - self.to_data(operand);
        self.register.P = self
            .register
            .P
            .set_n(flag_n(v))
            .set_n(flag_z(v))
            .set_c(v >= 1);
        unimplemented!("calicurate");
    }
    /// 演算の結果によって、フラグをセットします。
    /// Cフラグは、正かゼロのときセットし、負のときクリアします。 （単にキャリーをストア）
    /// CPY (Compare M and Y)	Y - M
    /// flags: N Z C
    fn cpy(&mut self, operand: Operand) {
        let v = self.register.Y - self.to_data(operand);
        self.register.P = self
            .register
            .P
            .set_n(flag_n(v))
            .set_n(flag_z(v))
            .set_c(v >= 1);
        unimplemented!("calicurate");
    }
    /// INC (Increment M by one)	M + 1 -> M
    /// flags: N Z
    fn inc(&mut self, operand: Operand) {
        let v = self.to_data(operand) + 1;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// INX (Increment X by one)	X + 1 -> X
    /// flags: N Z
    fn inx(&mut self) {
        let v = self.register.X + 1;
        self.register.X = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// INY (Increment Y by one)	Y + 1 -> Y
    /// flags: N Z
    fn iny(&mut self) {
        let v = self.register.Y + 1;
        self.register.Y = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// DEC (Decrement M by one)	M - 1 -> M
    /// flags: N Z
    fn dec(&mut self, operand: Operand) {
        let addr = operand.addr();
        let v = self.to_data(operand) - 1;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
        self.memory.write(addr, v);
    }
    /// DEX (Decrement X by one)	X - 1 -> X
    /// flags: N Z
    fn dex(&mut self) {
        let v = self.register.X - 1;
        self.register.X = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }
    /// DEY (Decrement Y by one)	Y - 1 -> Y
    /// flags: N Z
    fn dey(&mut self) {
        let v = self.register.Y - 1;
        self.register.Y = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_n(flag_z(v));
    }

    /// CLC (Clear C flag)	0 -> C
    /// flags: none
    fn clc(&mut self) {
        self.register.P = self.register.P.set_c(false);
    }
    /// SEC (Set C flag)	1 -> C
    /// flags: none
    fn sec(&mut self) {
        self.register.P = self.register.P.set_c(true);
    }
    /// CLI (Clear Interrupt disable)	0 -> I
    /// flags: none
    fn cli(&mut self) {
        self.register.P = self.register.P.set_i(false);
    }
    /// SEI (Set Interrupt disable)	1 -> I
    /// flags: none
    fn sei(&mut self) {
        self.register.P = self.register.P.set_i(true);
    }
    /// CLD (Clear Decimal mode)	0 -> D
    /// flags: none
    fn cld(&mut self) {
        self.register.P = self.register.P.set_d(false);
    }
    /// SED (Set Decimal mode)	1 -> D
    /// flags: none
    fn sed(&mut self) {
        self.register.P = self.register.P.set_d(true);
    }
    /// CLV (Clear V flag)	0 -> V
    /// flags: none
    fn clv(&mut self) {
        self.register.P = self.register.P.set_v(false);
    }

    /// STA (Store A to M)	A -> M
    /// flags: none
    fn sta(&mut self, operand: Operand) {
        let addr = operand.addr();
        self.memory.write(addr, self.register.A);
    }
    /// STX (Store X to M)	X -> M
    /// flags: none
    fn stx(&mut self, operand: Operand) {
        let addr = operand.addr();
        self.memory.write(addr, self.register.X);
    }
    /// STY (Store Y to M)	Y -> M
    /// flags: none
    fn sty(&mut self, operand: Operand) {
        let addr = operand.addr();
        self.memory.write(addr, self.register.Y);
    }

    /// LDA (Load A from M)	M -> A
    /// flags: N Z
    fn lda(&mut self, operand: Operand) {
        let v = self.to_data(operand);
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }
    /// LDX (Load X from M)	M -> X
    /// flags: N Z
    fn ldx(&mut self, operand: Operand) {
        let v = self.to_data(operand);
        self.register.X = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }
    /// LDY (Load Y from M)	M -> Y
    /// flags: N Z
    fn ldy(&mut self, operand: Operand) {
        let v = self.to_data(operand);
        self.register.Y = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TAX (Transfer A to X)	A -> X
    /// flags: N Z
    fn tax(&mut self) {
        let v = self.register.A;
        self.register.X = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TXA (Transfer X to A)	X -> A
    /// flags: N Z
    fn txa(&mut self) {
        let v = self.register.X;
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TAY (Transfer A to Y)	A -> Y
    /// flags: N Z
    fn tay(&mut self) {
        let v = self.register.A;
        self.register.Y = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TYA (Transfer Y to A)	Y -> A
    /// flags: N Z
    fn tya(&mut self) {
        let v = self.register.Y;
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TSX (Transfer S to X)	S -> X
    /// flags: N Z
    fn tsx(&mut self) {
        let v = self.register.SP;
        let v = (v >> 8) as u8;
        self.register.X = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// TXS (Transfer X to S)	X -> S
    /// flags: none
    fn txs(&mut self) {
        self.register.SP = binary::u8u8_to_u16(0x01, self.register.X);
    }

    /// PHA (Push A on stack)	A -> stack
    /// flags: none
    fn pha(&mut self) {
        self.push_stack(self.register.A);
    }

    /// PLA (Pull A from stack)	stack -> A
    /// flags: N Z
    fn pla(&mut self) {
        let v = self.pop_stack();
        self.register.A = v;
        self.register.P = self.register.P.set_n(flag_n(v)).set_z(flag_z(v));
    }

    /// PHP (Push P on stack)	P -> stack
    /// flags: none
    fn php(&mut self) {
        self.push_stack(u8::from(self.register.P));
    }

    /// PLP (Pull P from stack)	stack -> P
    /// flags: all
    /// phpとの対比で考えると
    /// phpで一旦現状のフラグをセーブして
    /// plpでフラグをもとに戻すってこと？
    fn plp(&mut self) {
        let v = self.pop_stack();
        self.register.P = StatusRegister::from(v);
    }

    fn nop(&mut self) {
        // Do Nothing
    }

    fn push_stack(&mut self, v: u8) {
        self.memory.write(self.register.SP, v);
        self.register.SP -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        let v = self.memory.read(self.register.SP);
        self.register.SP += 1;
        v
    }

    /// dataならData、アドレスならそのアドレスのデータを読み込む
    fn to_data(&self, operand: Operand) -> u8 {
        match operand {
            Operand::Data(v) => v,
            Operand::Addr(addr) => self.memory.read(addr),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ines::sprite::Sprite;
    use crate::io;
    use crate::ppu::io_register::IORegister;
    use crate::ppu::PPU;
    use std::{thread, time};

    #[test]
    fn it_cpu_run<'a>() {
        let mut contents =
            io::read_to_binary("../docs/demo/sample1.nes").expect("Don't warry, it is debug");

        let ines = crate::ines::parser(&mut contents).unwrap();
        println!("{:?}", ines);
        let sprites: Vec<Sprite> = Sprite::parse_sprites(&ines.character_rom_data);
        let mut cpu = Cpu::new(ines);
        let mut ppu = PPU::new(&sprites[..]);

        fn game_loop(cpu: &mut Cpu, ppu: &mut PPU) {
            cpu.run();
            ppu.refresh(&mut cpu.memory.ppu);
            ppu.draw();
        }

        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            game_loop(&mut cpu, &mut ppu);
        }
    }
}
