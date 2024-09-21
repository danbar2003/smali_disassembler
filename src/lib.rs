mod bytecode_format;
mod errors;
mod opcodes;

use bytecode_format::DexInstructionFormatReader;
use opcodes::*;

pub type Result<T> = std::result::Result<T, errors::Error>;
pub struct SmaliDecoder<'a> {
    stream: &'a [u8],
}

impl<'a> SmaliDecoder<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self { stream }
    }

    /// decode all instruction
    pub fn decode_all(&self, a: bool) -> Vec<DalvikInstruction> {
        // vector to hold instructions
        let mut instructions = vec![];

        // new reader so we start at the beginning
        let mut new_reader = DexInstructionFormatReader::new(&self.stream);

        // loop until finishing decoding all instructions
        while let Ok(inst) = DalvikInstruction::decode_instruction(&mut new_reader) {
            if a {
                println!("{:x?}", inst);
            }
            instructions.push(inst)
        }

        // decode all instructions
        instructions
    }
}
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
            NOP_OP => {
                let _ = reader.r_10x()?;
                Ok(DalvikBytecode::Nop)
            }

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

            RETURN_VOID_OP => {
                let dst_reg = reader.r_10x()?;
                Ok(DalvikBytecode::Return(ReturnKind::ReturnVoid, dst_reg))
            }

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
                let (reg, type_index): (u8, u16) = reader.r_21c()?;
                Ok(DalvikBytecode::ConstClass(reg, type_index))
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
                let (regs_count, type_index, first_argument_reg) = reader.r_3rc()?;
                Ok(DalvikBytecode::FilledNewArrayRange(
                    regs_count,
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

            op @ INVOKE_VIRTUAL_OP..=INVOKE_INTERFACE_OP => {
                let (regs, meth_id) = reader.r_35c()?;
                Ok(DalvikBytecode::Invoke(
                    InvokeKind::from_opcode(op),
                    regs,
                    meth_id,
                ))
            }

            op @ INVOKE_VIRTUAL_RANGE_OP..=INVOKE_INTERFACE_RANGE_OP => {
                let (reg, reg_count, meth_id) = reader.r_3rc()?;
                Ok(DalvikBytecode::InvokeRange(
                    InvokeKind::from_opcode(op),
                    reg,
                    reg_count,
                    meth_id,
                ))
            }

            op @ NEG_INT_OP..=INT_TO_SHORT_OP => {
                let (dst, src) = reader.r_12x()?;
                Ok(DalvikBytecode::Unop(UnopKind::from_opcode(op), dst, src))
            }

            op @ ADD_INT_OP..=REM_DOUBLE_OP => {
                let (dst, src1, src2) = reader.r_23x()?;
                Ok(DalvikBytecode::Binop(
                    ArithmeticKind::from_opcode(op),
                    dst,
                    src1,
                    src2,
                ))
            }

            op @ ADD_INT_2ADDR_OP..=REM_DOUBLE_2ADDR_OP => {
                let (dst, src) = reader.r_12x()?;
                Ok(DalvikBytecode::Binop2Addr(
                    ArithmeticKind::from_opcode(op),
                    dst,
                    src,
                ))
            }

            op @ ADD_INT_LIT16_OP..=XOR_INT_LIT16_OP => {
                let (dst, src, value) = reader.r_22s()?;
                Ok(DalvikBytecode::BinopLit16(
                    ArithmeticKind::from_opcode(op),
                    dst,
                    src,
                    value,
                ))
            }

            op @ ADD_INT_LIT8_OP..=USHR_INT_LIT8_OP => {
                let (dst, src, value) = reader.r_22b()?;
                Ok(DalvikBytecode::BinopLit8(
                    ArithmeticKind::from_opcode(op),
                    dst,
                    src,
                    value,
                ))
            }

            INVOKE_POLYMORPHIC_OP => {
                let (regs, meth_id, proto_id) = reader.r_45cc()?;
                Ok(DalvikBytecode::InvokePolymorphic(regs, meth_id, proto_id))
            }

            INVOKE_POLYMORPHIC_RANGE_OP => {
                let (reg, meth_id, dst_regs, proto_id) = reader.r_4rcc()?;
                Ok(DalvikBytecode::InvokePolymorphicRange(
                    reg, meth_id, dst_regs, proto_id,
                ))
            }

            INVOKE_CUSTOM_OP => {
                let (regs, call_site) = reader.r_35c()?;
                Ok(DalvikBytecode::InvokeCustom(regs, call_site))
            }

            INVOKE_CUSTOM_RANGE_OP => {
                let (regs_count, call_site, first_argument_reg) = reader.r_3rc()?;
                Ok(DalvikBytecode::InvokeCustomRange(
                    regs_count,
                    call_site,
                    first_argument_reg,
                ))
            }

            CONST_METHOD_HANDLE_OP => {
                let (dst, meth_id) = reader.r_21c()?;
                Ok(DalvikBytecode::ConstMethodHandle(dst, meth_id))
            }

            CONST_METHOD_TYPE_OP => {
                let (dst, type_index) = reader.r_21c()?;
                Ok(DalvikBytecode::ConstMethodType(dst, type_index))
            }

            _ => Err(errors::Error::InvalidOpcode),
        };
        let inst = dalvik_bytecode?;

        Ok(DalvikInstruction { inst, offset })
    }
}
// move this to another module?
