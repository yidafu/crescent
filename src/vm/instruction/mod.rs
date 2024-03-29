pub mod arith;
pub mod compare;
pub mod load;
pub mod misc;
pub mod repeat;

use super::{
    lua_state::LuaState,
    op_code::{OpArg, OpMode, OP_CODE},
};

pub type Instruction = u32;

const MAXARG_Bx: i32 = (1 << 17) - 1;
const MAXARG_sBx: i32 = MAXARG_Bx >> 1;

pub trait InstructionOperation {
    fn op_code(&self) -> usize;

    fn abc(&self) -> (i32, i32, i32);

    fn a_bx(&self) -> (i32, i32);

    fn a_sbx(&self) -> (i32, i32);

    fn ax(&self) -> i32;

    fn sj(&self) -> i32;

    fn k(&self) -> i32;

    fn bx(&self) -> i32;

    fn op_name(&self) -> &'static str;

    fn op_mode(&self) -> OpMode;

    fn b_mode(&self) -> OpArg;

    fn c_mode(&self) -> OpArg;

    fn execute(&self, state: &mut LuaState);
}

impl InstructionOperation for Instruction {
    fn op_code(&self) -> usize {
        (self & 0b111_1111) as usize
    }

    fn abc(&self) -> (i32, i32, i32) {
        let a = (self >> 7 & 0b1111_1111) as i32;
        let b = (self >> 16 & 0b1111_1111) as i32;
        let c = (self >> 24 & 0b1111_1111) as i32;
        (a, b, c)
    }

    fn a_bx(&self) -> (i32, i32) {
        let a = (self >> 7 & 0xff) as i32;

        let bx = (self >> 15 & (0x1_ff_ff)) as i32;
        (a, bx)
    }

    fn a_sbx(&self) -> (i32, i32) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_sBx)
    }

    fn ax(&self) -> i32 {
        (self >> 7) as i32
    }

    fn sj(&self) -> i32 {
        let offset_js = ((1 << 25) - 1) >> 1;
        ((self >> 7) & (0x1_ff_ff_ff)) as i32 - offset_js
    }
    fn k(&self) -> i32 {
        (self >> 15 & 0b1) as i32
    }
    fn bx(&self) -> i32 {
        let (_, b, c) = self.abc();
        b - (((1 << 8) - 1) >> 1)
    }
    fn op_name(&self) -> &'static str {
        OP_CODE[self.op_code()].name
    }

    fn op_mode(&self) -> OpMode {
        OP_CODE[self.op_code()].op_mode
    }

    fn b_mode(&self) -> OpArg {
        OP_CODE[self.op_code()].arg_b_mode
    }

    fn c_mode(&self) -> OpArg {
        OP_CODE[self.op_code()].arg_c_mode
    }

    fn execute(&self, state: &mut LuaState) {
        let action = OP_CODE[self.op_code()].action;
        action(self.clone(), state);
    }
}

#[test]
fn test_instruction() {
    fn print_operands(i: Instruction) {
        print!("op name => {:?}", i.op_name());

        match i.op_mode() {
            OpMode::IABC => {
                let (a, b, c) = i.abc();
                print!("\ta => {:?}", a);
                match i.b_mode() {
                    OpArg::OpArgN => print!("\tb => {:?}", b),
                    _ => print!("\tb => {:?}", -1 - b & 0xFF),
                }
                match i.c_mode() {
                    OpArg::OpArgN => print!("\tc => {:?}", c),
                    _ => print!("\tc => {:?}", -1 - c & 0xFF),
                }
                println!("");
            }
            OpMode::IABx => {
                let (a, bx) = i.a_bx();
                print!("\ta => {:?}", a);
                match i.b_mode() {
                    OpArg::OpArgK => print!("\tbx => {:?}", -1 - bx),
                    OpArg::OpArgU => print!("\tbx => {:?}", bx),
                    _ => (),
                }
                println!("");
            }
            OpMode::IAsBx => {
                let (a, sbx) = i.a_sbx();
                print!("\ta => {:?}\t sbx => {:?}", a, sbx);
                println!("");
            }
            OpMode::IAx => {
                let ax = i.ax();
                print!("\tax => {}", ax);
                println!("");
            }
        }
    }
    let codes: [Instruction; 3] = [81, 8, 16842950];
    for code in codes.iter() {
        print_operands(code.clone());
    }
}

// local a
// 81,
// 8,
// 16842950,
// 1       [1]     VARARGPREP      0
// 2       [1]     LOADNIL         0 0     ; 1 out
// 3       [1]     RETURN          1 1 1   ; 0 out

// print("hello Word!")
// 81, 11, 32899, 16908356, 16842822

// 1       [1]     VARARGPREP      0
// 2       [1]     GETTABUP        0 0 0   ; _ENV "print"
// 3       [1]     LOADK           1 1     ; "hello World!"
// 4       [1]     CALL            0 2 1   ; 1 in 0 out
// 5       [1]     RETURN          0 1 1   ; 0 out

#[test]
fn test_inst_move() {
    let i: Instruction = 2147483448;
    let sj = i.sj();
    assert_eq!(sj, -1)
}
