const IABC: u8 = 0;
const IABx: u8 = 0;
const IAsBs: u8 = 0;
const IAx: u8 = 0;

const OpArgN: u8 = 0;
const OpArgU: u8 = 0;
const OpArgR: u8 = 0;
const OpArgK: u8 = 0;

enum OpCodeEnum {
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

struct OpCode {
  test_flag: u8,
  set_a_flag: u8,
  arg_b_mode: u8,
  arg_c_mode: u8,
  op_mode: u8,
  name: String,
}

impl OpCode {
  pub fn new(test_flag: u8, set_a_flag: u8, arg_b_mode: u8,  arg_c_mode: u8, op_mode: u8, name: String) -> OpCode {
    OpCode {
      test_flag, set_a_flag, arg_b_mode, arg_c_mode, op_mode, name
    }
  }
}

const OP_CODE: [OpCode; 1] = [
  OpCode::new(0, 1, OpArgR, OpArgN, IABC, "MOVE".to_string()),
];