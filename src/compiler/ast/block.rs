use super::expression::Expression;
use super::statement::Statement;

#[derive(Debug)]
pub(crate) struct Block {
    pub statements: Vec<Box<dyn Statement>>,
    pub return_expression: Vec<Box<dyn Expression>>,
}
