use std::fmt::{Debug, Formatter, Error};

use super::block::Block;

pub(crate) trait Expression : Debug {

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
    pub(crate) value: String,
}
impl Expression for StringExpression {}
#[derive(Debug)]
pub(crate) struct NameExpression {
    pub(crate) name: String,
}
impl Expression for NameExpression {}

#[derive(Debug)]
pub(crate) struct UnaryExpression {
    pub(crate) operator: String,
    pub(crate) exp: Box<dyn Expression>,
}
impl Expression for UnaryExpression {}

#[derive(Debug)]
pub(crate) struct BinaryExpression {
    pub(crate) operator: String,
    pub(crate) exp_l: Box<dyn Expression>,
    pub(crate) exp_r: Box<dyn Expression>,
}
impl Expression for BinaryExpression {}

#[derive(Debug)]
pub(crate) struct ConcatExpression {
    pub(crate) exps: Vec<Box<dyn Expression>>,
}
impl Expression for ConcatExpression {}

#[derive(Debug)]
pub(crate) struct TableContructorExpression {
    pub(crate) key_exp: Vec<Box<dyn Expression>>,
    pub(crate) value_exp: Vec<Box<dyn Expression>>,
}
impl Expression for TableContructorExpression {}

#[derive(Debug)]
pub(crate) struct FunctionDefinedExpression {
    pub(crate) param_list: Vec<String>,
    pub(crate) is_vararg: bool,
    pub(crate) block: Box<Block>,
}

impl Expression for FunctionDefinedExpression {}

#[derive(Debug)]
pub(crate) struct ParenthesisExpression {
    pub(crate) exp: Box<dyn Expression>,
}

impl Expression for ParenthesisExpression {}
#[derive(Debug)]
pub(crate) struct TableAccessExpression {
    pub(crate) prefix_exp: Box<dyn Expression>,
    pub(crate) key_exp: Box<dyn Expression>,
}
impl Expression for TableAccessExpression {}

#[derive(Debug)]
pub(crate) struct FunctionCallExpression {
    pub(crate) prefix_exp: Box<dyn Expression>,
    pub(crate) name_exp: Box<StringExpression>,
    pub(crate) args: Vec<Box<dyn Expression>>,
}
impl Expression for FunctionCallExpression {}
