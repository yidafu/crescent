use crate::vm::lua_state::{CampareOperator, LuaVm};

use super::{Instruction, InstructionOperation};

pub fn equal_i(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, _, _) = i.abc();
    let i_val = i.bx();
    vm.get_pk(a);
    vm.push_integer(i_val.into());
    println!("k: {}", i.k());
    if vm.compare(-1, -2, CampareOperator::Equal) {
        vm.add_pc(1);
    }
    vm.pop(2);
}
