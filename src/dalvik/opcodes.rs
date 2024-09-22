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
pub const THROW_OP: u8 = 0x27;
pub const GOTO_OP: u8 = 0x28;
pub const GOTO16_OP: u8 = 0x29;
pub const GOTO32_OP: u8 = 0x2a;
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
pub const INVOKE_VIRTUAL_OP: u8 = 0x6e;
pub const INVOKE_SUPER_OP: u8 = 0x6f;
pub const INVOKE_DIRECT_OP: u8 = 0x70;
pub const INVOKE_STATIC_OP: u8 = 0x71;
pub const INVOKE_INTERFACE_OP: u8 = 0x72;
pub const INVOKE_VIRTUAL_RANGE_OP: u8 = 0x74;
pub const INVOKE_SUPER_RANGE_OP: u8 = 0x75;
pub const INVOKE_DIRECT_RANGE_OP: u8 = 0x76;
pub const INVOKE_STATIC_RANGE_OP: u8 = 0x77;
pub const INVOKE_INTERFACE_RANGE_OP: u8 = 0x78;
pub const NEG_INT_OP: u8 = 0x7b;
pub const NOT_INT_OP: u8 = 0x7c;
pub const NEG_LONG_OP: u8 = 0x7d;
pub const NOT_LONG_OP: u8 = 0x7e;
pub const NEG_FLOAT_OP: u8 = 0x7f;
pub const NEG_DOUBLE_OP: u8 = 0x80;
pub const INT_TO_LONG_OP: u8 = 0x81;
pub const INT_TO_FLOAT_OP: u8 = 0x82;
pub const INT_TO_DOUBLE_OP: u8 = 0x83;
pub const LONG_TO_INT_OP: u8 = 0x84;
pub const LONG_TO_FLOAT_OP: u8 = 0x85;
pub const LONG_TO_DOUBLE_OP: u8 = 0x86;
pub const FLOAT_TO_INT_OP: u8 = 0x87;
pub const FLOAT_TO_LONG_OP: u8 = 0x88;
pub const FLOAT_TO_DOUBLE_OP: u8 = 0x89;
pub const DOUBLE_TO_INT_OP: u8 = 0x8a;
pub const DOUBLE_TO_LONG_OP: u8 = 0x8b;
pub const DOUBLE_TO_FLOAT_OP: u8 = 0x8c;
pub const INT_TO_BYTE_OP: u8 = 0x8d;
pub const INT_TO_CHAR_OP: u8 = 0x8e;
pub const INT_TO_SHORT_OP: u8 = 0x8f;
pub const ADD_INT_OP: u8 = 0x90;
pub const SUB_INT_OP: u8 = 0x91;
pub const MUL_INT_OP: u8 = 0x92;
pub const DIV_INT_OP: u8 = 0x93;
pub const REM_INT_OP: u8 = 0x94;
pub const AND_INT_OP: u8 = 0x95;
pub const OR_INT_OP: u8 = 0x96;
pub const XOR_INT_OP: u8 = 0x97;
pub const SHL_INT_OP: u8 = 0x98;
pub const SHR_INT_OP: u8 = 0x99;
pub const USHR_INT_OP: u8 = 0x9a;
pub const ADD_LONG_OP: u8 = 0x9b;
pub const SUB_LONG_OP: u8 = 0x9c;
pub const MUL_LONG_OP: u8 = 0x9d;
pub const DIV_LONG_OP: u8 = 0x9e;
pub const REM_LONG_OP: u8 = 0x9f;
pub const AND_LONG_OP: u8 = 0xa0;
pub const OR_LONG_OP: u8 = 0xa1;
pub const XOR_LONG_OP: u8 = 0xa2;
pub const SHL_LONG_OP: u8 = 0xa3;
pub const SHR_LONG_OP: u8 = 0xa4;
pub const USHR_LONG_OP: u8 = 0xa5;
pub const ADD_FLOAT_OP: u8 = 0xa6;
pub const SUB_FLOAT_OP: u8 = 0xa7;
pub const MUL_FLOAT_OP: u8 = 0xa8;
pub const DIV_FLOAT_OP: u8 = 0xa9;
pub const REM_FLOAT_OP: u8 = 0xaa;
pub const ADD_DOUBLE_OP: u8 = 0xab;
pub const SUB_DOUBLE_OP: u8 = 0xac;
pub const MUL_DOUBLE_OP: u8 = 0xad;
pub const DIV_DOUBLE_OP: u8 = 0xae;
pub const REM_DOUBLE_OP: u8 = 0xaf;
pub const ADD_INT_2ADDR_OP: u8 = 0xb0;
pub const SUB_INT_2ADDR_OP: u8 = 0xb1;
pub const MUL_INT_2ADDR_OP: u8 = 0xb2;
pub const DIV_INT_2ADDR_OP: u8 = 0xb3;
pub const REM_INT_2ADDR_OP: u8 = 0xb4;
pub const AND_INT_2ADDR_OP: u8 = 0xb5;
pub const OR_INT_2ADDR_OP: u8 = 0xb6;
pub const XOR_INT_2ADDR_OP: u8 = 0xb7;
pub const SHL_INT_2ADDR_OP: u8 = 0xb8;
pub const SHR_INT_2ADDR_OP: u8 = 0xb9;
pub const USHR_INT_2ADDR_OP: u8 = 0xba;
pub const ADD_LONG_2ADDR_OP: u8 = 0xbb;
pub const SUB_LONG_2ADDR_OP: u8 = 0xbc;
pub const MUL_LONG_2ADDR_OP: u8 = 0xbd;
pub const DIV_LONG_2ADDR_OP: u8 = 0xbe;
pub const REM_LONG_2ADDR_OP: u8 = 0xbf;
pub const AND_LONG_2ADDR_OP: u8 = 0xc0;
pub const OR_LONG_2ADDR_OP: u8 = 0xc1;
pub const XOR_LONG_2ADDR_OP: u8 = 0xc2;
pub const SHL_LONG_2ADDR_OP: u8 = 0xc3;
pub const SHR_LONG_2ADDR_OP: u8 = 0xc4;
pub const USHR_LONG_2ADDR_OP: u8 = 0xc5;
pub const ADD_FLOAT_2ADDR_OP: u8 = 0xc6;
pub const SUB_FLOAT_2ADDR_OP: u8 = 0xc7;
pub const MUL_FLOAT_2ADDR_OP: u8 = 0xc8;
pub const DIV_FLOAT_2ADDR_OP: u8 = 0xc9;
pub const REM_FLOAT_2ADDR_OP: u8 = 0xca;
pub const ADD_DOUBLE_2ADDR_OP: u8 = 0xcb;
pub const SUB_DOUBLE_2ADDR_OP: u8 = 0xcc;
pub const MUL_DOUBLE_2ADDR_OP: u8 = 0xcd;
pub const DIV_DOUBLE_2ADDR_OP: u8 = 0xce;
pub const REM_DOUBLE_2ADDR_OP: u8 = 0xcf;
pub const ADD_INT_LIT16_OP: u8 = 0xd0;
pub const RSUB_INT_OP: u8 = 0xd1; // Reverse subtract
pub const MUL_INT_LIT16_OP: u8 = 0xd2;
pub const DIV_INT_LIT16_OP: u8 = 0xd3;
pub const REM_INT_LIT16_OP: u8 = 0xd4;
pub const AND_INT_LIT16_OP: u8 = 0xd5;
pub const OR_INT_LIT16_OP: u8 = 0xd6;
pub const XOR_INT_LIT16_OP: u8 = 0xd7;
pub const ADD_INT_LIT8_OP: u8 = 0xd8;
pub const RSUB_INT_LIT8_OP: u8 = 0xd9; // Reverse subtract with 8-bit literal
pub const MUL_INT_LIT8_OP: u8 = 0xda;
pub const DIV_INT_LIT8_OP: u8 = 0xdb;
pub const REM_INT_LIT8_OP: u8 = 0xdc;
pub const AND_INT_LIT8_OP: u8 = 0xdd;
pub const OR_INT_LIT8_OP: u8 = 0xde;
pub const XOR_INT_LIT8_OP: u8 = 0xdf;
pub const SHL_INT_LIT8_OP: u8 = 0xe0;
pub const SHR_INT_LIT8_OP: u8 = 0xe1;
pub const USHR_INT_LIT8_OP: u8 = 0xe2;
pub const INVOKE_POLYMORPHIC_OP: u8 = 0xfa;
pub const INVOKE_POLYMORPHIC_RANGE_OP: u8 = 0xfb;
pub const INVOKE_CUSTOM_OP: u8 = 0xfc;
pub const INVOKE_CUSTOM_RANGE_OP: u8 = 0xfd;
pub const CONST_METHOD_HANDLE_OP: u8 = 0xfe;
pub const CONST_METHOD_TYPE_OP: u8 = 0xff;

