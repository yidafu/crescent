#[derive(Debug, Clone, Copy)]
pub enum OpMode {
    IABC,
    IABx,
    IAsBx,
    IAx,
}


#[derive(Debug, Clone, Copy)]
pub enum OpArg {
    OpArgN,
    OpArgU,
    OpArgR,
    OpArgK,
}

pub enum OpCodeEnum {
    OpMove,
    OpLOADK,
    OpLOADKX,
    OpLOADBOOL,
    OpLOADNIL,
    OpGETUPVAL,
    OpGETTABUP,
    OpGETTABLE,
    OpSETTABUP,
    OpSETUPVAL,
    OpSETTABLE,
    OpNEWTABLE,
    OpSELF,
    OpADD,
    OpSUB,
    OpMUL,
    OpMOD,
    OpPOW,
    OpDIV,
    OpIDEV,
    OpBAND,
    OpBOR,
    OpBXOR,
    OpSHL,
    OpSHR,
    OpUNM,
    OpBNOT,
    OpNOT,
    OpLEN,
    OpCONCAT,
    OpJMP,
    OpEQ,
    OpLT,
    OpLE,
    OpTEST,
    OpTESTSET,
    OpCALL,
    OpTAILCALL,
    OpRETURN,
    OpFORLOOP,
    OpFORREAP,
    OpTFORCALL,
    OpSETLIST,
    OpCLOSUER,
    OpVARARG,
    OpEXTRAARG,
}

pub struct OpCode {
    pub test_flag: u8,
    pub set_a_flag: u8,
    pub arg_b_mode: OpArg,
    pub arg_c_mode: OpArg,
    pub op_mode: OpMode,
    pub name: &'static str,
}

pub const OP_CODE: [OpCode; 46] = [
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "Move",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABx,
        name: "LOADK",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgN,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABx,
        name: "LOADKX",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "LOADBOOL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "LOADNIL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "GETUPVAL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "GETTABUP",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "GETTABLE",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "GETTABUP",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "SETUPVAL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "SETTABLE",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "NEWTABLE",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "SELF",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "ADD",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "SUB",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "MUL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "MOD",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "POW",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "DIV",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "IDIV",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "BAND",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "BOR",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "SHL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "SHR",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "UNM",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "BNOT",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "LEN",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgR,
        op_mode: OpMode::IABC,
        name: "CONCAT",
    },
    OpCode {
        test_flag: 1,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IAsBx,
        name: "JMP",
    },
    OpCode {
        test_flag: 1,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "EQ",
    },
    OpCode {
        test_flag: 1,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "LT",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "LE",
    },
    OpCode {
        test_flag: 1,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgN,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "TEST",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgN,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "TESTSET",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgK,
        arg_c_mode: OpArg::OpArgK,
        op_mode: OpMode::IABC,
        name: "CALL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "TAILCALL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "RETURN",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IAsBx,
        name: "FORLOOP",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IAsBx,
        name: "FORREAP",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgN,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IABC,
        name: "FORCALL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgR,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "TFORCALL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgN,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IAsBx,
        name: "TFORCALL",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IAsBx,
        name: "SETLIST",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABx,
        name: "CLOSURE",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 1,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgN,
        op_mode: OpMode::IABC,
        name: "VARARG",
    },
    OpCode {
        test_flag: 0,
        set_a_flag: 0,
        arg_b_mode: OpArg::OpArgU,
        arg_c_mode: OpArg::OpArgU,
        op_mode: OpMode::IAx,
        name: "EXTRAARG",
    },
];