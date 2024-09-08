#![allow(dead_code)]

/// all dalvik opcodes
pub const NOP_OP: u8 = 0x0;
pub const MOV_OP: u8 = 0x1;
pub const MOV_FROM16_OP: u8 = 0x2;
pub const MOV16_OP: u8 = 0x3;
pub const MOV_WIDE_OP: u8 = 0x4;
pub const MOV_WIDE_FROM16_OP: u8 = 0x5;
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
pub const CONST_WIDE_HIGH16_OP: u8 = 0x19;
pub const CONST_STRING_OP: u8 = 0x1a;
pub const CONST_STRING_JUMBO_OP: u8 = 0x1b;
pub const CONST_CLASS_OP: u8 = 0x1c;
pub const MONITOR_ENTER_OP: u8 = 0x1d;
pub const MONITOR_EXIT_OP: u8 = 0x1e;
pub const CHECK_CAST_OP: u8 = 0x1f;
pub const INSTANCE_OF_OP: u8 = 0x20;
pub const ARRAY_LENGTH_OP: u8 = 0x21;
pub const NEW_INSTANCE_OP: u8 = 0x22;
pub const NEW_ARRAY_OP: u8 = 0x23;
pub const FILLED_NEW_ARRAY_OP: u8 = 0x24;
pub const FILLED_NEW_ARRAY_RANGE_OP: u8 = 0x25;
pub const FILL_ARRAY_DATA_OP: u8 = 0x26;
pub const THROW_OP: u8 = 0x27;
pub const GOTO_OP: u8 = 0x28;
pub const GOTO16_OP: u8 = 0x29;
pub const GOTO32_OP: u8 = 0x2a;
pub const PACKED_SWITCH_OP: u8 = 0x2b;
pub const SPARSE_SWITCH_OP: u8 = 0x2c;
pub const CMPL_FLOAT_OP: u8 = 0x2d;
pub const CMPG_FLOAT_OP: u8 = 0x2e;
pub const CMPL_DOUBLE_OP: u8 = 0x2f;
pub const CMPG_DOUBLE_OP: u8 = 0x30;
pub const CMP_LONG_OP: u8 = 0x31;
pub const IF_TEST_EQ_OP: u8 = 0x32;
pub const IF_TEST_NE_OP: u8 = 0x33;
pub const IF_TEST_IT_OP: u8 = 0x34;
pub const IF_TEST_GE_OP: u8 = 0x35;
pub const IF_TEST_GT_OP: u8 = 0x36;
pub const IF_TEST_LE_OP: u8 = 0x37;
pub const IF_TESTZ_EQ_OP: u8 = 0x38;
pub const IF_TESTZ_NE_OP: u8 = 0x39;
pub const IF_TESTZ_IT_OP: u8 = 0x3a;
pub const IF_TESTZ_GE_OP: u8 = 0x3b;
pub const IF_TESTZ_GT_OP: u8 = 0x3c;
pub const IF_TESTZ_LE_OP: u8 = 0x3d;
pub const ARRAY_GET_OP: u8 = 0x44;
pub const ARRAY_GET_WIDE_OP: u8 = 0x45;
pub const ARRAY_GET_OBJECT_OP: u8 = 0x46;
pub const ARRAY_GET_BOOLEAN_OP: u8 = 0x47;
pub const ARRAY_GET_BYTE_OP: u8 = 0x48;
pub const ARRAY_GET_CHAR_OP: u8 = 0x49;
pub const ARRAY_GET_SHORT_OP: u8 = 0x4a;
pub const ARRAY_PUT_OP: u8 = 0x4b;
pub const ARRAY_PUT_WIDE_OP: u8 = 0x4c;
pub const ARRAY_PUT_OBJECT_OP: u8 = 0x4d;
pub const ARRAY_PUT_BOOLEAN_OP: u8 = 0x4e;
pub const ARRAY_PUT_BYTE_OP: u8 = 0x4f;
pub const ARRAY_PUT_CHAR_OP: u8 = 0x50;
pub const ARRAY_PUT_SHORT_OP: u8 = 0x51;
pub const INSTANCE_GET_OP: u8 = 0x52;
pub const INSTANCE_GET_WIDE_OP: u8 = 0x53;
pub const INSTANCE_GET_OBJECT_OP: u8 = 0x54;
pub const INSTANCE_GET_BOOLEAN_OP: u8 = 0x55;
pub const INSTANCE_GET_BYTE_OP: u8 = 0x56;
pub const INSTANCE_GET_CHAR_OP: u8 = 0x57;
pub const INSTANCE_GET_SHORT_OP: u8 = 0x58;
pub const INSTANCE_PUT_OP: u8 = 0x59;
pub const INSTANCE_PUT_WIDE_OP: u8 = 0x5a;
pub const INSTANCE_PUT_OBJECT_OP: u8 = 0x5b;
pub const INSTANCE_PUT_BOOLEAN_OP: u8 = 0x5c;
pub const INSTANCE_PUT_BYTE_OP: u8 = 0x5d;
pub const INSTANCE_PUT_CHAR_OP: u8 = 0x5e;
pub const INSTANCE_PUT_SHORT_OP: u8 = 0x5f;
pub const STATIC_GET_OP: u8 = 0x60;
pub const STATIC_GET_WIDE_OP: u8 = 0x61;
pub const STATIC_GET_OBJECT_OP: u8 = 0x62;
pub const STATIC_GET_BOOLEAN_OP: u8 = 0x63;
pub const STATIC_GET_BYTE_OP: u8 = 0x64;
pub const STATIC_GET_CHAR_OP: u8 = 0x65;
pub const STATIC_GET_SHORT_OP: u8 = 0x66;
pub const STATIC_PUT_OP: u8 = 0x67;
pub const STATIC_PUT_WIDE_OP: u8 = 0x68;
pub const STATIC_PUT_OBJECT_OP: u8 = 0x69;
pub const STATIC_PUT_BOOLEAN_OP: u8 = 0x6a;
pub const STATIC_PUT_BYTE_OP: u8 = 0x6b;
pub const STATIC_PUT_CHAR_OP: u8 = 0x6c;
pub const STATIC_PUT_SHORT_OP: u8 = 0x6d;

