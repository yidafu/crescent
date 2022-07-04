use std::fmt::Debug;

use super::block::Block;


#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expression {
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
    pub fn integet_expresion(value: i64) -> Expression {
        Expression::IntegerExpression(value)
    }

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
    pub fn parenthesis_expression(exp: Expression) -> Expression {
        Expression::ParenthesisExpression(ParenthesisExpression { exp: Box::new(exp) })
    }
    pub fn table_access_expression(prefix_exp: Expression, key_exp: Expression) -> Expression {
        Expression::TableAccessExpression(TableAccessExpression {
            prefix_exp: Box::new(prefix_exp),
            key_exp: Box::new(key_exp),
        })
    }

    pub fn table_constructor_expression(
        key_exps: Vec<Expression>,
        value_exps: Vec<Expression>,
    ) -> Expression {
        Expression::TableConstructorExpression(TableConstructorExpression {
            key_exps,
            value_exps,
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UnaryExpression {
    pub operator: String,
    pub exp: Box<Expression>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BinaryExpression {
    pub operator: String,
    pub exp_l: Box<Expression>,
    pub exp_r: Box<Expression>,
}
impl BinaryExpression {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ConcatExpression {
    pub exps: Vec<Expression>,
}
impl ConcatExpression {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TableConstructorExpression {
    pub key_exps: Vec<Expression>,
    pub value_exps: Vec<Expression>,
}
impl TableConstructorExpression {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionDefinedExpression {
    pub param_list: Vec<String>,
    pub is_vararg: bool,
    pub block: Block,
}

impl FunctionDefinedExpression {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ParenthesisExpression {
    pub exp: Box<Expression>,
}

impl ParenthesisExpression {}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TableAccessExpression {
    pub prefix_exp: Box<Expression>,
    pub key_exp: Box<Expression>,
}
impl TableAccessExpression {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCallExpression {
    pub prefix_exp: Box<Expression>,
    pub name_exp: Box<Expression>,
    pub args: Vec<Expression>,
}
impl FunctionCallExpression {}

#[test]
fn format_unary_expression() {
    print!(
        "{:#?}",
        Expression::unary_expression("+".to_string(), Expression::NilExpression)
    )
}
