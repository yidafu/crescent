use crate::vm::lua_state::LuaVm;

use super::{Instruction, InstructionOperation};

pub fn for_prep(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, bx) = i.a_bx();

    vm.push_value(a + 2); // step
    vm.push_value(a); // init value
    vm.arith(Some(|a, b| a + b), Some(|a, b| a + b));
    vm.replace(a);
    // TODO judge RA(a) > MAX_INTEGER
    vm.add_pc(bx);
}

pub fn fot_loop(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, bx) = i.a_bx();
    // vm.push_value(index)
}
