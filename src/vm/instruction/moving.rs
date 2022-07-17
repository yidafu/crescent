use crate::vm::lua_state::LuaVm;

use super::{Instruction, InstructionOperation};

pub fn moving(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, b, _) = i.abc();
    println!("a:{}, b:{}", a, b);
    vm.copy(b, a);
}
