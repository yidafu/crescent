use crate::vm::lua_state::LuaVm;

use super::{Instruction, InstructionOperation};

#[inline]
fn arith_k(
    i: Instruction,
    vm: &mut dyn LuaVm,
    i_func: Option<fn(a: i64, a: i64) -> i64>,
    f_func: Option<fn(a: f64, b: f64) -> f64>,
) {
    let (a, b, c) = i.abc();
    vm.get_pk(b);
    vm.get_const(c as usize);
    vm.is_number(vm.get_top() - 1);

    vm.arith(i_func, f_func);
    vm.replace(a);
}

pub fn add_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a + b), Some(|a, b| a + b));
}

pub fn sub_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a - b), Some(|a, b| a - b));
}

pub fn mul_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a * b), Some(|a, b| a * b));
}

pub fn mod_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a % b), Some(|a, b| a % b));
}

pub fn pow_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, None, Some(|a, b| a.powf(b)));
}

pub fn div_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, None, Some(|a, b| a / b));
}

/// TODO idiv implement
pub fn idiv_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a / b), Some(|a, b| (a / b).floor()));
}

pub fn b_and_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a & b), None)
}
pub fn b_or_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a | b), None)
}

pub fn b_xor_k(i: Instruction, vm: &mut dyn LuaVm) {
    arith_k(i, vm, Some(|a, b| a ^ b), None)
}

#[inline]
fn arith_i(
    i: Instruction,
    vm: &mut dyn LuaVm,
    i_func: Option<fn(a: i64, a: i64) -> i64>,
    f_func: Option<fn(a: f64, b: f64) -> f64>,
) {
    let (a, b, c) = i.abc();
    let ic = c - (1 << 8 - 1) >> 1;
    vm.get_pk(b);
    vm.push_integer(ic.into());

    vm.arith(i_func, f_func);
    vm.replace(a);
}

// TODO: logic shift left
pub fn shl_i(i: Instruction, vm: &mut dyn LuaVm) {
    arith_i(i, vm, Some(|a, b| a >> b), None)
}

// TODO: logic shift right
pub fn shr_i(i: Instruction, vm: &mut dyn LuaVm) {
    arith_i(i, vm, Some(|a, b| a << b), None)
}

#[inline]
fn arith(
    i: Instruction,
    vm: &mut dyn LuaVm,
    i_func: Option<fn(a: i64, a: i64) -> i64>,
    f_func: Option<fn(a: f64, b: f64) -> f64>,
) {
    let (a, b, c) = i.abc();
    vm.get_pk(c);
    vm.get_pk(b);

    vm.arith(i_func, f_func);
    vm.replace(a);
}

pub fn add(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a + b), Some(|a, b| a + b));
}

pub fn sub(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a - b), Some(|a, b| a - b));
}

pub fn mul(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a * b), Some(|a, b| a * b));
}

pub fn mod_(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a % b), Some(|a, b| a % b));
}

pub fn pow(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, None, Some(|a, b| a.powf(b)));
}

pub fn div(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, None, Some(|a, b| a / b));
}

/// TODO idiv implement
pub fn idiv(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a / b), Some(|a, b| (a / b).floor()));
}

pub fn b_and(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a & b), None)
}
pub fn b_or(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a | b), None)
}

pub fn b_xor(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a ^ b), None)
}

// TODO: logic shift left
pub fn shl(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a >> b), None)
}

// TODO: logic shift right
pub fn shr(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, b| a << b), None)
}

pub fn unm(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, _| -a), Some(|a, _| -a))
}

pub fn b_not(i: Instruction, vm: &mut dyn LuaVm) {
    arith(i, vm, Some(|a, _| !a), None)
}
