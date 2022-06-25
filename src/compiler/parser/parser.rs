use crate::compiler::lexer::token::TokenType;

use super::super::ast::block::Block;
use super::super::ast::expression::Expression;
use super::super::ast::statement::Statement;
use super::super::lexer::lexer::Lexer;
use super::super::lexer::token::Token;
use super::parse_expression::parse_expression_list;
use super::parse_statement::parse_statement;

pub fn parse_block(lexer: &mut Lexer) -> Block {
    Block {
        statements: parse_statements(lexer),
        return_expression: parse_return_expression(lexer),
    }
}

fn parse_statements(lexer: &mut Lexer) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();

    while !is_return_or_block_end(lexer.peek_token()) {
        let statement = parse_statement(lexer);

        statements.push(statement);
    }

    statements
}

fn parse_return_expression(lexer: &mut Lexer) -> Vec<Expression> {
    let expressions = Vec::new();
    let token = lexer.peek_token();
    if token.kind != TokenType::KeywrodReturn {
        return expressions;
    }

    lexer.next_token(); // eat return keyword
    match lexer.peek_token().kind {
        TokenType::Eof
        | TokenType::KeywrodEnd
        | TokenType::KeywrodElse
        | TokenType::KeywrodElse
        | TokenType::KeywrodUntil => expressions,
        TokenType::SeparatorSemicolon => expressions,
        _ => {
            let exps = parse_expression_list(lexer);

            exps
        }
    }
}

fn is_return_or_block_end(token: Token) -> bool {
    match token.kind {
        TokenType::KeywrodReturn
        | TokenType::Eof
        | TokenType::KeywrodEnd
        | TokenType::KeywrodElse
        | TokenType::KeywrodElseIf
        | TokenType::KeywrodUntil => true,
        _ => false,
    }
}
