use crate::vm::lua_state::{CampareOperator, LuaVm};

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

pub fn for_loop(i: Instruction, vm: &mut dyn LuaVm) {
    let (a, bx) = i.a_bx();
    let is_positive_step = vm.to_numberx((a + 2) as usize).unwrap_or(0.0) >= 0.0;

    if (is_positive_step && vm.compare(a, a + 1, CampareOperator::LessThen))
        || (!is_positive_step && vm.compare(a + 1, a, CampareOperator::LessThen))
    {
        vm.copy(a, a + 3);
        vm.add_pc(-bx);
    }
}
