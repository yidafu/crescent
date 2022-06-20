use std::ptr::null;

use crate::compiler::{
    ast::{
        block::Block,
        expression::{
            Expression, NameExpression, StringExpression, TableAccessExpression, TrueExpression,
        },
        statement::{
            AssignStatement, BreakStatement, DotStatement, EmptyStatement, ForStatement,
            GotoStatement, IfStatement, LabelStatement, LocalFunctionDefinedStatement,
            LocalVarDeclareStatement, RepeatStatement, Statement, WhileStatement,
        },
    },
    lexer::{lexer::Lexer, token::TokenType, chunk_stream::ChunkStream},
    parser::{
        parse_expression::{
            parse_expression, parse_expression_list, parse_function_defined_expression,
            parse_prefix_expression,
        },
        parser::parse_block,
    },
};

pub(crate) fn parse_statement(lexer: &mut Lexer) -> Box<dyn Statement> {
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

fn parse_empty_statement(lexer: &mut Lexer) -> Box<EmptyStatement> {
    lexer.next_token(); // eat ;
    Box::new(EmptyStatement {})
}

fn parse_break_statement(lexer: &mut Lexer) -> Box<BreakStatement> {
    lexer.next_special_token(TokenType::KeywrodBreak); // eat break
    Box::new(BreakStatement {})
}

fn parse_label_statement(lexer: &mut Lexer) -> Box<LabelStatement> {
    lexer.next_special_token(TokenType::SeparatorLabel); // eat ::
    let identifier = lexer.next_token();
    lexer.next_special_token(TokenType::SeparatorLabel); // eat ::

    Box::new(LabelStatement {
        name: identifier.value,
    })
}

fn parse_goto_statement(lexer: &mut Lexer) -> Box<GotoStatement> {
    lexer.next_special_token(TokenType::KeywrodGoto); // eat goto
    let identifier = lexer.next_token();
    Box::new(GotoStatement {
        name: identifier.value,
    })
}

fn parse_do_statement(lexer: &mut Lexer) -> Box<DotStatement> {
    lexer.next_special_token(TokenType::KeywrodDo); // eat do
    let block = parse_block(lexer);

    lexer.next_special_token(TokenType::KeywrodEnd);
    Box::new(DotStatement {})
}

fn parse_while_statement(lexer: &mut Lexer) -> Box<WhileStatement> {
    lexer.next_special_token(TokenType::KeywrodWhile);
    let condition = parse_expression(lexer);

    lexer.next_special_token(TokenType::KeywrodDo);
    let block = parse_block(lexer);
    lexer.next_special_token(TokenType::KeywrodEnd);
    Box::new(WhileStatement { condition, block })
}

fn parse_repeat_statement(lexer: &mut Lexer) -> Box<RepeatStatement> {
    lexer.next_special_token(TokenType::KeywrodRepeat);
    let block = parse_block(lexer);
    lexer.next_special_token(TokenType::KeywrodUntil);
    let condition = parse_expression(lexer);

    Box::new(RepeatStatement { condition, block })
}

fn parse_if_statement(lexer: &mut Lexer, is_if: bool) -> Box<IfStatement> {
    if is_if == true {
        lexer.next_special_token(TokenType::KeywrodIf);
    } else {
        lexer.next_special_token(TokenType::KeywrodElseIf);
    }

    let condition = parse_expression(lexer);
    lexer.next_special_token(TokenType::KeywrodThen);
    let then_block = parse_block(lexer);

    if lexer.peek_token().kind == TokenType::KeywrodElseIf {
        let else_statement = parse_if_statement(lexer, false);
        Box::new(IfStatement {
            condition,
            then_block,
            else_block: else_statement,
        })
    } else if lexer.peek_token().kind == TokenType::KeywrodElse {
        lexer.next_token();
        let else_block = parse_block(lexer);
        Box::new(IfStatement {
            condition,
            then_block,
            else_block: Box::new(IfStatement {
                condition: Box::new(TrueExpression {}),
                then_block: else_block,
                else_block: Box::new(EmptyStatement {}),
            }),
        })
    } else {
        Box::new(IfStatement {
            condition,
            then_block,
            else_block:  Box::new(EmptyStatement {}),
        })
    }
}

fn parse_for_statement(lexer: &mut Lexer) -> Box<ForStatement> {
    Box::new(ForStatement {
        initial: todo!(),
        condition: todo!(),
        increment: todo!(),
        block: todo!(),
    })
}

/**
 * @see https://www.lua.org/manual/5.4/manual.html#3.4.11
 */
fn parse_function_defined_statement(lexer: &mut Lexer) -> Box<AssignStatement> {
    lexer.next_special_token(TokenType::KeywrodFunction);
    let fn_name_exp = parse_function_name(lexer);
    let fn_body_exp = parse_function_defined_expression(lexer);
    // TODO: has colon case
    Box::new(AssignStatement {
        var_list: vec![fn_name_exp],
        exp_list: vec![fn_body_exp],
    })
}

fn parse_function_name(lexer: &mut Lexer) -> Box<dyn Expression> {
    let fn_name = lexer.next_identifier_token();
    let mut exp: Box<dyn Expression> = Box::new(NameExpression {
        name: fn_name.value,
    });

    while lexer.peek_token().kind == TokenType::SeparatorDot {
        lexer.next_token(); // eat .
        let name = lexer.next_identifier_token();
        let key_exp = Box::new(StringExpression { value: name.value });
        exp = Box::new(TableAccessExpression {
            prefix_exp: exp,
            key_exp,
        });
    }

    while lexer.peek_token().kind == TokenType::SeparatorColon {
        lexer.next_token(); // eat :
        let name = lexer.next_identifier_token();
        let key_exp = Box::new(StringExpression { value: name.value });
        exp = Box::new(TableAccessExpression {
            prefix_exp: exp,
            key_exp,
        });
        let hasColon = true;
    }

    exp
}

fn parse_local_assign_or_function_defined_statement(lexer: &mut Lexer) -> Box<dyn Statement> {
    lexer.next_special_token(TokenType::KeywrodLocal);
    if lexer.peek_token().kind == TokenType::KeywrodFunction {
        _parse_local_function_defined_statement(lexer)
    } else {
        _parse_local_var_defined_statement(lexer)
    }
}

fn _parse_local_function_defined_statement(
    lexer: &mut Lexer,
) -> Box<LocalFunctionDefinedStatement> {
    lexer.next_special_token(TokenType::KeywrodFunction);
    let name = lexer.next_identifier_token();
    let fn_body_exp = parse_function_defined_expression(lexer);

    Box::new(LocalFunctionDefinedStatement {
        name: name.value,
        exp: fn_body_exp,
    })
}

fn _parse_local_var_defined_statement(lexer: &mut Lexer) -> Box<LocalVarDeclareStatement> {
    let var_name = lexer.next_identifier_token();

    let name_list = _parse_name_list(lexer);

    let mut exp_list: Vec<Box<dyn Expression>> = Vec::new();

    if lexer.peek_token().kind == TokenType::OperatorAssign {
        lexer.next_token();
        exp_list = parse_expression_list(lexer);
    }

    Box::new(LocalVarDeclareStatement {
        name_list,
        exp_list,
    })
}

fn _parse_name_list(lexer: &mut Lexer) -> Vec<String> {
    let mut name_list = Vec::new();
    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token();
        let token = lexer.next_identifier_token();
        name_list.push(token.value);
    }
    name_list
}

fn parse_assign_or_function_call_statement(lexer: &mut Lexer) -> Box<AssignStatement> {
    let prefix_exp = parse_prefix_expression(lexer);

    if true {
        todo!()
    } else {
        parse_assign_statement(lexer)
    }
}

fn parse_assign_statement(lexer: &mut Lexer) -> Box<AssignStatement> {
    let var_list = parse_var_list(lexer);
    lexer.next_special_token(TokenType::OperatorAssign); // eat =
    let exp_list = parse_expression_list(lexer);

    Box::new(AssignStatement { var_list, exp_list })
}

fn parse_var_list(lexer: &mut Lexer) -> Vec<Box<dyn Expression>> {
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

      let stmt = parse_statement(
        &mut Lexer::new(ChunkStream::new("test.lua", "while true do break; end"))
      );
      print!("statement {:?}", stmt)
}

#[test]
fn test_parse_simple_if_statement() {

      let stmt = parse_statement(
        &mut Lexer::new(ChunkStream::new("test.lua", "if true then break; else break; end"))
      );
      print!("statement {:?}", stmt)
}