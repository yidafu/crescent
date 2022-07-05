use super::op_code::{OP_CODE, OpMode, OpArg};

type Instruction = u32;

const MAXARG_Bx: i32 = 1 << 18 - 1;
const MAXARG_sBx: i32 = MAXARG_Bx >> 1;

trait InstructionOperation {
    fn op_code(&self) -> usize;

    fn abc(&self) -> (i32, i32, i32);

    fn abx(&self) -> (i32, i32);

    fn a_sbx(&self) -> (i32, i32);

    fn ax(&self) -> i32;

    fn op_name(&self) -> &'static str;

    fn op_mode(&self) -> OpMode;

    fn b_mode(&self) -> OpArg;

    fn c_mode(&self) -> OpArg;
}

impl InstructionOperation for Instruction {
    fn op_code(&self) -> usize {
        (self & 0b111_1111) as usize
    }

    fn abc(&self) -> (i32, i32, i32) {
        let a = (self >> 6 & 0b1111_1111) as i32;
        let b = (self >> 14 & 0b1_1111_1111) as i32;
        let c = (self >> 23 & 0b1_1111_1111) as i32;
        (a, b, c)
    }

    fn abx(&self) -> (i32, i32) {
        let a = (self >> 6 & 0b1111_1111) as i32;
        let bx = (self >> 14) as i32;
        (a, bx)
    }

    fn a_sbx(&self) -> (i32, i32) {
        let (a, bx) = self.abx();
        (a, bx - MAXARG_sBx)
    }

    fn ax(&self) -> i32 {
        (self >> 6) as i32
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
}

#[test]
fn test_instruction() {

  fn print_operands(i: Instruction) {
    match i.op_mode() {
        OpMode::IABC => {
          let (a, b, c) = i.abc();
          print!("a => {:?}", a);
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
          let (a, bx) = i.abx();
          print!("a => {:?}", a);
          match i.b_mode() {
              OpArg::OpArgK =>
                print!("\tbx => {:?}", -1 - bx),
                OpArg::OpArgU =>
                print!("\tbx => {:?}", bx),
              _ => (),
          }
          println!("");
        },
        OpMode::IAsBx => {
          let (a, sbx) = i.a_sbx();
          print!("a => {:?}\t sbx => {:?}", a, sbx);
          println!("");
        },
        OpMode::IAx => {
          let ax = i.ax();
          print!("ax => {}", ax);
          println!("");
        },
    }

  }
  // 0b100000001000000001_1000110
  let codes: [Instruction; 1] = [0b1010001,];
    for code in codes.iter() {
      print_operands(code.clone());
    }

}
// 1010001
// 1000000010000000011000110
// 81,
//         8,
//         16842950,