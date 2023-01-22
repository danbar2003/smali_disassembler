#![allow(dead_code)]

use crate::reader::DexInstructionFormatReader;

#[derive(Debug)]
pub enum DalvikInstruction {
    Nop,

    Move(u8, u8),
    MoveFrom16(u8, u16),
    Move16(u16, u16),

    MoveWide(u8, u8),
    MoveWideFrom16(u8, u16),
    MoveWide16(u16, u16),

    MoveObject(u8, u8),
    MoveObjectFrom16(u8, u16),
    MoveObject16(u16, u16),

    MoveResult(u8),
    MoveResultWide(u8),
    MoveResultObject(u8),

    MoveException(u8),
    ReturnVoid,
    Return(u8),
    ReturnWide(u8),
    ReturnObject(u8),

    CONST4(u8, u8),
}

impl DalvikInstruction {
    pub fn from_raw_bytes(raw_bytes: &[u8]) -> Vec<Self> {
        // final instruction vector to be returned
        let mut instructions = vec![];
        // idk
        let mut reader = DexInstructionFormatReader::new(raw_bytes);

        while let Some((opcode, offset)) = reader.read_byte() {
            instructions.push(match opcode {
                0x0 => Self::Nop,
                0x1 => {
                    let (src, dst) = reader.r_12x().unwrap();
                    Self::Move(src, dst)
                }
                _ => unreachable!(),
            });
        }

        instructions
    }
}
