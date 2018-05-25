use std::convert::From;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

impl Register {
    pub fn to_usize(&self) -> usize {
        match self {
            Register::V0 => 0,
            Register::V1 => 1,
            Register::V2 => 2,
            Register::V3 => 3,
            Register::V4 => 4,
            Register::V5 => 5,
            Register::V6 => 6,
            Register::V7 => 7,
            Register::V8 => 8,
            Register::V9 => 9,
            Register::Va => 10,
            Register::Vb => 11,
            Register::Vc => 12,
            Register::Vd => 13,
            Register::Ve => 14,
            Register::Vf => 15,
            _ => unreachable!(),
        }
    }
}
