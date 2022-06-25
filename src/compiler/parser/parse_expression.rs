use crate::compiler::{
    ast::{expression::*, statement::Statement},
    lexer::{chunk_stream::ChunkStream, lexer::Lexer, token::TokenType},
};

use super::{
    parse_table_constructor_expression::parse_table_constructor_expression, parser::parse_block,
};

pub fn parse_expression_list(lexer: &mut Lexer) -> Vec<Expression> {
    let mut exp_list: Vec<Expression> = Vec::new();
    exp_list.push(parse_expression(lexer));

    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token(); // eat ,
        exp_list.push(parse_expression(lexer))
    }
    exp_list
}

pub fn parse_expression(lexer: &mut Lexer) -> Expression {
    parse_expression_12(lexer)
}

fn parse_expression_12(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_11(lexer);
    while lexer.peek_token().kind == TokenType::OperatorOr {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_11(lexer))
    }
    exp_l
}

fn parse_expression_11(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_10(lexer);
    while lexer.peek_token().kind == TokenType::OperatorAnd {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_10(lexer))
    }
    exp_l
}

fn parse_expression_10(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_9(lexer);
    while lexer.peek_token().kind == TokenType::OperatorGt
        || lexer.peek_token().kind == TokenType::OperatorLt
        || lexer.peek_token().kind == TokenType::OperatorGe
        || lexer.peek_token().kind == TokenType::OperatorLe
        || lexer.peek_token().kind == TokenType::OperatorLe
        || lexer.peek_token().kind == TokenType::OperatorNotEqual
        || lexer.peek_token().kind == TokenType::OperatorEq
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_9(lexer))
    }
    exp_l
}

fn parse_expression_9(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_8(lexer);
    while lexer.peek_token().kind == TokenType::OperatorBor {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_8(lexer))
    }
    exp_l
}

fn parse_expression_8(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_7(lexer);
    while lexer.peek_token().kind == TokenType::OperatorWave {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_7(lexer))
    }
    exp_l
}

fn parse_expression_7(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_6(lexer);
    while lexer.peek_token().kind == TokenType::OperatorBand {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_6(lexer))
    }
    exp_l
}

fn parse_expression_6(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_5(lexer);
    while lexer.peek_token().kind == TokenType::OperatorShl
        || lexer.peek_token().kind == TokenType::OperatorShr
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_5(lexer))
    }
    exp_l
}

fn parse_expression_5(lexer: &mut Lexer) -> Expression {
    let mut exp = parse_expression_4(lexer);

    if lexer.peek_token().kind != TokenType::OperatorConcat {
        return exp;
    }

    let mut exps = Vec::new();
    while lexer.peek_token().kind == TokenType::OperatorConcat {
        lexer.next_token();
        exps.push(parse_expression_4(lexer));
    }

    Expression::concat_expresion(exps)
}

fn parse_expression_4(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_3(lexer);
    while lexer.peek_token().kind == TokenType::OperatorPlus
        || lexer.peek_token().kind == TokenType::OperatorMinus
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_3(lexer))
    }
    exp_l
}

fn parse_expression_3(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_2(lexer);
    while lexer.peek_token().kind == TokenType::OperatorMultiply
        || lexer.peek_token().kind == TokenType::OperatorDivide
        || lexer.peek_token().kind == TokenType::OperatorMod
        || lexer.peek_token().kind == TokenType::OperatorIDivide
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_2(lexer))
    }
    exp_l
}

fn parse_expression_2(lexer: &mut Lexer) -> Expression {
    match lexer.peek_token().kind {
        TokenType::OperatorLen
        | TokenType::OperatorNot
        | TokenType::OperatorLen
        | TokenType::OperatorBnot
        | TokenType::OperatorUnm => {
            let operator = lexer.peek_token();
            lexer.next_token();
            Expression::unary_expression(operator.value, parse_expression_2(lexer))
        }
        _ => parse_expression_1(lexer),
    }
}

