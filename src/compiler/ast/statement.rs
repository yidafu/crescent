use std::fmt::Debug;

use super::{
    block::Block,
    expression::{Expression, FunctionDefinedExpression},
};

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    BreakStatement,
    LabelStatement(String),
    GotoStatement(String),
    DotStatement,
    WhileStatement(WhileStatement),
    RepeatStatement(RepeatStatement),
    /// https://snacky.blog/en/recursive-rust.html
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    LocalVarDeclareStatement(LocalVarDeclareStatement),
    AssignStatement(AssignStatement),
    LocalFunctionDefinedStatement(LocalFunctionDefinedStatement),
}

impl Statement {
    pub fn while_statement(condition: Expression, block: Block) -> Statement {
        Statement::WhileStatement(WhileStatement { condition, block })
    }

    pub fn repeat_statement(condition: Expression, block: Block) -> Statement {
        Statement::RepeatStatement(RepeatStatement { condition, block })
    }

    pub fn if_statement(condition: Expression, then_block: Block, else_block: Block) -> Statement {
        Statement::IfStatement(IfStatement {
            condition,
            then_block,
            else_block,
        })
    }
    pub fn for_statement(
        initial: Expression,
        condition: Expression,
        increment: Expression,
        block: Block,
    ) -> Statement {
        Statement::ForStatement(ForStatement {
            initial,
            condition,
            increment,
            block,
        })
    }

    pub fn local_var_declare_statement(
        name_list: Vec<String>,
        exp_list: Vec<Expression>,
    ) -> Statement {
        Statement::LocalVarDeclareStatement(LocalVarDeclareStatement {
            name_list,
            exp_list,
        })
    }

    pub fn assign_statement(var_list: Vec<Expression>, exp_list: Vec<Expression>) -> Statement {
        Statement::AssignStatement(AssignStatement { var_list, exp_list })
    }

    pub fn local_function_defined_statement(
        name: String,
        exp: Expression,
    ) -> Statement {
        Statement::LocalFunctionDefinedStatement(LocalFunctionDefinedStatement { name, exp })
    }
}

// #[derive(Debug)]
// pub struct EmptyStatement {}

// #[derive(Debug)]
// pub struct BreakStatement {}

// impl BreakStatement {}

// #[derive(Debug)]
// pub struct LabelStatement {
//     pub name: String,
// }

// impl LabelStatement {}

// #[derive(Debug)]
// pub struct GotoStatement {
//     pub name: String,
// }

// impl GotoStatement {
//     fn new(name: String) -> GotoStatement {
//         GotoStatement { name }
//     }
// }

// #[derive(Debug)]
// pub struct DotStatement {}

// impl DotStatement {}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}

impl WhileStatement {
    fn new(condition: Expression, block: Block) -> WhileStatement {
        WhileStatement { condition, block }
    }
}

#[derive(Debug)]
pub struct RepeatStatement {
    pub condition: Expression,
    pub block: Block,
}

impl RepeatStatement {}


pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Block,
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfStatement\n").field("\tcondition", &self.condition).field("\n\tthen_block", &self.then_block).field("\n\telse_block", &self.else_block).finish()
    }
}

#[derive(Debug)]
pub struct ForStatement {
    pub initial: Expression,
    pub condition: Expression,
    pub increment: Expression,
    pub block: Block,
}

impl ForStatement {}

#[derive(Debug)]
pub struct LocalVarDeclareStatement {
    pub name_list: Vec<String>,
    pub exp_list: Vec<Expression>,
}

impl LocalVarDeclareStatement {}

#[derive(Debug)]
pub struct AssignStatement {
    pub var_list: Vec<Expression>,
    pub exp_list: Vec<Expression>,
}
impl AssignStatement {}

#[derive(Debug)]
pub struct LocalFunctionDefinedStatement {
    pub name: String,
    pub exp: Expression,
}
impl LocalFunctionDefinedStatement {}

#[test]
fn test_statement_enum() {
    let stmt = Statement::LabelStatement(String::from(""));
    print!("{:?}", stmt);
}
