mod opcodes;
mod reader;

use opcodes::DalvikInstruction;
use reader::DexInstructionFormatReader;

/// all dalvik opcodes
pub const NOP_OP: u8 = 0x0;
pub const MOV_OP: u8 = 0x1;
pub const MOV_FROM16_OP: u8 = 0x2;
pub const MOV16_OP: u8 = 0x3;
pub const MOV_WIDE_OP: u8 = 0x4;
pub const MOV_WIDE_FROM16: u8 = 0x5;
pub const MOV_WIDE16_OP: u8 = 0x6;
pub const MOV_OBJECT_OP: u8 = 0x7;
pub const MOV_OBJECT_FROM16_OP: u8 = 0x8;
pub const MOV_OBJECT16_OP: u8 = 0x9;
pub const MOV_RESULT_OP: u8 = 0xa;
pub const MOV_RESULT_WIDE_OP: u8 = 0xb;
pub const MOV_RESULT_OBJECT_OP: u8 = 0xc;
pub const MOV_EXCEPTION_OP: u8 = 0xd;
pub const RETURN_VOID_OP: u8 = 0xe;
pub const RETURN_OP: u8 = 0xf;
pub const RETURN_WIDE_OP: u8 = 0x10;
pub const RETURN_OBJECT_OP: u8 = 0x11;

pub const CONST4_OP: u8 = 0x12;
pub const CONST16_OP: u8 = 0x13;
pub const CONST_OP: u8 = 0x14;
pub const CONST_HIGH16_OP: u8 = 0x15;

pub const CONST_WIDE16_OP: u8 = 0x16;
pub const CONST_WIDE32_OP: u8 = 0x17;
pub const CONST_WIDE_OP: u8 = 0x18;

// pub const CONST_: u8 = 0x12;
// pub const CONST_HIGH16: u8 = 0x12;
// pub const CONST_HIGH16: u8 = 0x12;

pub struct SmaliDeocder<'a> {
    reader: DexInstructionFormatReader<'a>,
}

#[allow(dead_code)]
impl<'a> SmaliDeocder<'a> {
    fn new(stream: &'a [u8]) -> Self {
        Self {
            reader: DexInstructionFormatReader::new(stream),
        }
    }

    /// return the next instruction and its offset from the beginning of the function
    /// if there aren't any other instructions None is returned.
    fn decode_instruction(&mut self) -> Option<(DalvikInstruction, usize)> {
        if let Some((opcode, offset)) = self.reader.read_byte() {
            let instruction = match opcode {
                NOP_OP => DalvikInstruction::Nop,

                MOV_OP => {
                    let (dst, src) = self.reader.r_12x()?;
                    DalvikInstruction::Move(dst, src)
                }

                MOV_FROM16_OP => {
                    let (dst, src) = self.reader.r_22x()?;
                    DalvikInstruction::MoveFrom16(dst, src)
                }

                MOV16_OP => {
                    let (dst, src) = self.reader.r_32x()?;
                    DalvikInstruction::Move16(dst, src)
                }

                MOV_WIDE_OP => {
                    let (dst, src) = self.reader.r_12x()?;
                    DalvikInstruction::MoveWide(dst, src)
                }

                MOV_WIDE_FROM16 => {
                    let (dst, src) = self.reader.r_22x()?;
                    DalvikInstruction::MoveWideFrom16(dst, src)
                }

                MOV_WIDE16_OP => {
                    let (dst, src) = self.reader.r_32x()?;
                    DalvikInstruction::MoveWide16(dst, src)
                }

                MOV_OBJECT_OP => {
                    let (dst, src) = self.reader.r_12x()?;
                    DalvikInstruction::MoveObject(dst, src)
                }

                MOV_OBJECT_FROM16_OP => {
                    let (dst, src) = self.reader.r_22x()?;
                    DalvikInstruction::MoveWideFrom16(dst, src)
                }

                MOV_OBJECT16_OP => {
                    let (dst, src) = self.reader.r_32x()?;
                    DalvikInstruction::MoveWide16(dst, src)
                }

                MOV_RESULT_OP => {
                    let dst = self.reader.r_11x()?;
                    DalvikInstruction::MoveResult(dst)
                }

                MOV_RESULT_WIDE_OP => {
                    let dst = self.reader.r_11x()?;
                    DalvikInstruction::MoveResultWide(dst)
                }

                MOV_RESULT_OBJECT_OP => {
                    let dst = self.reader.r_11x()?;
                    DalvikInstruction::MoveResultObject(dst)
                }

                MOV_EXCEPTION_OP => {
                    let dst = self.reader.r_11x()?;
                    DalvikInstruction::MoveException(dst)
                }

                RETURN_VOID_OP => {
                    self.reader.r_10x();
                    DalvikInstruction::ReturnVoid
                }

                RETURN_OP => {
                    let reg = self.reader.r_11x()?;
                    DalvikInstruction::Return(reg)
                }

                RETURN_WIDE_OP => {
                    let reg = self.reader.r_11x()?;
                    DalvikInstruction::ReturnWide(reg)
                }

                RETURN_OBJECT_OP => {
                    let reg = self.reader.r_11x()?;
                    DalvikInstruction::ReturnObject(reg)
                }

                CONST4_OP => {
                    let (dst, value) = self.reader.r_11n()?;
                    DalvikInstruction::CONST4(dst, value)
                }

                CONST16_OP => {
                    let (dst, value) = self.reader.r_21s()?;
                    todo!()
                }

                _ => unreachable!(),
            };
            Some((instruction, offset))
        } else {
            None
        }
    }

    fn decode_all(&mut self) -> Vec<DalvikInstruction> {
        let mut instructions = vec![];
        instructions
    }
}

#[cfg(test)]
mod tests {}