pub const PSEUDO_PACKED_SWITCH_OP: u8 = 0x1;
pub const PSEUDO_SPARSE_SWITCH_OP: u8 = 0x2;
pub const PSEUDO_FILL_ARRAY_DATA_OP: u8 = 0x3;

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

    Invoke(InvokeKind, Vec<u8>, u16),
    InvokeRange(InvokeKind, u8, u16, u16),

    Unop(UnopKind, u8, u8),
    Binop(ArithmeticKind, u8, u8, u8),
    Binop2Addr(ArithmeticKind, u8, u8),
    BinopLit16(ArithmeticKind, u8, u8, i16),
    BinopLit8(ArithmeticKind, u8, u8, i8),

    InvokePolymorphic(Vec<u8>, u16, u16),
    InvokePolymorphicRange(u8, u16, u16, u16),
    InvokeCustom(Vec<u8>, u16),
    InvokeCustomRange(u8, u16, u16),
    ConstMethodHandle(u8, u16),
    ConstMethodType(u8, u16),
}

impl DalvikBytecode {
    pub fn is_basic_block_terminator(&self) -> bool {
        match self {
            Self::Return(_, _) => true,
            Self::Goto(_) => true,
            Self::Goto16(_) => true,
            Self::Goto32(_) => true,
            Self::Throw(_) => true,
            _ => false,
        }
    }
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

#[derive(Debug)]
pub enum InvokeKind {
    Virtual,
    Super,
    Direct,
    Static,
    Interface,
}

impl InvokeKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            INVOKE_VIRTUAL_OP | INVOKE_VIRTUAL_RANGE_OP => Self::Virtual,
            INVOKE_SUPER_OP | INVOKE_SUPER_RANGE_OP => Self::Super,
            INVOKE_DIRECT_OP | INVOKE_DIRECT_RANGE_OP => Self::Direct,
            INVOKE_STATIC_OP | INVOKE_STATIC_RANGE_OP => Self::Static,
            INVOKE_INTERFACE_OP | INVOKE_INTERFACE_RANGE_OP => Self::Interface,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum UnopKind {
    NegInt,
    NotInt,
    NegLong,
    NotLong,
    NegFloat,
    NegDouble,
    IntToLong,
    IntToFloat,
    IntToDouble,
    LongToInt,
    LongToFloat,
    LongToDouble,
    FloatToInt,
    FloatToLong,
    FloatToDouble,
    DoubleToInt,
    DoubleToLong,
    DoubleToFloat,
    IntToByte,
    IntToChar,
    IntToShort,
}

impl UnopKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            NEG_INT_OP => Self::NegInt,
            NOT_INT_OP => Self::NotInt,
            NEG_LONG_OP => Self::NegLong,
            NOT_LONG_OP => Self::NotLong,
            NEG_FLOAT_OP => Self::NegFloat,
            NEG_DOUBLE_OP => Self::NegDouble,
            INT_TO_LONG_OP => Self::IntToLong,
            INT_TO_FLOAT_OP => Self::IntToFloat,
            INT_TO_DOUBLE_OP => Self::IntToDouble,
            LONG_TO_INT_OP => Self::LongToInt,
            LONG_TO_FLOAT_OP => Self::LongToFloat,
            LONG_TO_DOUBLE_OP => Self::LongToDouble,
            FLOAT_TO_INT_OP => Self::FloatToInt,
            FLOAT_TO_LONG_OP => Self::FloatToLong,
            FLOAT_TO_DOUBLE_OP => Self::FloatToDouble,
            DOUBLE_TO_INT_OP => Self::DoubleToInt,
            DOUBLE_TO_LONG_OP => Self::DoubleToLong,
            DOUBLE_TO_FLOAT_OP => Self::DoubleToFloat,
            INT_TO_BYTE_OP => Self::IntToByte,
            INT_TO_CHAR_OP => Self::IntToChar,
            INT_TO_SHORT_OP => Self::IntToShort,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticKind {
    AddInt,
    SubInt,
    MulInt,
    DivInt,
    RemInt,
    AndInt,
    OrInt,
    XorInt,
    ShlInt,
    ShrInt,
    UshrInt,
    AddLong,
    SubLong,
    MulLong,
    DivLong,
    RemLong,
    AndLong,
    OrLong,
    XorLong,
    ShlLong,
    ShrLong,
    UshrLong,
    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,
    RemFloat,
    AddDouble,
    SubDouble,
    MulDouble,
    DivDouble,
    RemDouble,
    RSubInt,
}

impl ArithmeticKind {
    pub fn from_opcode(op: u8) -> Self {
        match op {
            ADD_INT_OP | ADD_INT_2ADDR_OP | ADD_INT_LIT16_OP | ADD_INT_LIT8_OP => Self::AddInt,
            SUB_INT_OP | SUB_INT_2ADDR_OP => Self::SubInt,
            MUL_INT_OP | MUL_INT_2ADDR_OP | MUL_INT_LIT16_OP | MUL_INT_LIT8_OP => Self::MulInt,
            DIV_INT_OP | DIV_INT_2ADDR_OP | DIV_INT_LIT16_OP | DIV_INT_LIT8_OP => Self::DivInt,
            REM_INT_OP | REM_INT_2ADDR_OP | REM_INT_LIT16_OP | REM_INT_LIT8_OP => Self::RemInt,
            AND_INT_OP | AND_INT_2ADDR_OP | AND_INT_LIT16_OP | AND_INT_LIT8_OP => Self::AndInt,
            OR_INT_OP | OR_INT_2ADDR_OP | OR_INT_LIT16_OP | OR_INT_LIT8_OP => Self::OrInt,
            XOR_INT_OP | XOR_INT_2ADDR_OP | XOR_INT_LIT16_OP | XOR_INT_LIT8_OP => Self::XorInt,
            SHL_INT_OP | SHL_INT_2ADDR_OP | SHL_INT_LIT8_OP => Self::ShlInt,
            SHR_INT_OP | SHR_INT_2ADDR_OP | SHR_INT_LIT8_OP => Self::ShrInt,
            USHR_INT_OP | USHR_INT_2ADDR_OP | USHR_INT_LIT8_OP => Self::UshrInt,
            ADD_LONG_OP | ADD_LONG_2ADDR_OP => Self::AddLong,
            SUB_LONG_OP | SUB_LONG_2ADDR_OP => Self::SubLong,
            MUL_LONG_OP | MUL_LONG_2ADDR_OP => Self::MulLong,
            DIV_LONG_OP | DIV_LONG_2ADDR_OP => Self::DivLong,
            REM_LONG_OP | REM_LONG_2ADDR_OP => Self::RemLong,
            AND_LONG_OP | AND_LONG_2ADDR_OP => Self::AndLong,
            OR_LONG_OP | OR_LONG_2ADDR_OP => Self::OrLong,
            XOR_LONG_OP | XOR_LONG_2ADDR_OP => Self::XorLong,
            SHL_LONG_OP | SHL_LONG_2ADDR_OP => Self::ShlLong,
            SHR_LONG_OP | SHR_LONG_2ADDR_OP => Self::ShrLong,
            USHR_LONG_OP | USHR_LONG_2ADDR_OP => Self::UshrLong,
            ADD_FLOAT_OP | ADD_FLOAT_2ADDR_OP => Self::AddFloat,
            SUB_FLOAT_OP | SUB_FLOAT_2ADDR_OP => Self::SubFloat,
            MUL_FLOAT_OP | MUL_FLOAT_2ADDR_OP => Self::MulFloat,
            DIV_FLOAT_OP | DIV_FLOAT_2ADDR_OP => Self::DivFloat,
            REM_FLOAT_OP | REM_FLOAT_2ADDR_OP => Self::RemFloat,
            ADD_DOUBLE_OP | ADD_DOUBLE_2ADDR_OP => Self::AddDouble,
            SUB_DOUBLE_OP | SUB_DOUBLE_2ADDR_OP => Self::SubDouble,
            MUL_DOUBLE_OP | MUL_DOUBLE_2ADDR_OP => Self::MulDouble,
            DIV_DOUBLE_OP | DIV_DOUBLE_2ADDR_OP => Self::DivDouble,
            REM_DOUBLE_OP | REM_DOUBLE_2ADDR_OP => Self::RemDouble,
            RSUB_INT_OP | RSUB_INT_LIT8_OP => Self::RSubInt,
            _ => unreachable!(),
        }
    }
}
