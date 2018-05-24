use std::convert::From;

#[derive(PartialEq, Eq, Debug)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    Va,
    Vb,
    Vc,
    Vd,
    Ve,
    Vf,
    I,
    Delay,
    Sound,
    Pc,
    Sp,
}

impl From<u8> for Register {
    fn from(i: u8) -> Register {
        match i {
            0x0 => Register::V0,
            0x1 => Register::V1,
            0x2 => Register::V2,
            0x3 => Register::V3,
            0x4 => Register::V4,
            0x5 => Register::V5,
            0x6 => Register::V6,
            0x7 => Register::V7,
            0x8 => Register::V8,
            0x9 => Register::V9,
            0xa => Register::Va,
            0xb => Register::Vb,
            0xc => Register::Vc,
            0xd => Register::Vd,
            0xe => Register::Ve,
            0xf => Register::Vf,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Opcode {
    Sys(u16),
    Cls,
    Ret,
    Jp(u16),
    Call(u16),
    SeVal(Register, u8),
    Sne(Register, u8),
    SeReg(Register, Register),
}

impl Opcode {
    pub fn decode(hex: u16) -> Option<Opcode> {
        match hex >> 12 {
            0 if hex == 0x00e0 => Some(Opcode::Cls),
            0 if hex == 0x00ee => Some(Opcode::Ret),
            0 => Some(Opcode::Sys(hex & 0x0fff)),
            1 => Some(Opcode::Jp(hex & 0x0fff)),
            2 => Some(Opcode::Call(hex & 0x0fff)),
            3 => Some(Opcode::SeVal(
                Register::from(((hex & 0x0f00) >> 8) as u8),
                (hex & 0x00ff) as u8,
            )),
            4 => Some(Opcode::Sne(
                Register::from(((hex & 0x0f00) >> 8) as u8),
                (hex & 0x00ff) as u8,
            )),
            5 => Some(Opcode::SeReg(
                Register::from(((hex & 0x0f00) >> 8) as u8),
                Register::from(((hex & 0x00f0) >> 4) as u8),
            )),
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
            Some(Opcode::Sne(Register::V1, 0x12))
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
