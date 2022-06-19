use std::fmt::{Debug, Formatter, Error};

use super::block::Block;

pub(crate) trait Expression {
 fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
  }
}

#[derive(Debug)]
pub(crate) struct EmptyExpression {}
impl Expression for EmptyExpression {}

#[derive(Debug)]
pub(crate) struct NilExpression {}
impl Expression for NilExpression {}

#[derive(Debug)]
pub(crate) struct TrueExpression {}
impl Expression for TrueExpression {}

#[derive(Debug)]
pub(crate) struct FalseExpression {}
impl Expression for FalseExpression {}

#[derive(Debug)]
pub(crate) struct VarargExpression {}
impl Expression for VarargExpression {}

#[derive(Debug)]
pub(crate) struct IntegerExpression {
    pub(crate) value: i64,
}
impl Expression for IntegerExpression {}

#[derive(Debug)]
pub(crate) struct FloatExpression {
    pub(crate) value: f64,
}
impl Expression for FloatExpression {}

#[derive(Debug)]
pub(crate) struct StringExpression {
    value: String,
}
impl Expression for StringExpression {}
pub(crate) struct NameExpression {
    name: String,
}
impl Expression for NameExpression {}

#[derive(Debug)]
pub(crate) struct UnaryExpression {
    pub(crate) operator: String,
    exp: Box<dyn Expression>,
}
impl Expression for UnaryExpression {}

#[derive(Debug)]
pub(crate) struct BinaryExpression {
    pub(crate) operator: String,
    exp_l: Box<dyn Expression>,
    exp_r: Box<dyn Expression>,
}
impl Expression for BinaryExpression {}

pub(crate) struct ConcatExpression {
    exps: Vec<Box<dyn Expression>>,
}
impl Expression for ConcatExpression {}

pub(crate) struct TableContructorExpression {
    key_exp: Vec<Box<dyn Expression>>,
    value_exp: Vec<Box<dyn Expression>>,
}
impl Expression for TableContructorExpression {}

pub(crate) struct FunctionDefinedExpression {
    param_list: Vec<String>,
    is_vararg: bool,
    block: Block,
}

impl Expression for FunctionDefinedExpression {}

pub(crate) struct ParenthesisExpression {
    exp: Box<dyn Expression>,
}

impl Expression for ParenthesisExpression {}
pub(crate) struct TableAccessExpression {
    prefix_exp: Box<dyn Expression>,
    key_exp: Box<dyn Expression>,
}
impl Expression for TableAccessExpression {}

pub(crate) struct FunctionCallExpression {
    prefix_exp: Box<dyn Expression>,
    name_exp: Box<StringExpression>,
    args: Vec<Box<dyn Expression>>,
}
impl Expression for FunctionCallExpression {}
