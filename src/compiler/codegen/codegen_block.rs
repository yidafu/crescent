use crate::compiler::ast::{block::Block, expression::Expression, statement::Statement};

use super::function_info::FunctionInfo;


pub fn codegen_block(func_info: &mut FunctionInfo, node: Block) {
  for stmt in node.statements {
    codegen_statement(func_info, stmt);
  }

  if node.return_expression.len() > 0 {
    codegen_return_statement(func_info, node.return_expression);
  }
}

pub fn codegen_statement(func_info: &mut FunctionInfo, node: Statement) {
  todo!()
}

pub fn codegen_return_statement(func_info: &mut FunctionInfo, exps: Vec<Expression>) {
  let exp_cout = exps.len();
  // TODO: emit reture

  let is_multi_return = is_vararg_or_func_call(exps.get(exp_cout - 1).unwrap());
  for i in 0..exp_cout {
    let exp = exps.get(i).unwrap();
    let r = func_info.alloc_register();
    if i == exp_cout - 1 && is_multi_return {
      codegen_expression(func_info, exp, r, -1);
    } else {
      codegen_expression(func_info, exp, r, 1);
    }
  }

  func_info.free_registers(exp_cout as i64);
  let a = func_info.used_regs;
  if is_multi_return {
    func_info.emit_return(a, -1);
  } else {
    func_info.emit_return(a, exp_cout.try_into().unwrap());
  }
}

pub fn is_vararg_or_func_call(exp: &Expression) -> bool {
  match exp {
      Expression::VarargExpression | Expression::FunctionCallExpression(_) => true,
      _ => false,
  }
}

pub fn codegen_expression(func_info: &mut FunctionInfo, exp: &Expression, arga: i64, argx: i64) {
  match exp {
      Expression::NilExpression => func_info.emit_load_nil(arga, argx),
      Expression::FalseExpression => func_info.emit_load_bool(arga, 0, 0),
      Expression::TrueExpression => func_info.emit_load_bool(arga, 1, 0),
      Expression::IntegerExpression(value) => func_info.emit_load_k(arga, value.clone()),
      Expression::FloatExpresion(value) => func_info.emit_load_k(arga, value.clone()),
  }
}