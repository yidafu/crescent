use std::fmt::{Debug, Formatter, Error};

use super::block::Block;

pub(crate) enum Expression {

}
#[derive(Debug)]
pub(crate) struct EmptyExpression {}
impl EmptyExpression {}

#[derive(Debug)]
pub(crate) struct NilExpression {}
impl NilExpression {}

#[derive(Debug)]
pub(crate) struct TrueExpression {}
impl TrueExpression {}


#[derive(Debug)]
pub(crate) struct FalseExpression {}
impl FalseExpression {}

#[derive(Debug)]
pub(crate) struct VarargExpression {}
impl VarargExpression {}

#[derive(Debug)]
pub(crate) struct IntegerExpression {
    pub(crate) value: i64,
}
impl IntegerExpression {}

#[derive(Debug)]
pub(crate) struct FloatExpression {
    pub(crate) value: f64,
}
impl FloatExpression {}

#[derive(Debug)]
pub(crate) struct StringExpression {
    pub(crate) value: String,
}
impl StringExpression {}
#[derive(Debug)]
pub(crate) struct NameExpression {
    pub(crate) name: String,
}
impl NameExpression {}

#[derive(Debug)]
pub(crate) struct UnaryExpression {
    pub(crate) operator: String,
    pub(crate) exp: Box<Expression>,
}
impl UnaryExpression {}

#[derive(Debug)]
pub(crate) struct BinaryExpression {
    pub(crate) operator: String,
    pub(crate) exp_l: Box<Expression>,
    pub(crate) exp_r: Box<Expression>,
}
impl BinaryExpression {}

#[derive(Debug)]
pub(crate) struct ConcatExpression {
    pub(crate) exps: Vec<Box<Expression>>,
}
impl ConcatExpression {}

#[derive(Debug)]
pub(crate) struct TableContructorExpression {
    pub(crate) key_exp: Vec<Box<Expression>>,
    pub(crate) value_exp: Vec<Box<Expression>>,
}
impl TableContructorExpression {}

#[derive(Debug)]
pub(crate) struct FunctionDefinedExpression {
    pub(crate) param_list: Vec<String>,
    pub(crate) is_vararg: bool,
    pub(crate) block: Box<Block>,
}

impl FunctionDefinedExpression {}

#[derive(Debug)]
pub(crate) struct ParenthesisExpression {
    pub(crate) exp: Box<Expression>,
}

impl ParenthesisExpression {}
#[derive(Debug)]
pub(crate) struct TableAccessExpression {
    pub(crate) prefix_exp: Box<Expression>,
    pub(crate) key_exp: Box<Expression>,
}
impl TableAccessExpression {}

#[derive(Debug)]
pub(crate) struct FunctionCallExpression {
    pub(crate) prefix_exp: Box<Expression>,
    pub(crate) name_exp: Box<StringExpression>,
    pub(crate) args: Vec<Box<Expression>>,
}
impl FunctionCallExpression {}
