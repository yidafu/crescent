use std::fmt::Debug;

use super::block::Block;

#[derive(Debug)]
pub(crate) enum Expression {
    EmptyExpression,
    NilExpression,
    TrueExpression,
    FalseExpression,
    VarargExpression,
    IntegerExpression(i64),
    FloatExpresion(f64),
    StringExpression(String),
    NameString(String),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    ConcatExpression(ConcatExpression),
    TableConstructorExpression(TableConstructorExpression),
    FunctionDefinedExpression(FunctionDefinedExpression),
    ParenthesisExpression(ParenthesisExpression),
    TableAccessExpression(TableAccessExpression),
    FunctionCallExpression(FunctionCallExpression),
}

impl Expression {
    #[inline]
    pub fn unary_expression(operator: String, exp: Expression) -> Expression {
        Expression::UnaryExpression(UnaryExpression {
            operator,
            exp: Box::new(exp),
        })
    }

    pub fn binary_expression(operator: String, exp_l: Expression, exp_r: Expression) -> Expression {
        Expression::BinaryExpression(BinaryExpression {
            operator,
            exp_l: Box::new(exp_l),
            exp_r: Box::new(exp_r),
        })
    }

    pub fn concat_expresion(exps: Vec<Expression>) -> Expression {
        Expression::ConcatExpression(ConcatExpression { exps })
    }

    pub fn function_defined_expression(
        param_list: Vec<String>,
        is_vararg: bool,
        block: Block,
    ) -> Expression {
        Expression::FunctionDefinedExpression(FunctionDefinedExpression {
            param_list,
            is_vararg,
            block,
        })
    }

    pub fn function_call_expression(
        prefix_exp: Expression,
        name_exp: Expression,
        args: Vec<Expression>,
    ) -> Expression {
        Expression::FunctionCallExpression(FunctionCallExpression {
            prefix_exp: Box::new(prefix_exp),
            name_exp: Box::new(name_exp),
            args,
        })
    }
    pub fn parenthesis_expression(exp: Expression) -> Expression{
        Expression::ParenthesisExpression(ParenthesisExpression { exp: Box::new(exp) })
    }
    pub fn table_access_expression(prefix_exp: Expression, key_exp: Expression) -> Expression {
        Expression::TableAccessExpression(TableAccessExpression {
            prefix_exp: Box::new(prefix_exp),
            key_exp: Box::new(key_exp),
        })
    }

    pub fn table_constructor_expression(key_exp: Vec<Expression>, value_exp: Vec<Expression>) -> Expression {
        Expression::TableConstructorExpression( TableConstructorExpression { key_exp, value_exp })
    }
}

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
    pub(crate) exps: Vec<Expression>,
}
impl ConcatExpression {}

#[derive(Debug)]
pub(crate) struct TableConstructorExpression {
    pub(crate) key_exp: Vec<Expression>,
    pub(crate) value_exp: Vec<Expression>,
}
impl TableConstructorExpression {}

#[derive(Debug)]
pub(crate) struct FunctionDefinedExpression {
    pub(crate) param_list: Vec<String>,
    pub(crate) is_vararg: bool,
    pub(crate) block: Block,
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
    pub(crate) name_exp: Box<Expression>,
    pub(crate) args: Vec<Expression>,
}
impl FunctionCallExpression {}
