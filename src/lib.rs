mod opcodes;
mod errors;
mod bytecode_format;

use opcodes::*;
use bytecode_format::DexInstructionFormatReader;

#[derive(Debug)]
pub struct DexInstruction {
    pub inst: DexBytecode,
    pub offset: usize,
}

pub type Result<T> = std::result::Result<T, errors::Error>;

pub struct SmaliDeocder<'a> { 
    reader: DexInstructionFormatReader<'a>
}

#[allow(dead_code)]
impl<'a> SmaliDeocder<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self {
            reader: DexInstructionFormatReader::new(stream),
        }
    }

    /// return the next instruction and its offset from the beginning of the function
    /// if there aren't any other instructions None is returned.
    fn decode_instruction(&mut self) -> Result<DexInstruction> {
        let (opcode, offset) = self.reader.read_byte()?;

        let inst = match opcode {
            NOP_OP => Ok(DexBytecode::Nop),

            op @ (MOV_OP | MOV_WIDE_OP | MOV_OBJECT_OP) => {
                let (dst, src) = self.reader.r_12x()?;
                Ok(DexBytecode::Move(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_FROM16_OP | MOV_WIDE_FROM16_OP | MOV_OBJECT_FROM16_OP) => {
                let (dst, src) = self.reader.r_22x()?;
                Ok(DexBytecode::MoveFrom16(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV16_OP | MOV_WIDE16_OP | MOV_OBJECT16_OP) => {
                let (dst, src) = self.reader.r_32x()?;
                Ok(DexBytecode::Move16(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_RESULT_OP | MOV_RESULT_WIDE_OP | MOV_RESULT_OBJECT_OP | MOV_EXCEPTION_OP) => {
                let reg = self.reader.r_11x()?;
                Ok(DexBytecode::MoveResult(MoveKind::from_opcode(op), reg))
            }

            RETURN_VOID_OP => Ok(DexBytecode::Return(ReturnKind::ReturnVoid, 0)),

            op @ (RETURN_OP | RETURN_WIDE_OP | RETURN_OBJECT_OP) => {
                let reg = self.reader.r_11x()?;
                Ok(DexBytecode::Return(ReturnKind::from_opcode(op), reg))
            }

            CONST4_OP => {
                let (reg, value) = self.reader.r_11n()?;
                Ok(DexBytecode::Const4(reg, value))
            }

            CONST16_OP => {
                let (reg, value) = self.reader.r_21s()?;
                Ok(DexBytecode::Const16(reg, value))
            }

            CONST_OP => {
                let (reg, value) = self.reader.r_31i()?;
                Ok(DexBytecode::Const(reg, value))
            }

            CONST_HIGH16_OP => {
                let (reg, value) = self.reader.r_21h()?;
                Ok(DexBytecode::ConstHigh16(reg, value))
            }

            CONST_WIDE16_OP => {
                let (reg, value) = self.reader.r_21s()?;
                Ok(DexBytecode::ConstWide16(reg, value))
            }

            CONST_WIDE32_OP => {
                let (reg, value) = self.reader.r_31i()?;
                Ok(DexBytecode::ConstWide32(reg, value))
            }

            CONST_WIDE_OP => {
                let (reg, value) = self.reader.r_51()?;
                Ok(DexBytecode::ConstWide(reg, value))
            }

            CONST_WIDE_HIGH16_OP => {
                let (reg, value) = self.reader.r_21h()?;
                Ok(DexBytecode::ConstWideHigh16(reg, value))
            }

            CONST_STRING_OP => {
                let (reg, str_index) = self.reader.r_21c()?;
                Ok(DexBytecode::ConstString(reg, str_index))
            }

            CONST_STRING_JUMBO_OP => {
                let (reg, str_index) = self.reader.r_31c()?;
                Ok(DexBytecode::ConstStringJumbo(reg, str_index))
            }

            CONST_CLASS_OP => {
                let (reg, type_index) = self.reader.r_21c()?;
                Ok(DexBytecode::ConstClass(reg, type_index))
            }

            MONITOR_ENTER_OP => {
                let reg = self.reader.r_11x()?;
                Ok(DexBytecode::MonitorEnter(reg))
            }

            MONITOR_EXIT_OP => {
                let reg = self.reader.r_11x()?;
                Ok(DexBytecode::MonitorExit(reg))
            }

            CHECK_CAST_OP => {
                let (_reg, _type_index): (u8, u16) = self.reader.r_21c()?;
                todo!();
            }

            _ => Err(errors::Error::InvalidOpcode),
        }?;

        Ok(DexInstruction { inst, offset })
    }

    /// decode all instruction
    pub fn decode_all(&mut self) -> Vec<DexInstruction> {
        // make sure we start from the first instruction
        self.reader.reset();

        // vector to hold instructions
        let mut instructions = vec![];

        // decode all instructions
        while let Ok(inst) = self.decode_instruction() {
            instructions.push(inst)
        }

        instructions
    }
}

// #[cfg(test)]
// mod tests {}
