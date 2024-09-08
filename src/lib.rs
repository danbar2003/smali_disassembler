mod bytecode_format;
mod errors;
mod opcodes;

use bytecode_format::DexInstructionFormatReader;
use opcodes::*;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug)]
pub struct DalvikInstruction {
    pub inst: DalvikBytecode,
    pub offset: usize,
}

impl DalvikInstruction {
    /// return the next instruction and its offset from the beginning of the function
    /// if there aren't any other instructions None is returned.
    fn decode_instruction<'a>(reader: &'a mut DexInstructionFormatReader) -> Result<Self> {
        let (opcode, offset) = reader.read_byte()?;

        let dalvik_bytecode = match opcode {
            NOP_OP => Ok(DalvikBytecode::Nop),

            op @ (MOV_OP | MOV_WIDE_OP | MOV_OBJECT_OP) => {
                let (dst, src) = reader.r_12x()?;
                Ok(DalvikBytecode::Move(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_FROM16_OP | MOV_WIDE_FROM16_OP | MOV_OBJECT_FROM16_OP) => {
                let (dst, src) = reader.r_22x()?;
                Ok(DalvikBytecode::MoveFrom16(
                    MoveKind::from_opcode(op),
                    dst,
                    src,
                ))
            }

            op @ (MOV16_OP | MOV_WIDE16_OP | MOV_OBJECT16_OP) => {
                let (dst, src) = reader.r_32x()?;
                Ok(DalvikBytecode::Move16(MoveKind::from_opcode(op), dst, src))
            }

            op @ (MOV_RESULT_OP | MOV_RESULT_WIDE_OP | MOV_RESULT_OBJECT_OP | MOV_EXCEPTION_OP) => {
                let reg = reader.r_11x()?;
                Ok(DalvikBytecode::MoveResult(MoveKind::from_opcode(op), reg))
            }

            RETURN_VOID_OP => Ok(DalvikBytecode::Return(ReturnKind::ReturnVoid, 0)),

            op @ (RETURN_OP | RETURN_WIDE_OP | RETURN_OBJECT_OP) => {
                let reg = reader.r_11x()?;
                Ok(DalvikBytecode::Return(ReturnKind::from_opcode(op), reg))
            }

            CONST4_OP => {
                let (reg, value) = reader.r_11n()?;
                Ok(DalvikBytecode::Const4(reg, value))
            }

            CONST16_OP => {
                let (reg, value) = reader.r_21s()?;
                Ok(DalvikBytecode::Const16(reg, value))
            }

            CONST_OP => {
                let (reg, value) = reader.r_31i()?;
                Ok(DalvikBytecode::Const(reg, value))
            }

            CONST_HIGH16_OP => {
                let (reg, value) = reader.r_21h()?;
                Ok(DalvikBytecode::ConstHigh16(reg, value))
            }

            CONST_WIDE16_OP => {
                let (reg, value) = reader.r_21s()?;
                Ok(DalvikBytecode::ConstWide16(reg, value))
            }

            CONST_WIDE32_OP => {
                let (reg, value) = reader.r_31i()?;
                Ok(DalvikBytecode::ConstWide32(reg, value))
            }

            CONST_WIDE_OP => {
                let (reg, value) = reader.r_51()?;
                Ok(DalvikBytecode::ConstWide(reg, value))
            }

            CONST_WIDE_HIGH16_OP => {
                let (reg, value) = reader.r_21h()?;
                Ok(DalvikBytecode::ConstWideHigh16(reg, value))
            }

            CONST_STRING_OP => {
                let (reg, str_index) = reader.r_21c()?;
                Ok(DalvikBytecode::ConstString(reg, str_index))
            }

            CONST_STRING_JUMBO_OP => {
                let (reg, str_index) = reader.r_31c()?;
                Ok(DalvikBytecode::ConstStringJumbo(reg, str_index))
            }

            CONST_CLASS_OP => {
                let (reg, type_index) = reader.r_21c()?;
                Ok(DalvikBytecode::ConstClass(reg, type_index))
            }

            MONITOR_ENTER_OP => {
                let reg = reader.r_11x()?;
                Ok(DalvikBytecode::MonitorEnter(reg))
            }

            MONITOR_EXIT_OP => {
                let reg = reader.r_11x()?;
                Ok(DalvikBytecode::MonitorExit(reg))
            }

            CHECK_CAST_OP => {
                let (_reg, _type_index): (u8, u16) = reader.r_21c()?;
                todo!();
            }

            INSTANCE_OF_OP => {
                let (dst, src, type_index) = reader.r_22c()?;
                Ok(DalvikBytecode::InstanceOf(dst, src, type_index))
            }

            ARRAY_LENGTH_OP => {
                let (dst, src) = reader.r_12x()?;
                Ok(DalvikBytecode::ArrayLength(dst, src))
            }

            NEW_INSTANCE_OP => {
                let (dst, type_index) = reader.r_21c()?;
                Ok(DalvikBytecode::NewInstance(dst, type_index))
            }

            NEW_ARRAY_OP => {
                let (dst, src, type_index) = reader.r_22c()?;
                Ok(DalvikBytecode::NewArray(dst, src, type_index))
            }

            FILLED_NEW_ARRAY_OP => {
                let (argument_regs, type_index) = reader.r_35c()?;
                Ok(DalvikBytecode::FilledNewArray(type_index, argument_regs))
            }

            FILLED_NEW_ARRAY_RANGE_OP => {
                let (reg, type_index, first_argument_reg) = reader.r_3rc()?;
                Ok(DalvikBytecode::FilledNewArrayRange(
                    reg,
                    type_index,
                    first_argument_reg,
                ))
            }

            FILL_ARRAY_DATA_OP => {
                let (reg, value) = reader.r_31t()?;
                Ok(DalvikBytecode::FilledArrayData(reg, value))
            }

            THROW_OP => {
                let reg = reader.r_11x()?;
                Ok(DalvikBytecode::Throw(reg))
            }

            GOTO_OP => {
                let reg = reader.r_10t()?;
                Ok(DalvikBytecode::Goto(reg))
            }

            GOTO16_OP => {
                let reg = reader.r_20t()?;
                Ok(DalvikBytecode::Goto16(reg))
            }

            GOTO32_OP => {
                let reg = reader.r_30t()?;
                Ok(DalvikBytecode::Goto32(reg))
            }

            PACKED_SWITCH_OP => {
                let (reg, value) = reader.r_31t()?;
                Ok(DalvikBytecode::PackedSwitch(reg, value))
            }

            SPARSE_SWITCH_OP => {
                let (reg, value) = reader.r_31t()?;
                Ok(DalvikBytecode::SparseSwitch(reg, value))
            }

            op @ CMPL_FLOAT_OP..=CMP_LONG_OP => {
                let (dst, src, regs) = reader.r_23x()?;
                Ok(DalvikBytecode::Cmp(
                    CmpKind::from_opcode(op),
                    dst,
                    src,
                    regs,
                ))
            }

            op @ IF_TEST_EQ_OP..=IF_TEST_LE_OP => {
                let (reg1, reg2, offset) = reader.r_22t()?;
                Ok(DalvikBytecode::IfTest(
                    IfKind::from_opcode(op),
                    reg1,
                    reg2,
                    offset,
                ))
            }

            op @ IF_TESTZ_EQ_OP..=IF_TESTZ_LE_OP => {
                let (reg, offset) = reader.r_21t()?;
                Ok(DalvikBytecode::IfTestZ(
                    IfKind::from_opcode(op),
                    reg,
                    offset,
                ))
            }

            op @ ARRAY_GET_OP..=ARRAY_PUT_SHORT_OP => {
                let (reg1, reg2, reg3) = reader.r_23x()?;
                Ok(DalvikBytecode::ArrayOp(
                    OpKind::from_opcode(op),
                    reg1,
                    reg2,
                    reg3,
                ))
            }

            op @ INSTANCE_GET_OP..=INSTANCE_PUT_SHORT_OP => {
                let (reg1, reg2, offset) = reader.r_22c()?;
                Ok(DalvikBytecode::InstanceOp(
                    OpKind::from_opcode(op),
                    reg1,
                    reg2,
                    offset,
                ))
            }

            op @ STATIC_GET_OP..=STATIC_PUT_SHORT_OP => {
                let (reg, offset) = reader.r_21c()?;
                Ok(DalvikBytecode::StaticOp(
                    OpKind::from_opcode(op),
                    reg,
                    offset,
                ))
            }

            _ => Err(errors::Error::InvalidOpcode),
        };
        let inst = dalvik_bytecode?;

        Ok(DalvikInstruction { inst, offset })
    }
}
// move this to another module?

pub struct SmaliDecoder<'a> {
    stream: &'a [u8],
}

#[allow(dead_code)]
impl<'a> SmaliDecoder<'a> {
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