/// TODO: optimize
fn parse_expression_1(lexer: &mut Lexer) -> Expression {
    let mut exp_l = parse_expression_0(lexer);
    if lexer.peek_token().kind == TokenType::OperatorPow {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Expression::binary_expression(operator.value, exp_l, parse_expression_0(lexer))
    }
    exp_l
}

fn parse_expression_0(lexer: &mut Lexer) -> Expression {
    match lexer.peek_token().kind {
        TokenType::Vararg => {
            lexer.next_token();
            Expression::VarargExpression
        }
        TokenType::KeywrodNil => {
            lexer.next_token();
            Expression::NilExpression
        }
        TokenType::KeywrodTrue => {
            lexer.next_token();
            Expression::TrueExpression
        }
        TokenType::KeywrodFalse => {
            lexer.next_token();
            Expression::FalseExpression
        }
        TokenType::String => {
            let token = lexer.peek_token();
            lexer.next_token();
            Expression::StringExpression(token.value)
        }
        TokenType::Number => parse_number_expression(lexer),
        TokenType::SeparatorOpenBrace => parse_table_constructor_expression(lexer),
        TokenType::KeywrodFunction => {
            lexer.next_token();
            parse_function_defined_expression(lexer)
        }
        _ => parse_prefix_expression(lexer),
    }
}

fn parse_number_expression(lexer: &mut Lexer) -> Expression {
    let token = lexer.peek_token();
    if token.value.contains('.') {
        lexer.next_token();
        Expression::FloatExpresion(token.value.parse::<f64>().unwrap())
    } else {
        lexer.next_token();
        Expression::IntegerExpression(token.value.parse::<i64>().unwrap())
    }
}

pub fn parse_function_defined_expression(lexer: &mut Lexer) -> Expression {
    // eat function keyword
    lexer.should_be_special_token(TokenType::SeparatorOpenParenthesis);
    lexer.next_token();
    let (is_vararg, param_list) = parse_param_list(lexer);
    lexer.should_be_special_token(TokenType::SeparatorCloseParenthesis);
    lexer.next_token();

    let block = parse_block(lexer);
    lexer.should_be_special_token(TokenType::KeywrodEnd);
    lexer.next_token();
    Expression::function_defined_expression(param_list, is_vararg, block)
}

fn parse_param_list(lexer: &mut Lexer) -> (bool, Vec<String>) {
    match lexer.peek_token().kind {
        TokenType::SeparatorCloseParenthesis => (false, Vec::new()),
        TokenType::Vararg => {
            lexer.next_token();
            (false, Vec::new())
        }
        _ => {
            let mut is_vararg = false;
            let mut name_list = Vec::new();
            name_list.push(lexer.should_be_identifier_token().value);
            lexer.next_token();
            while lexer.peek_token().kind == TokenType::SeparetorComma {
                lexer.next_token();
                let token = lexer.peek_token();
                if token.kind == TokenType::Identifier {
                    name_list.push(token.value);
                    lexer.next_token();
                } else {
                    is_vararg = true;
                    lexer.should_be_special_token(TokenType::Vararg);
                    lexer.next_token();
                }
            }
            (is_vararg, name_list)
        }
    }
}

pub fn parse_prefix_expression(lexer: &mut Lexer) -> Expression {
    let mut exp;
    if lexer.peek_token().kind == TokenType::Identifier {
        exp = Expression::NameString(lexer.peek_token().value);
        lexer.next_token();
    } else {
        exp = parse_parenthesis_expression(lexer);
    }

    _parse_prefix_expression(lexer, exp)
}

fn _parse_prefix_expression(lexer: &mut Lexer, mut exp: Expression) -> Expression {
    loop {
        exp = match lexer.peek_token().kind {
            TokenType::SeparatorOpenBracket => {
                lexer.next_token();
                let key_exp = parse_expression(lexer);
                lexer.next_if_special_token(TokenType::SeparatorCloseBracket);
                Expression::table_access_expression(exp, key_exp)
            }
            TokenType::SeparatorDot => {
                lexer.next_token();
                let name = lexer.should_be_identifier_token();
                let key_exp = Expression::StringExpression(name.value);
                Expression::table_access_expression(exp, key_exp)
            }
            TokenType::SeparatorColon
            | TokenType::SeparatorOpenParenthesis
            | TokenType::SeparatorOpenBrace
            | TokenType::String => parse_function_call_expression(lexer, exp),
            _ => return exp,
        };
    }
}