#[derive(Debug)]
pub enum DalvikBytecode {
    Nop,

    Move(MoveKind, u8, u8),
    MoveFrom16(MoveKind, u8, u16),
    Move16(MoveKind, u16, u16),
    MoveResult(MoveKind, u8),

    Return(ReturnKind, u8),

    Const4(u8, i8),
    Const16(u8, i16),
    Const(u8, i32),
    ConstHigh16(u8, i16),
    ConstWide16(u8, i16),
    ConstWide32(u8, i32),
    ConstWide(u8, u64),
    ConstWideHigh16(u8, i16),
    ConstString(u8, u16),
    ConstStringJumbo(u8, u32),
    ConstClass(u8, u16),

    MonitorEnter(u8),
    MonitorExit(u8),

    InstanceOf(u8, u8, u16),
    ArrayLength(u8, u8),

    NewInstance(u8, u16),

    NewArray(u8, u8, u16),
    FilledNewArray(u16, Vec<u8>),
    FilledNewArrayRange(u8, u16, u16),
    FilledArrayData(u8, i32),

    Throw(u8),

    Goto(i8),
    Goto16(i16),
    Goto32(i32),

    PackedSwitch(u8, i32),
    SparseSwitch(u8, i32),

    Cmp(CmpKind, u8, u8, u8),
    IfTest(IfKind, u8, u8, i16),
    IfTestZ(IfKind, u8, i16),

    ArrayOp(OpKind, u8, u8, u8),
    InstanceOp(OpKind, u8, u8, u16),
    StaticOp(OpKind, u8, u16),
}

#[derive(Debug)]
pub enum MoveKind {
    Move,
    MovWide,
    MoveObject,
    Exception,
}

impl MoveKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            MOV_OP | MOV_FROM16_OP | MOV16_OP | MOV_RESULT_OP => Self::Move,
            MOV_WIDE_OP | MOV_WIDE_FROM16_OP | MOV_WIDE16_OP | MOV_RESULT_WIDE_OP => Self::MovWide,
            MOV_OBJECT_OP | MOV_OBJECT_FROM16_OP | MOV_OBJECT16_OP | MOV_RESULT_OBJECT_OP => {
                Self::MoveObject
            }
            MOV_EXCEPTION_OP => Self::Exception,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ReturnKind {
    Return,
    ReturnWide,
    ReturnObject,
    ReturnVoid,
}

