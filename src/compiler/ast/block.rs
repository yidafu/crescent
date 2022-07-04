use super::expression::Expression;
use super::statement::Statement;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expression: Vec<Expression>,
}
