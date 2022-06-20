use std::fmt::Debug;

use super::{
    block::Block,
    expression::{Expression, FunctionDefinedExpression},
};

pub(crate) trait Statement : Debug {}

#[derive(Debug)]
pub(crate) struct EmptyStatement {}

impl Statement for EmptyStatement {}

#[derive(Debug)]
pub(crate) struct BreakStatement {}

impl Statement for BreakStatement {}

#[derive(Debug)]
pub(crate) struct LabelStatement {
    pub(crate) name: String,
}

impl Statement for LabelStatement {}

#[derive(Debug)]
pub(crate) struct GotoStatement {
    pub(crate) name: String,
}

impl Statement for GotoStatement {}

#[derive(Debug)]
pub(crate) struct DotStatement {}

impl Statement for DotStatement {}

#[derive(Debug)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Box<dyn Expression>,
    pub(crate) block: Box<Block>,
}

impl Statement for WhileStatement {}

#[derive(Debug)]
pub(crate) struct RepeatStatement {
    pub(crate) condition: Box<dyn Expression>,
    pub(crate) block: Box<Block>,
}

impl Statement for RepeatStatement {}

#[derive(Debug)]
pub(crate) struct IfStatement {
    pub(crate) condition: Box<dyn Expression>,
    pub(crate) then_block: Box<Block>,
    pub(crate) else_block: Box<dyn Statement>,
}

impl Statement for IfStatement {}

#[derive(Debug)]
pub(crate) struct ForStatement {
    pub(crate) initial: Box<dyn Expression>,
    pub(crate) condition: Box<dyn Expression>,
    pub(crate) increment: Box<dyn Expression>,
    pub(crate) block: Block,
}

impl Statement for ForStatement {}

#[derive(Debug)]
pub(crate) struct LocalVarDeclareStatement {
    pub(crate) name_list: Vec<String>,
    pub(crate) exp_list: Vec<Box<dyn Expression>>,
}

impl Statement for LocalVarDeclareStatement {}

#[derive(Debug)]
pub(crate) struct AssignStatement {
    pub(crate) var_list: Vec<Box<dyn Expression>>,
    pub(crate) exp_list: Vec<Box<dyn Expression>>,
}
impl Statement for AssignStatement {}

#[derive(Debug)]
pub(crate) struct LocalFunctionDefinedStatement {
    pub(crate) name: String,
    pub(crate) exp: Box<FunctionDefinedExpression>,
}
impl Statement for LocalFunctionDefinedStatement {}
