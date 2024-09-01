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

#[derive(Debug)]
pub enum DexBytecode {
    Nop,

    Move(MoveKind, u8, u8),
    MoveFrom16(MoveKind, u8, u16),
    Move16(MoveKind, u16, u16),
    MoveResult(MoveKind, u8),

    Return(ReturnKind, u8),

    Const4(u8, u8),
    Const16(u8, u16),
    Const(u8, u32),
    ConstHigh16(u8, u16),
    ConstWide16(u8, u16),
    ConstWide32(u8, u32),
    ConstWide(u8, u64),
    ConstWideHigh16(u8, u16),
    ConstString(u8, u16),
    ConstStringJumbo(u8, u32),
    ConstClass(u8, u16),

    MonitorEnter(u8),
    MonitorExit(u8),

    InstanceOf(u8, u8, u16),
    ArrayLength(u8, u8),

    NewInstance(u8, u16),
    NewArray(u8, u8, u16),
    FilledNewArray()
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