impl ReturnKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            RETURN_VOID_OP => Self::ReturnVoid,
            RETURN_OP => Self::Return,
            RETURN_WIDE_OP => Self::ReturnWide,
            RETURN_OBJECT_OP => Self::ReturnObject,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum CmpKind {
    CmplFloat,
    CmpgFloat,
    CmplDouble,
    CmpgDouble,
    CmpLong,
}

impl CmpKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            CMPL_FLOAT_OP => Self::CmplFloat,
            CMPG_FLOAT_OP => Self::CmpgFloat,
            CMPL_DOUBLE_OP => Self::CmplDouble,
            CMPG_DOUBLE_OP => Self::CmpgDouble,
            CMP_LONG_OP => Self::CmpLong,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum IfKind {
    Eq,
    Ne,
    It,
    Ge,
    Gt,
    Le,
}

impl IfKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            IF_TEST_EQ_OP | IF_TESTZ_EQ_OP => Self::Eq,
            IF_TEST_NE_OP | IF_TESTZ_NE_OP => Self::Ne,
            IF_TEST_IT_OP | IF_TESTZ_IT_OP => Self::It,
            IF_TEST_GE_OP | IF_TESTZ_GE_OP => Self::Ge,
            IF_TEST_GT_OP | IF_TESTZ_GT_OP => Self::Gt,
            IF_TEST_LE_OP | IF_TESTZ_LE_OP => Self::Le,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum OpKind {
    Get,
    GetWide,
    GetObject,
    GetBoolean,
    GetByte,
    GetChar,
    GetShort,
    Put,
    PutWide,
    PutObject,
    PutBoolean,
    PutByte,
    PutChar,
    PutShort,
}

impl OpKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            ARRAY_GET_OP | INSTANCE_GET_OP | STATIC_GET_OP => Self::Get,
            ARRAY_GET_WIDE_OP | INSTANCE_GET_WIDE_OP | STATIC_GET_WIDE_OP => Self::GetWide,
            ARRAY_GET_OBJECT_OP | INSTANCE_GET_OBJECT_OP | STATIC_GET_OBJECT_OP => Self::GetObject,
            ARRAY_GET_BOOLEAN_OP | INSTANCE_GET_BOOLEAN_OP | STATIC_GET_BOOLEAN_OP => {
                Self::GetBoolean
            }
            ARRAY_GET_BYTE_OP | INSTANCE_GET_BYTE_OP | STATIC_GET_BYTE_OP => Self::GetByte,
            ARRAY_GET_CHAR_OP | INSTANCE_GET_CHAR_OP | STATIC_GET_CHAR_OP => Self::GetChar,
            ARRAY_GET_SHORT_OP | INSTANCE_GET_SHORT_OP | STATIC_GET_SHORT_OP => Self::GetShort,
            ARRAY_PUT_OP | INSTANCE_PUT_OP | STATIC_PUT_OP => Self::Put,
            ARRAY_PUT_WIDE_OP | INSTANCE_PUT_WIDE_OP | STATIC_PUT_WIDE_OP => Self::PutWide,
            ARRAY_PUT_OBJECT_OP | INSTANCE_PUT_OBJECT_OP | STATIC_PUT_OBJECT_OP => Self::PutObject,
            ARRAY_PUT_BOOLEAN_OP | INSTANCE_PUT_BOOLEAN_OP | STATIC_PUT_BOOLEAN_OP => {
                Self::PutBoolean
            }
            ARRAY_PUT_BYTE_OP | INSTANCE_PUT_BYTE_OP | STATIC_PUT_BYTE_OP => Self::PutByte,
            ARRAY_PUT_CHAR_OP | INSTANCE_PUT_CHAR_OP | STATIC_PUT_CHAR_OP => Self::PutChar,
            ARRAY_PUT_SHORT_OP | INSTANCE_PUT_SHORT_OP | STATIC_PUT_SHORT_OP => Self::PutShort,
            _ => unreachable!(),
        }
    }
}