fn parse_parenthesis_expression(lexer: &mut Lexer) -> Expression {
    lexer.next_if_special_token(TokenType::SeparatorOpenParenthesis);
    let exp = parse_expression(lexer);
    lexer.next_if_special_token(TokenType::SeparatorCloseParenthesis);
    Expression::parenthesis_expression(exp)
    // match exp {
    //   VarargExpression { }
    //     | FunctionCallExpressio { prefix_exp, name_exp, args }
    //     | NameExpression
    //     | TableAccessExpression => ,
    //   _ => exp
    // }
}

fn parse_function_call_expression(lexer: &mut Lexer, prefix_exp: Expression) -> Expression {
    let name_exp = parse_name_expression(lexer);
    let args = parse_args(lexer);
    Expression::function_call_expression(prefix_exp, name_exp, args)
}

fn parse_name_expression(lexer: &mut Lexer) -> Expression {
    if lexer.peek_token().kind == TokenType::SeparatorColon {
        lexer.next_token();
        let token = lexer.should_be_identifier_token();
        Expression::StringExpression(token.value)
    } else {
        Expression::StringExpression(String::from(""))
    }
}

fn parse_args(lexer: &mut Lexer) -> Vec<Expression> {
    match lexer.peek_token().kind {
        TokenType::SeparatorOpenParenthesis => {
            lexer.next_token();
            let mut args = Vec::new();
            if lexer.peek_token().kind != TokenType::SeparatorCloseParenthesis {
                args = parse_expression_list(lexer);
            }
            lexer.next_if_special_token(TokenType::SeparatorCloseParenthesis);
            args
        }
        TokenType::SeparatorOpenBrace => {
            todo!()
        }
        _ => {
            lexer.next_if_special_token(TokenType::String);
            let string = lexer.peek_token();
            vec![Expression::StringExpression(string.value)]
        }
    }
}

#[test]
fn test_simple_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new("test.lua", "1 + 2 * 3")));

    print!("expression {:#?}", exp)
}

#[test]
fn test_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new("test.lua", "1 * 2 + 3")));

    print!("expression {:#?}", exp)
}

#[test]
fn test_expresion_list() {
    let exp = parse_expression_list(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "1 * 2 + 3,1 + 2 * 3",
    )));

    println!("{:#?}", exp);
}
#[test]
fn test_function_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "function (param1, param2) break end",
    )));

    match exp {
        Expression::FunctionDefinedExpression(fnDef) => {
            assert_eq!(fnDef.is_vararg, false);
            assert_eq!(
                fnDef.param_list,
                vec!["param1".to_string(), "param2".to_string()]
            );
            // assert_eq!(fnDef.block.statements[0], Statement::BreakStatement);
        }
        _ => panic!("{:#?}", exp),
    }
}

#[test]
fn test_vararg_function_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "function (param1, param2, ...) break end",
    )));

    match exp {
        Expression::FunctionDefinedExpression(fnDef) => {
            assert_eq!(fnDef.is_vararg, true);
            assert_eq!(
                fnDef.param_list,
                vec!["param1".to_string(), "param2".to_string()]
            );
            // assert_eq!(fnDef.block.statements[0], Statement::BreakStatement);
        }
        _ => panic!("{:#?}", exp),
    }
}

#[test]
fn test_function_call_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new(
        "test.lua",
        "print(10,24)",
    )));
    match exp {
        Expression::FunctionCallExpression(fnCall) => {
            assert_eq!(
                fnCall.name_exp,
                Box::new(Expression::StringExpression("".to_string()))
            );
            assert_eq!(
                fnCall.prefix_exp,
                Box::new(Expression::NameString("print".to_string()))
            );

            assert_eq!(
                fnCall.args,
                vec![
                    Expression::integet_expresion(10),
                    Expression::integet_expresion(24)
                ]
            )
        }
        _ => panic!("{:#?}", exp),
    }
}
