use std::ptr::null;

use crate::compiler::{
    ast::{
        block::Block,
        expression::{Expression, TableAccessExpression},
        statement::{
            AssignStatement, ForStatement, IfStatement, LocalFunctionDefinedStatement,
            LocalVarDeclareStatement, RepeatStatement, Statement, WhileStatement,
        },
    },
    lexer::{chunk_stream::ChunkStream, lexer::Lexer, token::TokenType},
    parser::{
        parse_expression::{
            parse_expression, parse_expression_list, parse_function_defined_expression,
            parse_prefix_expression,
        },
        parser::parse_block,
    },
};

pub fn parse_statement(lexer: &mut Lexer) -> Statement {
    match lexer.peek_token().kind {
        TokenType::SeparatorSemicolon => parse_empty_statement(lexer),
        TokenType::KeywrodBreak => parse_break_statement(lexer),
        TokenType::SeparatorLabel => parse_label_statement(lexer),
        TokenType::KeywrodGoto => parse_goto_statement(lexer),
        TokenType::KeywrodDo => parse_do_statement(lexer),
        TokenType::KeywrodWhile => parse_while_statement(lexer),
        TokenType::KeywrodRepeat => parse_repeat_statement(lexer),
        TokenType::KeywrodIf => parse_if_statement(lexer, true),
        TokenType::KeywrodFor => parse_for_statement(lexer),
        TokenType::KeywrodFunction => parse_function_defined_statement(lexer),
        TokenType::KeywrodLocal => parse_local_assign_or_function_defined_statement(lexer),
        _ => parse_assign_or_function_call_statement(lexer),
    }
}

fn parse_empty_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_token(); // eat ;
    Statement::EmptyStatement
}

fn parse_break_statement(lexer: &mut Lexer) -> Statement {
    lexer.should_be_special_token(TokenType::KeywrodBreak); // eat break
    lexer.next_token();
    Statement::BreakStatement
}

fn parse_label_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::SeparatorLabel); // eat ::
    let identifier = lexer.next_token();
    lexer.next_if_special_token(TokenType::SeparatorLabel); // eat ::
    Statement::LabelStatement(identifier.value)
}

fn parse_goto_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodGoto); // eat goto
    let identifier = lexer.next_token();
    Statement::GotoStatement(identifier.value)
}

fn parse_do_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodDo); // eat do
    let block = parse_block(lexer);

    lexer.next_if_special_token(TokenType::KeywrodEnd);
    Statement::DotStatement
}

fn parse_while_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodWhile);
    let condition = parse_expression(lexer);

    lexer.next_if_special_token(TokenType::KeywrodDo);
    let block = parse_block(lexer);
    lexer.next_if_special_token(TokenType::KeywrodEnd);
    Statement::while_statement(condition, block)
}

fn parse_repeat_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodRepeat);
    let block = parse_block(lexer);
    lexer.next_if_special_token(TokenType::KeywrodUntil);
    let condition = parse_expression(lexer);

    Statement::repeat_statement(condition, block)
}

fn parse_if_statement(lexer: &mut Lexer, is_if: bool) -> Statement {
    if is_if == true {
        lexer.next_if_special_token(TokenType::KeywrodIf);
    } else {
        lexer.next_if_special_token(TokenType::KeywrodElseIf);
    }

    let condition = parse_expression(lexer);
    lexer.next_if_special_token(TokenType::KeywrodThen);
    let then_block = parse_block(lexer);

    if lexer.peek_token().kind == TokenType::KeywrodElseIf {
        let else_statement = parse_if_statement(lexer, false);
        let else_block = Block {
            statements: vec![else_statement],
            return_expression: vec![],
        };
        Statement::if_statement(condition, then_block, else_block)
    } else if lexer.peek_token().kind == TokenType::KeywrodElse {
        lexer.next_token();
        let else_block = parse_block(lexer);
        Statement::if_statement(condition, then_block, else_block)
    } else {
        Statement::if_statement(
            condition,
            then_block,
            Block {
                statements: vec![],
                return_expression: vec![],
            },
        )
    }
}

