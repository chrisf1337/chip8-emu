use opcode::Opcode;

pub struct Emu {
    pc: u16,          // Program counter
    sp: usize,        // Stack pointer
    stack: [u16; 16], // Stack
    pub display: Vec<u8>,
}

impl Emu {
    pub fn new() -> Emu {
        Emu {
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
}
