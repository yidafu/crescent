use super::{
    block::Block,
    expression::{Expression, FunctionDefinedExpression},
};

pub(crate) trait Statement {}

pub(crate) struct EmptyStatement {}

impl Statement for EmptyStatement {}

pub(crate) struct BreakStatement {}

impl Statement for BreakStatement {}

pub(crate) struct LabelStatement {
    name: String,
}

impl Statement for LabelStatement {}

pub(crate) struct GotoStatement {
    name: String,
}

impl Statement for GotoStatement {}

pub(crate) struct DotStatement {}

impl Statement for DotStatement {}

pub(crate) struct WhileStatement {
    condition: Box<dyn Expression>,
    block: Block,
}

impl Statement for WhileStatement {}

pub(crate) struct RepeatStatement {
    condition: Box<dyn Expression>,
    block: Block,
}

impl Statement for RepeatStatement {}

pub(crate) struct IfStatement {
    pub(crate) condition: Box<dyn Expression>,
    then_block: Box<Block>,
    pub(crate) else_block: IfStatement,
}

impl Statement for IfStatement {}

pub(crate) struct ForStatement {
    initial: Box<dyn Expression>,
    condition: Box<dyn Expression>,
    increment: Box<dyn Expression>,
    block: Block,
}

impl Statement for ForStatement {}

pub(crate) struct LocalVarDeclareStatement {
    name_list: Vec<String>,
    exp_list: Vec<Box<dyn Expression>>,
}

pub(crate) struct AssignStatement {
    var_list: Vec<Box<dyn Expression>>,
    exp_list: Vec<Box<dyn Expression>>,
}

pub(crate) struct LocalFunctionDefinedStatement {
    name: String,
    exp: FunctionDefinedExpression,
}
