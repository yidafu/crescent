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
    vm.push_integer(sbx.into());
    vm.replace(a);
}

pub fn load_k(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, bx) = i.a_bx();
    println!("a=>{}, bx=>{}", a, bx);
    vm.get_const(bx as usize);
    vm.replace(a);
}

pub fn add(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, b, c) = i.abc();
    vm.get_pk(c);
    vm.get_pk(b);

    vm.arith_i(a, |a, b| a + b, |a, b| a + b);
}
