use crate::vm::lua_state::LuaVm;

use super::{Instruction, InstructionOperation};

pub fn moving(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, b, _) = i.abc();
    println!("a:{}, b:{}", a, b);
    vm.copy(b, a);
}

pub fn jump(i: Instruction, vm: &mut dyn LuaVm) {
    let sj = i.sj();
    vm.add_pc(sj);
    // assert!(a == 0, "TODO");
}

pub fn len(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, b, _) = i.abc();
    vm.len(b);
    vm.replace(a);
}

pub fn concat(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, _, c) = i.abc();
    // let n = c - b + 1;

    vm.check_stack(c as usize);
    vm.concat(c as usize);
    vm.replace(a);
}
