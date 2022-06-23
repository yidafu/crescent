use std::fmt::Debug;

use super::{
    block::Block,
    expression::{Expression, FunctionDefinedExpression},
};

pub(crate) enum Statement {
    EmptyStatement,
    BreakStatement,
    LabelStatement,
    GotoStatement(String),
    DotStatement,
    WhileStatement(Expression, Block),
}

#[derive(Debug)]
pub(crate) struct EmptyStatement {}

#[derive(Debug)]
pub(crate) struct BreakStatement {}

impl BreakStatement {}

#[derive(Debug)]
pub(crate) struct LabelStatement {
    pub(crate) name: String,
}

impl LabelStatement {}

#[derive(Debug)]
pub(crate) struct GotoStatement {
    pub(crate) name: String,
}

impl GotoStatement {
    fn new(name: String) -> GotoStatement {
        GotoStatement { name }
    }
}

#[derive(Debug)]
pub(crate) struct DotStatement {}

impl DotStatement {}

#[derive(Debug)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) block: Block,
}

impl WhileStatement {
    fn new(condition: Expression, block: Block) -> WhileStatement {
        WhileStatement { condition, block }
    }
}

#[derive(Debug)]
pub(crate) struct RepeatStatement {
    pub(crate) condition: Box<Expression>,
    pub(crate) block: Box<Block>,
}

impl RepeatStatement {}

#[derive(Debug)]
pub(crate) struct IfStatement {
    pub(crate) condition: Box<Expression>,
    pub(crate) then_block: Box<Block>,
    pub(crate) else_block: Box<Statement>,
}

impl IfStatement {}

#[derive(Debug)]
pub(crate) struct ForStatement {
    pub(crate) initial: Box<Expression>,
    pub(crate) condition: Box<Expression>,
    pub(crate) increment: Box<Expression>,
    pub(crate) block: Block,
}

impl ForStatement {}

#[derive(Debug)]
pub(crate) struct LocalVarDeclareStatement {
    pub(crate) name_list: Vec<String>,
    pub(crate) exp_list: Vec<Box<Expression>>,
}

impl LocalVarDeclareStatement {}

#[derive(Debug)]
pub(crate) struct AssignStatement {
    pub(crate) var_list: Vec<Box<Expression>>,
    pub(crate) exp_list: Vec<Box<Expression>>,
}
impl AssignStatement {}

#[derive(Debug)]
pub(crate) struct LocalFunctionDefinedStatement {
    pub(crate) name: String,
    pub(crate) exp: Box<FunctionDefinedExpression>,
}
impl LocalFunctionDefinedStatement {}
