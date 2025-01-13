#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    // Move
    MOVE, // i, a, b; r[b] = r[a]
    SWAP, // i, a, b; swap(r[a], r[b])
    // Loads
    LOAD_I16_AS_I32, // i, a, bx; r[a] = (i32)bx
    LOAD_I16_AS_I64, // i, a, bx; r[a] = (i64)bx
    LOAD_U16_AS_U32, // i, a, bx; r[a] = (u32)bx
    LOAD_U16_AS_U64, // i, a, bx; r[a] = (u64)bx
    LOADK,           // i, a, bx; r[a] = constant[bx]
    LOAD_FALSE,      // i, a; r[a] = false
    LOAD_TRUE,       // i, a; r[a] = true
    LOAD_NULL,       // i, a; r[a] = null
    // Arithmatic
    ADD, // i, a, b, c; r[a] = r[b].__add(r[c])
    // Equality
    EQ,  // i, a, b, c; if r[a] == r[b] { r[c] = true } else { r[c] = false }
    NEQ, // i, a, b, c; if r[a] != r[b] { r[c] = true } else { r[c] = false }
    GT,  // i, a, b, c; if r[a] > r[b] { r[c] = true } else { r[c] = false }
    LT,  // i, a, b, c; if r[a] < r[b] { r[c] = true } else { r[c] = false }
    GTQ, // i, a, b, c; if r[a] >= r[b] { r[c] = true } else { r[c] = false }
    LTQ, // i, a, b, c; if r[a] <= r[b] { r[c] = true } else { r[c] = false }
    // Control
    JMP,  // i, a; pc = (int)r[a]
    JMPI, // i, ax; pc = pc + (int)ax;
    JEQ,
    // Call related
    SAVE_STATE, // i; rn[...] = rm[...]
    CALL,       // i, a, b, c; (function)globals[(string)r[a]](r[0], r[1], r[2], ..., r[b]) -> r[c]
    RECALL,     // r(n+1)[...] = rn[...]
    // Shared Libs
    LOAD_LIB, // i, a, b; import (file)a as b
    LOAD_SYM, // i, a, b; import a.b
    // Misc
    DBG_PRINT,
    Invalid(u8),
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            // Move
            0x01 => Opcode::MOVE,
            0x02 => Opcode::SWAP,
            // Loads
            0x10 => Opcode::LOAD_I16_AS_I32,
            0x11 => Opcode::LOADK,
            0x12 => Opcode::LOAD_FALSE,
            0x13 => Opcode::LOAD_TRUE,
            0x14 => Opcode::LOAD_NULL,
            0x15 => Opcode::LOAD_I16_AS_I64,
            0x16 => Opcode::LOAD_U16_AS_U32,
            0x17 => Opcode::LOAD_U16_AS_U64,
            // Arithmatic
            0x20 => Opcode::ADD,
            // Equality
            0x30 => Opcode::EQ,
            0x31 => Opcode::NEQ,
            0x32 => Opcode::GT,
            0x33 => Opcode::LT,
            0x34 => Opcode::GTQ,
            0x35 => Opcode::LTQ,
            // Function Call
            0x40 => Opcode::SAVE_STATE,
            0x41 => Opcode::CALL,
            0x42 => Opcode::RECALL,
            // Shared Libs
            0x50 => Opcode::LOAD_LIB,
            0x51 => Opcode::LOAD_SYM,
            // Jump
            0xe0 => Opcode::JMP,
            0xe1 => Opcode::JMPI,
            0xe2 => Opcode::JEQ,
            // VM Control
            0xfe => Opcode::DBG_PRINT,
            _ => Opcode::Invalid(value),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::MOVE => 0x01,
            Opcode::SWAP => 0x02,

            Opcode::LOAD_I16_AS_I32 => 0x10,
            Opcode::LOADK => 0x11,
            Opcode::LOAD_TRUE => 0x12,
            Opcode::LOAD_FALSE => 0x13,
            Opcode::LOAD_NULL => 0x14,
            Opcode::LOAD_I16_AS_I64 => 0x15,
            Opcode::LOAD_U16_AS_U32 => 0x16,
            Opcode::LOAD_U16_AS_U64 => 0x17,

            Opcode::ADD => 0x20,

            Opcode::EQ => 0x30,
            Opcode::NEQ => 0x31,
            Opcode::GT => 0x32,
            Opcode::LT => 0x33,
            Opcode::GTQ => 0x34,
            Opcode::LTQ => 0x35,

            Opcode::SAVE_STATE => 0x40,
            Opcode::CALL => 0x41,
            Opcode::RECALL => 0x42,

            Opcode::LOAD_LIB => 0x50,
            Opcode::LOAD_SYM => 0x51,

            Opcode::JMP => 0xe0,
            Opcode::JMPI => 0xe1,
            Opcode::JEQ => 0xe2,

            Opcode::DBG_PRINT => 0xfe,
            Opcode::Invalid(i) => i,
        }
    }
}
