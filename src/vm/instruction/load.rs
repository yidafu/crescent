use crate::vm::lua_state::LuaVm;

use super::{Instruction, InstructionOperation};

pub fn load_nil(i: Instruction, vm: &mut dyn LuaVm) {
    let (mut a, b, _) = i.abc();
    a += 1;
    vm.push_nil();
    for i in a..(a + b) {
        vm.copy(-1, i);
    }
    vm.pop(1);
}

pub fn load_i(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, sbx) = i.a_sbx();
    println!("LOADI {} {}", a, sbx);
}