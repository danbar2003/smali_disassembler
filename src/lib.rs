mod bytecode_format;
mod errors;
mod opcodes;

use bytecode_format::DexInstructionFormatReader;
use opcodes::*;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug)]
pub struct DalvikInstruction {
    pub inst: DexBytecode,
    pub offset: usize,
}

impl DalvikInstruction {
    /// return the next instruction and its offset from the beginning of the function
    /// if there aren't any other instructions None is returned.
    fn decode_instruction<'a>(reader: &'a mut DexInstructionFormatReader) -> Result<Self> {
        let (opcode, offset) = reader.read_byte()?;

        let inst = match opcode {
            NOP_OP => Ok(DexBytecode::Nop),

            op @ (MOV_OP | MOV_WIDE_OP | MOV_OBJECT_OP) => {
                let (dst, src) = reader.r_12x()?;
                Ok(DexBytecode::Move(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_FROM16_OP | MOV_WIDE_FROM16_OP | MOV_OBJECT_FROM16_OP) => {
                let (dst, src) = reader.r_22x()?;
                Ok(DexBytecode::MoveFrom16(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV16_OP | MOV_WIDE16_OP | MOV_OBJECT16_OP) => {
                let (dst, src) = reader.r_32x()?;
                Ok(DexBytecode::Move16(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_RESULT_OP | MOV_RESULT_WIDE_OP | MOV_RESULT_OBJECT_OP | MOV_EXCEPTION_OP) => {
                let reg = reader.r_11x()?;
                Ok(DexBytecode::MoveResult(MoveKind::from_opcode(op), reg))
            }

            RETURN_VOID_OP => Ok(DexBytecode::Return(ReturnKind::ReturnVoid, 0)),

            op @ (RETURN_OP | RETURN_WIDE_OP | RETURN_OBJECT_OP) => {
                let reg = reader.r_11x()?;
                Ok(DexBytecode::Return(ReturnKind::from_opcode(op), reg))
            }

            CONST4_OP => {
                let (reg, value) = reader.r_11n()?;
                Ok(DexBytecode::Const4(reg, value))
            }

            CONST16_OP => {
                let (reg, value) = reader.r_21s()?;
                Ok(DexBytecode::Const16(reg, value))
            }

            CONST_OP => {
                let (reg, value) = reader.r_31i()?;
                Ok(DexBytecode::Const(reg, value))
            }

            CONST_HIGH16_OP => {
                let (reg, value) = reader.r_21h()?;
                Ok(DexBytecode::ConstHigh16(reg, value))
            }

            CONST_WIDE16_OP => {
                let (reg, value) = reader.r_21s()?;
                Ok(DexBytecode::ConstWide16(reg, value))
            }

            CONST_WIDE32_OP => {
                let (reg, value) = reader.r_31i()?;
                Ok(DexBytecode::ConstWide32(reg, value))
            }

            CONST_WIDE_OP => {
                let (reg, value) = reader.r_51()?;
                Ok(DexBytecode::ConstWide(reg, value))
            }

            CONST_WIDE_HIGH16_OP => {
                let (reg, value) = reader.r_21h()?;
                Ok(DexBytecode::ConstWideHigh16(reg, value))
            }

            CONST_STRING_OP => {
                let (reg, str_index) = reader.r_21c()?;
                Ok(DexBytecode::ConstString(reg, str_index))
            }

            CONST_STRING_JUMBO_OP => {
                let (reg, str_index) = reader.r_31c()?;
                Ok(DexBytecode::ConstStringJumbo(reg, str_index))
            }

            CONST_CLASS_OP => {
                let (reg, type_index) = reader.r_21c()?;
                Ok(DexBytecode::ConstClass(reg, type_index))
            }

            MONITOR_ENTER_OP => {
                let reg = reader.r_11x()?;
                Ok(DexBytecode::MonitorEnter(reg))
            }

            MONITOR_EXIT_OP => {
                let reg = reader.r_11x()?;
                Ok(DexBytecode::MonitorExit(reg))
            }

            CHECK_CAST_OP => {
                let (_reg, _type_index): (u8, u16) = reader.r_21c()?;
                todo!();
            }

            INSTANCE_OF_OP => {
                let (dst, src, type_index) = reader.r_22c()?;
                Ok(DexBytecode::InstanceOf(dst, src, type_index))
            }

            ARRAY_LENGTH_OP => {
                let (dst, src) = reader.r_12x()?;
                Ok(DexBytecode::ArrayLength(dst, src))
            }

            NEW_INSTANCE_OP => {
                let (dst, type_index) = reader.r_21c()?;
                Ok(DexBytecode::NewInstance(dst, type_index))
            }

            NEW_ARRAY_OP => {
                let (dst, src, type_index) = reader.r_22c()?;
                Ok(DexBytecode::NewArray(dst, src, type_index))
            }

            FILLED_NEW_ARRAY_OP => {
                todo!()
            }

            _ => Err(errors::Error::InvalidOpcode),
        }?;

        Ok(DalvikInstruction { inst, offset })
    }
}

pub struct SmaliDeocder<'a> {
    stream: &'a [u8],
}

#[allow(dead_code)]
impl<'a> SmaliDeocder<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self { stream }
    }

    /// decode all instruction
    pub fn decode_all(&self) -> Vec<DalvikInstruction> {
        // vector to hold instructions
        let mut instructions = vec![];

        let mut new_reader = DexInstructionFormatReader::new(&self.stream);
        while let Ok(inst) = DalvikInstruction::decode_instruction(&mut new_reader) {
            instructions.push(inst)
        }

        // decode all instructions
        instructions
    }
}

// #[cfg(test)]
// mod tests {}
