use register::Register;

#[derive(PartialEq, Eq, Debug)]
pub enum Opcode {
    Sys(u16),
    Cls,
    Ret,
    Jp(u16),
    Call(u16),
    SeVal(Register, u8),
    SneVal(Register, u8),
    SeReg(Register, Register),
    LdVal(Register, u8),
    AddVal(Register, u8),
    LdReg(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddReg(Register, Register),
    Sub(Register, Register),
    Shr(Register),
    Subn(Register, Register),
    Shl(Register),
    SneReg(Register, Register),
    LdI(u16),
    JpV0(u16),
    Rnd(Register, u8),
    Drw(Register, Register, u8),
    Skp(Register),
    SkNp(Register),
    LdVxDt(Register),
    LdK(Register),
    LdDtVx(Register),
    LdSt(Register),
    AddI(Register),
    LdF(Register),
    LdB(Register),
    StoreRegs(Register),
    ReadRegs(Register),
}

impl Opcode {
    pub fn decode(hex: u16) -> Option<Opcode> {
        let nnn = hex & 0x0fff;
        let n = (hex & 0x000f) as u8;
        let kk = (hex & 0x00ff) as u8;
        let x = ((hex & 0x0f00) >> 8) as u8;
        let y = ((hex & 0x00f0) >> 4) as u8;
        match (
            hex >> 12,
            hex & 0x0f00 >> 8,
            hex & 0x00f0 >> 4,
            hex & 0x000f,
        ) {
            (0x0, 0x0, 0xe, 0x0) => Some(Opcode::Cls),
            (0x0, 0x0, 0xe, 0xe) => Some(Opcode::Ret),
            (0x0, ..) => Some(Opcode::Sys(nnn)),
            (0x1, ..) => Some(Opcode::Jp(nnn)),
            (0x2, ..) => Some(Opcode::Call(nnn)),
            (0x3, ..) => Some(Opcode::SeVal(Register::from(x), kk)),
            (0x4, ..) => Some(Opcode::SneVal(Register::from(x), kk)),
            (0x5, _, _, 0x0) => Some(Opcode::SeReg(Register::from(x), Register::from(y))),
            (0x6, ..) => Some(Opcode::LdVal(Register::from(x), kk)),
            (0x7, ..) => Some(Opcode::AddVal(Register::from(x), kk)),
            (0x8, _, _, 0x0) => Some(Opcode::LdReg(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x1) => Some(Opcode::Or(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x2) => Some(Opcode::And(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x3) => Some(Opcode::Xor(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x4) => Some(Opcode::AddReg(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x5) => Some(Opcode::Sub(Register::from(x), Register::from(y))),
            (0x8, _, _, 0x6) => Some(Opcode::Shr(Register::from(x))),
            (0x8, _, _, 0x7) => Some(Opcode::Subn(Register::from(x), Register::from(y))),
            (0x8, _, _, 0xe) => Some(Opcode::Shl(Register::from(x))),
            (0x9, _, _, 0x0) => Some(Opcode::SneReg(Register::from(x), Register::from(y))),
            (0xa, ..) => Some(Opcode::LdI(hex & 0x0fff)),
            (0xb, ..) => Some(Opcode::JpV0(hex & 0x0fff)),
            (0xc, ..) => Some(Opcode::Rnd(Register::from(x), kk)),
            (0xd, ..) => Some(Opcode::Drw(Register::from(x), Register::from(y), n)),
            (0xe, _, 0x9, 0xe) => Some(Opcode::Skp(Register::from(x))),
            (0xe, _, 0xa, 0x1) => Some(Opcode::SkNp(Register::from(x))),
            (0xf, _, 0x0, 0x7) => Some(Opcode::LdVxDt(Register::from(x))),
            (0xf, _, 0x0, 0xa) => Some(Opcode::LdK(Register::from(x))),
            (0xf, _, 0x1, 0x5) => Some(Opcode::LdDtVx(Register::from(x))),
            (0xf, _, 0x1, 0x8) => Some(Opcode::LdSt(Register::from(x))),
            (0xf, _, 0x1, 0xe) => Some(Opcode::AddI(Register::from(x))),
            (0xf, _, 0x2, 0x9) => Some(Opcode::LdF(Register::from(x))),
            (0xf, _, 0x3, 0x3) => Some(Opcode::LdB(Register::from(x))),
            (0xf, _, 0x5, 0x5) => Some(Opcode::StoreRegs(Register::from(x))),
            (0xf, _, 0x6, 0x5) => Some(Opcode::ReadRegs(Register::from(x))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_se_val() {
        assert_eq!(
            Opcode::decode(0x3012),
            Some(Opcode::SeVal(Register::V0, 0x12))
        )
    }

    #[test]
    fn test_decode_sne() {
        assert_eq!(
            Opcode::decode(0x4112),
            Some(Opcode::SneVal(Register::V1, 0x12))
        )
    }

    #[test]
    fn test_decode_se_reg() {
        assert_eq!(
            Opcode::decode(0x5230),
            Some(Opcode::SeReg(Register::V2, Register::V3))
        )
    }
}
