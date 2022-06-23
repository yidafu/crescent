use crate::compiler::{
    ast::expression::*,
    lexer::{chunk_stream::ChunkStream, lexer::Lexer, token::TokenType},
};

use super::parser::parse_block;

pub(crate) fn parse_expression_list(lexer: &mut Lexer) -> Vec<Expression> {
    let mut exp_list: Vec<Expression> = Vec::new();

    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token(); // eat ,
        exp_list.push(parse_expression(lexer))
    }
    exp_list
}

pub(crate) fn parse_expression(lexer: &mut Lexer) -> Expression {
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
            let token = lexer.next_token();
            Expression::StringExpression( token.value )
        }
        TokenType::Number => parse_number_expression(lexer),
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
        Expression::FloatExpresion( token.value.parse::<f64>().unwrap())
    } else {
        lexer.next_token();
        Expression::IntegerExpression( token.value.parse::<i64>().unwrap())
    }
}

pub(crate) fn parse_function_defined_expression(
    lexer: &mut Lexer,
) -> Expression {
    lexer.next_special_token(TokenType::SeparatorOpenParenthesis);
    let param_list = parse_param_list(lexer);
    lexer.next_special_token(TokenType::SeparatorCloseParenthesis);

    let block = parse_block(lexer);
    lexer.next_special_token(TokenType::KeywrodEnd);
    Expression::function_defined_expression(param_list, false, block)
}

fn parse_param_list(lexer: &mut Lexer) -> Vec<String> {
    match lexer.peek_token().kind {
        TokenType::SeparatorCloseParenthesis => Vec::new(),
        TokenType::Vararg => {
            lexer.next_token();
            Vec::new()
        }
        _ => {
            let mut name_list = Vec::new();
            name_list.push(lexer.next_identifier_token().value);
            while lexer.peek_token().kind == TokenType::SeparetorComma {
                lexer.next_token();
                if lexer.peek_token().kind == TokenType::Identifier {
                    name_list.push(lexer.next_token().value);
                } else {
                    lexer.next_special_token(TokenType::Vararg);
                }
            }
            name_list
        }
    }
}

pub(crate) fn parse_prefix_expression(lexer: &mut Lexer) -> Expression {
    if lexer.peek_token().kind == TokenType::Identifier {
        Expression::NameString(lexer.next_identifier_token().value)
    } else {
        let exp = parse_parenthesis_expression(lexer);
        _parse_prefix_expression(lexer, exp)
    }
}

fn _parse_prefix_expression(lexer: &mut Lexer, mut exp: Expression) -> Expression {
    loop {
        exp = match lexer.peek_token().kind {
            TokenType::SeparatorOpenBracket => {
                lexer.next_token();
                let key_exp = parse_expression(lexer);
                lexer.next_special_token(TokenType::SeparatorCloseBracket);
                Expression::table_access_expression(exp, key_exp)

            }
            TokenType::SeparatorDot => {
                lexer.next_token();
                let name = lexer.next_identifier_token();
                let key_exp = Expression::StringExpression( name.value );
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
    lexer.next_special_token(TokenType::SeparatorOpenParenthesis);
    let exp = parse_expression(lexer);
    lexer.next_special_token(TokenType::SeparatorCloseParenthesis);
    Expression::parenthesis_expression( exp )
    // match exp {
    //   VarargExpression { }
    //     | FunctionCallExpressio { prefix_exp, name_exp, args }
    //     | NameExpression
    //     | TableAccessExpression => ,
    //   _ => exp
    // }
}

fn parse_function_call_expression(
    lexer: &mut Lexer,
    prefix_exp: Expression,
) -> Expression {
    let name_exp = parse_name_expression(lexer);
    let args = parse_args(lexer);
    Expression::function_call_expression(prefix_exp, name_exp, args)
}

fn parse_name_expression(lexer: &mut Lexer) -> Expression {
    if lexer.peek_token().kind == TokenType::SeparatorColon {
        lexer.next_token();
        let token = lexer.next_identifier_token();
        Expression::StringExpression( token.value )
    } else {
        Expression::StringExpression( String::from("") )
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
            lexer.next_special_token(TokenType::SeparatorCloseParenthesis);
            args
        }
        TokenType::SeparatorOpenBrace => {
            todo!()
        }
        _ => {
            let string = lexer.next_special_token(TokenType::String);
            vec![Expression::StringExpression(string.value)]
        }
    }
}

#[test]
fn test_simple_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new("test.lua", "1 + 2 * 3")));

    print!("expression {:?}", exp)
}
