use crate::compiler::{
    ast::expression::{Expression, TableConstructorExpression},
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
};

use super::parse_expression::parse_expression;

pub fn parse_table_constructor_expression(lexer: &mut Lexer) -> Expression {
    lexer.next_if_special_token(TokenType::SeparatorOpenBrace);
    let (key_exps, value_exps) = parse_field_list(lexer);
    lexer.next_if_special_token(TokenType::SeparatorCloseBrace);

    Expression::TableConstructorExpression(TableConstructorExpression {
        key_exps,
        value_exps,
    })
}

pub fn parse_field_list(lexer: &mut Lexer) -> (Vec<Expression>, Vec<Expression>) {
    let mut key_exps = Vec::new();
    let mut value_exps = Vec::new();

    if lexer.peek_token().kind != TokenType::SeparatorCloseBrace {
        let (key, value) = parse_field(lexer);
        key_exps.push(key);
        value_exps.push(value);
        while is_field_separator(lexer.peek_token()) {
            lexer.next_token();
            if lexer.peek_token().kind != TokenType::SeparatorCloseBrace {
                let (key, value) = parse_field(lexer);
                key_exps.push(key);
                value_exps.push(value);
            } else {
                break;
            }
        }
    }

    (key_exps, value_exps)
}

pub fn parse_field(lexer: &mut Lexer) -> (Expression, Expression) {
    if lexer.peek_token().kind == TokenType::SeparatorOpenBracket {
        lexer.next_token(); // eat [
        let key = parse_expression(lexer);
        lexer.next_if_special_token(TokenType::SeparatorCloseBracket);
        lexer.next_if_special_token(TokenType::OperatorAssign);
        let value = parse_expression(lexer);
        (key, value)
    } else {
        let exp = parse_expression(lexer);
        match exp {
            Expression::NameString(name) => {
                lexer.next_token();
                lexer.next_if_special_token(TokenType::OperatorAssign);
                let key = Expression::StringExpression(name);
                let value = parse_expression(lexer);
                (key, value)
            }
            otherExp => (Expression::NilExpression, otherExp),
        }
    }
}

pub fn is_field_separator(token: Token) -> bool {
    match token.kind {
        TokenType::SeparetorComma | TokenType::SeparatorSemicolon => true,
        _ => false,
    }
}


#[test]
fn test_simple_table_constructor() {
    let exp = parse_table_constructor_expression(&mut Lexer::create("test.lua", "{  \"x\", \"y\"; x = 1, [30] = 23; 45 } "));

    println!("{:#?}", &exp)
}
