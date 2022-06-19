use super::expression::Expression;
use super::statement::Statement;

pub(crate) struct Block {
    pub statements: Vec<Box<dyn Statement>>,
    pub return_expression: Vec<Box<dyn Expression>>,
}
