use opcode::Opcode;
use register::Register;
use std::u8;

pub struct Emu {
    registers: [u8; 16],
    i: u16,
    pc: u16,          // Program counter
    sp: usize,        // Stack pointer
    stack: [u16; 16], // Stack
    pub display: Vec<u8>,
}

impl Emu {
    pub fn new() -> Emu {
        Emu {
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            display: vec![0; 64 * 32],
        }
    }

    pub fn execute(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Cls => self.cls(),
            Opcode::Ret => self.ret(),
            Opcode::Sys(_) => unimplemented!(),
            Opcode::Jp(addr) => self.jp(*addr),
            Opcode::Call(addr) => self.call(*addr),
            Opcode::SeVal(reg, byte) => self.se_val(*reg, *byte),
            Opcode::SneVal(reg, byte) => self.sne_val(*reg, *byte),
            Opcode::SeReg(reg_x, reg_y) => self.se_reg(*reg_x, *reg_y),
            Opcode::LdVal(reg, byte) => self.ld_val(*reg, *byte),
            Opcode::AddVal(reg, byte) => self.add_val(*reg, *byte),
            Opcode::LdReg(reg_x, reg_y) => self.ld_reg(*reg_x, *reg_y),
            Opcode::Or(reg_x, reg_y) => self.or(*reg_x, *reg_y),
            Opcode::And(reg_x, reg_y) => self.and(*reg_x, *reg_y),
            Opcode::Xor(reg_x, reg_y) => self.xor(*reg_x, *reg_y),
            Opcode::AddReg(reg_x, reg_y) => self.add_reg(*reg_x, *reg_y),
            Opcode::Sub(reg_x, reg_y) => self.sub(*reg_x, *reg_y),
            Opcode::Shr(reg_x) => self.shr(*reg_x),
            Opcode::Subn(reg_x, reg_y) => self.subn(*reg_x, *reg_y),
            _ => unreachable!(),
        }
    }

    fn cls(&mut self) {
        self.display = vec![0; 64 * 32];
    }

    fn ret(&mut self) {
        self.pc = self.stack[self.sp - 1];
        self.sp -= 1;
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn se_val(&mut self, reg: Register, byte: u8) {
        if self.registers[reg.to_usize()] == byte {
            self.pc += 2;
        }
    }

    fn sne_val(&mut self, reg: Register, byte: u8) {
        if self.registers[reg.to_usize()] != byte {
            self.pc += 2;
        }
    }

    fn se_reg(&mut self, reg_x: Register, reg_y: Register) {
        if self.registers[reg_x.to_usize()] == self.registers[reg_y.to_usize()] {
            self.pc += 2;
        }
    }

    fn ld_val(&mut self, reg: Register, byte: u8) {
        self.registers[reg.to_usize()] = byte;
    }

    fn add_val(&mut self, reg: Register, byte: u8) {
        self.registers[reg.to_usize()] += byte;
    }

    fn ld_reg(&mut self, reg_x: Register, reg_y: Register) {
        self.registers[reg_x.to_usize()] = self.registers[reg_y.to_usize()];
    }

    fn or(&mut self, reg_x: Register, reg_y: Register) {
        self.registers[reg_x.to_usize()] |= self.registers[reg_y.to_usize()];
    }

    fn and(&mut self, reg_x: Register, reg_y: Register) {
        self.registers[reg_x.to_usize()] &= self.registers[reg_y.to_usize()];
    }

    fn xor(&mut self, reg_x: Register, reg_y: Register) {
        self.registers[reg_x.to_usize()] ^= self.registers[reg_y.to_usize()];
    }

    fn add_reg(&mut self, reg_x: Register, reg_y: Register) {
        let vx = self.registers[reg_x.to_usize()] as u16;
        let vy = self.registers[reg_y.to_usize()] as u16;
        self.registers[Register::Vf.to_usize()] = if vx + vy > (u8::MAX as u16) { 1 } else { 0 };
        // truncate
        self.registers[reg_x.to_usize()] = (vx + vy) as u8;
    }

    fn sub(&mut self, reg_x: Register, reg_y: Register) {
        let vx = self.registers[reg_x.to_usize()];
        let vy = self.registers[reg_y.to_usize()];
        self.registers[Register::Vf.to_usize()] = if vx > vy { 1 } else { 0 };
        self.registers[reg_x.to_usize()] = vx.overflowing_sub(vy).0;
    }

    fn shr(&mut self, reg: Register) {
        self.registers[reg.to_usize()] >>= 1;
    }

    fn subn(&mut self, reg_x: Register, reg_y: Register) {
        let vx = self.registers[reg_x.to_usize()];
        let vy = self.registers[reg_y.to_usize()];
        self.registers[Register::Vf.to_usize()] = if vy > vx { 1 } else { 0 };
        self.registers[reg_x.to_usize()] = vx.overflowing_sub(vy).0;
    }
}
