#[repr(u32)]
pub enum Opcode {
    LUI = 0b0110111,
    AUIPC = 0b0010111,
    JAL = 0b1101111,
    /// the opcode for BEQ, BNE, BLT, BGE, BLTU and BGEU.
    BXX = 0b1100011,
    /// the opcode for LB, LH, LW, LBU and LHU
    LX = 0b0000011,
    /// the opcode for SB, SH, and SW.
    SX = 0b0100011,
    /// the opcode for `ADDI`, `SLTI`, `SLTIU`, `XORI`, `ORI`, `ANDI`, `SLLI`, `SRLI`, `SRAI`.
    MATHI = 0b0010011,
    /// the opcode for `ADD`, `SUB`, `SLL`, `SLT`, `SLTU`, `XOR`, `SRL`, `SRA`, `OR` and `AND`.
    MATHX = 0b0110011,
    ERR = 0xFF,
}

impl From<u32> for Opcode {
    fn from(val: u32) -> Opcode {
        use Opcode::*;
        match val & 0b1111111 {
            0b0110111 => LUI,
            0b0010111 => AUIPC,
            0b1101111 => JAL,
            0b1100011 => BXX,
            0b0000011 => LX,
            0b0100011 => SX,
            0b0010011 => MATHI,
            0b0110011 => MATHX,
            _ => ERR,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum Register {
    /// Hard-Coded Zero
    Zero = 0,
    /// Return Address
    Ra = 1,
    /// Stack Pointer
    Sp = 2,
    /// Global Pointer
    Gp = 3,
    /// Thread Pointer
    Tp = 4,
    /// Temporary/Alternate link Pointer
    T0 = 5,
    /// Temporary no.1
    T1 = 6,
    /// Temporary no.1
    T2 = 7,
    /// Saved register/frame pointer
    S0 = 8,
    /// Saved register
    S1 = 9,
    /// Function arguments / return values
    A0 = 10,
    /// Function arguments / return values
    A1 = 11,
    /// Function Arguments
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,

    ERR = 0xFF,
    // TODO floating point regs?
}

impl From<u32> for Register {
    fn from(v: u32) -> Register {
        use Register::*;
        match v {
            0 => Zero,
            1 => Ra,
            2 => Sp,
            3 => Gp,
            4 => Tp,
            5 => T0,
            6 => T1,
            7 => T2,
            8 => S0,
            9 => S1,
            10 => A0,
            11 => A1,
            12 => A2,
            13 => A3,
            14 => A4,
            15 => A5,
            16 => A6,
            17 => A7,
            18 => S2,
            19 => S3,
            20 => S4,
            21 => S5,
            22 => S6,
            23 => S7,
            24 => S8,
            25 => S9,
            26 => S10,
            27 => S11,
            28 => T3,
            29 => T4,
            30 => T5,
            31 => T6,
            _ => ERR,
        }
    }
}

pub trait InsType {}

pub struct RType {
    pub rd: Register,
    pub funct3: u32,
    pub rs1: Register,
    pub rs2: Register,
    pub funct7: u32,
}

impl From<u32> for RType {
    fn from(v: u32) -> RType {
        Self {
            rd: Register::from((v >> 7) & 0b11111),
            funct3: ((v >> 12) & 0b111),
            rs1: Register::from((v >> 15) & 0b11111),
            rs2: Register::from((v >> 20) & 0b11111),
            funct7: ((v >> 25) & 0b1111111),
        }
    }
}

pub struct IType {
    pub rd: Register,
    pub funct3: u32,
    pub rs1: Register,
    pub imm: u32,
}

impl From<u32> for IType {
    fn from(v: u32) -> IType {
        Self {
            rd: Register::from((v >> 7) & 0b11111),
            funct3: ((v >> 12) & 0b111),
            rs1: Register::from((v >> 15) & 0b11111),
            imm: ((v >> 20) & 0b111111111111),
        }
    }
}

pub struct SType {
    pub imm1: u32,
    pub funct3: u32,
    pub rs1: Register,
    pub rs2: Register,
    pub imm2: u32,
}

impl From<u32> for SType {
    fn from(v: u32) -> SType {
        Self {
            imm1: ((v >> 7) & 0b11111),
            funct3: ((v >> 12) & 0b111),
            rs1: Register::from((v >> 15) & 0b11111),
            rs2: Register::from((v >> 20) & 0b11111),
            imm2: ((v >> 25) & 0b1111111),
        }
    }
}

pub struct UType {
    pub rd: Register,
    pub imm: u32,
}

impl From<u32> for UType {
    fn from(v: u32) -> UType {
        Self {
            rd: Register::from((v >> 7) & 0b11111),
            imm: ((v >> 25) & 0b1111111),
        }
    }
}

impl InsType for RType {}
impl InsType for IType {}
impl InsType for SType {}
impl InsType for UType {}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum MATHX_OPS {
    add_sub = 0b000,
    sll = 0b001,
    slt = 0b010,
    sltu = 0b011,
    xor = 0b100,
    srl_sra = 0b101,
    or = 0b110,
    and = 0b111,
    err = 0xFF,
}
impl From<u32> for MATHX_OPS {
    fn from(v: u32) -> Self {
        use MATHX_OPS::*;
        match v {
            0b000 => add_sub,
            0b001 => sll,
            0b010 => slt,
            0b011 => sltu,
            0b100 => xor,
            0b101 => srl_sra,
            0b110 => or,
            0b111 => and,
            _ => err,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum MATHI_OPS {
    addi = 0b000,
    slti = 0b010,
    sltiu = 0b011,
    xori = 0b100,
    ori = 0b110,
    andi = 0b111,
    slli = 0b001,
    srli_srai = 0b101,
    err = 0xFF,
}
impl From<u32> for MATHI_OPS {
    fn from(v: u32) -> Self {
        use MATHI_OPS::*;
        let res = match v {
            0b000 => addi,
            0b001 => slli,
            0b010 => slti,
            0b011 => sltiu,
            0b100 => xori,
            0b110 => ori,
            0b101 => srli_srai,
            0b111 => andi,
            _ => err,
        };
        if res == err {
            panic!("ERROR INSTRUCTION {:b}", v);
        }
        res
    }
}
