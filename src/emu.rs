use opcode::Opcode;
use register::Register;

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
            Opcode::SeReg(reg_a, reg_b) => self.se_reg(*reg_a, *reg_b),
            Opcode::LdVal(reg, byte) => self.ld_val(*reg, *byte),
            Opcode::AddVal(reg, byte) => self.add_val(*reg, *byte),
            Opcode::LdReg(reg_a, reg_b) => self.ld_reg(*reg_a, *reg_b),
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

    fn se_val(&mut self, register: Register, byte: u8) {
        if self.registers[register.to_usize()] == byte {
            self.pc += 2;
        }
    }

    fn sne_val(&mut self, register: Register, byte: u8) {
        if self.registers[register.to_usize()] != byte {
            self.pc += 2;
        }
    }

    fn se_reg(&mut self, reg_a: Register, reg_b: Register) {
        if self.registers[reg_a.to_usize()] == self.registers[reg_b.to_usize()] {
            self.pc += 2;
        }
    }

    fn ld_val(&mut self, reg: Register, byte: u8) {
        self.registers[reg.to_usize()] = byte;
    }

    fn add_val(&mut self, reg: Register, byte: u8) {
        self.registers[reg.to_usize()] += byte;
    }

    fn ld_reg(&mut self, reg_a: Register, reg_b: Register) {
        self.registers[reg_a.to_usize()] = self.registers[reg_b.to_usize()];
    }
}