fn parse_for_statement(lexer: &mut Lexer) -> Statement {
    Statement::for_statement(todo!(), todo!(), todo!(), todo!())
}

/**
 * @see https://www.lua.org/manual/5.4/manual.html#3.4.11
 */
fn parse_function_defined_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodFunction);
    let (has_colol, fn_name_exp) = parse_function_name(lexer);
    let fn_body_exp = parse_function_defined_expression(lexer);
    // TODO: has colon case
    Statement::assign_statement(vec![fn_name_exp], vec![fn_body_exp])
}

fn parse_function_name(lexer: &mut Lexer) -> (bool, Expression) {
    let fn_name = lexer.should_be_identifier_token();
    let mut exp: Expression = Expression::NameString(fn_name.value);

    while lexer.peek_token().kind == TokenType::SeparatorDot {
        lexer.next_token(); // eat .
        let name = lexer.should_be_identifier_token();
        let key_exp = Expression::StringExpression(name.value);

        exp = Expression::table_access_expression(exp, key_exp);
    }
    let mut has_colon = false;
    while lexer.peek_token().kind == TokenType::SeparatorColon {
        lexer.next_token(); // eat :
        let name = lexer.should_be_identifier_token();
        let key_exp = Expression::StringExpression(name.value);
        exp = Expression::table_access_expression(exp, key_exp);
        let has_colon = true;
    }

    (has_colon, exp)
}

fn parse_local_assign_or_function_defined_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodLocal);
    if lexer.peek_token().kind == TokenType::KeywrodFunction {
        _parse_local_function_defined_statement(lexer)
    } else {
        _parse_local_var_defined_statement(lexer)
    }
}

fn _parse_local_function_defined_statement(lexer: &mut Lexer) -> Statement {
    lexer.next_if_special_token(TokenType::KeywrodFunction);
    let name = lexer.should_be_identifier_token();
    let fn_body_exp = parse_function_defined_expression(lexer);

    Statement::local_function_defined_statement(name.value, fn_body_exp)
}

fn _parse_local_var_defined_statement(lexer: &mut Lexer) -> Statement {
    let var_name = lexer.should_be_identifier_token();

    let name_list = _parse_name_list(lexer);

    let mut exp_list: Vec<Expression> = Vec::new();

    if lexer.peek_token().kind == TokenType::OperatorAssign {
        lexer.next_token();
        exp_list = parse_expression_list(lexer);
    }
    Statement::local_var_declare_statement(name_list, exp_list)
}

fn _parse_name_list(lexer: &mut Lexer) -> Vec<String> {
    let mut name_list = Vec::new();
    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token();
        let token = lexer.should_be_identifier_token();
        name_list.push(token.value);
    }
    name_list
}

fn parse_assign_or_function_call_statement(lexer: &mut Lexer) -> Statement {
    let prefix_exp = parse_prefix_expression(lexer);

    if true {
        todo!()
    } else {
        parse_assign_statement(lexer)
    }
}

fn parse_assign_statement(lexer: &mut Lexer) -> Statement {
    let var_list = parse_var_list(lexer);
    lexer.next_if_special_token(TokenType::OperatorAssign); // eat =
    let exp_list = parse_expression_list(lexer);
    Statement::assign_statement(var_list, exp_list)
}

fn parse_var_list(lexer: &mut Lexer) -> Vec<Expression> {
    let mut var_list = Vec::new();
    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token();
        let exp = parse_prefix_expression(lexer);
        var_list.push(exp);
    }
    var_list
}

#[test]
fn test_parse_simple_while_statement() {
    let stmt = parse_statement(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "while true do break; end",
    )));
    print!("statement {:?}", stmt)
}

#[test]
fn test_parse_simple_if_statement() {
    let stmt = parse_statement(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "if true then break; else break; end",
    )));
    print!("statement {:?}", stmt)
}
