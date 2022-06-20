use crate::compiler::{
    ast::expression::*,
    lexer::{chunk_stream::ChunkStream, lexer::Lexer, token::TokenType},
};

use super::parser::parse_block;

pub(crate) fn parse_expression_list(lexer: &mut Lexer) -> Vec<Box<dyn Expression>> {
    let mut exp_list: Vec<Box<dyn Expression>> = Vec::new();

    while lexer.peek_token().kind == TokenType::SeparetorComma {
        lexer.next_token(); // eat ,
        exp_list.push(parse_expression(lexer))
    }
    exp_list
}

pub(crate) fn parse_expression(lexer: &mut Lexer) -> Box<dyn Expression> {
    parse_expression_12(lexer)
}

fn parse_expression_12(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_11(lexer);
    while lexer.peek_token().kind == TokenType::OperatorOr {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_11(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_11(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_10(lexer);
    while lexer.peek_token().kind == TokenType::OperatorAnd {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_10(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_10(lexer: &mut Lexer) -> Box<dyn Expression> {
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
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_9(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_9(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_8(lexer);
    while lexer.peek_token().kind == TokenType::OperatorBor {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_8(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_8(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_7(lexer);
    while lexer.peek_token().kind == TokenType::OperatorWave {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_7(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_7(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_6(lexer);
    while lexer.peek_token().kind == TokenType::OperatorBand {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_6(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_6(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_5(lexer);
    while lexer.peek_token().kind == TokenType::OperatorShl
        || lexer.peek_token().kind == TokenType::OperatorShr
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_5(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_5(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp = parse_expression_4(lexer);

    if lexer.peek_token().kind != TokenType::OperatorConcat {
        return exp;
    }

    let mut exps = Vec::new();
    while lexer.peek_token().kind == TokenType::OperatorConcat {
        lexer.next_token();
        exps.push(parse_expression_4(lexer));
    }

    Box::new(ConcatExpression { exps })
}

fn parse_expression_4(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_3(lexer);
    while lexer.peek_token().kind == TokenType::OperatorPlus
        || lexer.peek_token().kind == TokenType::OperatorMinus
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_3(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_3(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_2(lexer);
    while lexer.peek_token().kind == TokenType::OperatorMultiply
        || lexer.peek_token().kind == TokenType::OperatorDivide
        || lexer.peek_token().kind == TokenType::OperatorMod
        || lexer.peek_token().kind == TokenType::OperatorIDivide
    {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            exp_l,
            exp_r: parse_expression_2(lexer),
            operator: operator.value,
        })
    }
    exp_l
}

fn parse_expression_2(lexer: &mut Lexer) -> Box<dyn Expression> {
    match lexer.peek_token().kind {
        TokenType::OperatorLen
        | TokenType::OperatorNot
        | TokenType::OperatorLen
        | TokenType::OperatorBnot
        | TokenType::OperatorUnm => {
            let operator = lexer.peek_token();
            lexer.next_token();
            Box::new(UnaryExpression {
                operator: operator.value,
                exp: parse_expression_2(lexer),
            })
        }
        _ => parse_expression_1(lexer),
    }
}

fn parse_expression_1(lexer: &mut Lexer) -> Box<dyn Expression> {
    let mut exp_l = parse_expression_0(lexer);
    if lexer.peek_token().kind == TokenType::OperatorPow {
        let operator = lexer.peek_token();
        lexer.next_token();
        exp_l = Box::new(BinaryExpression {
            operator: operator.value,
            exp_l,
            exp_r: parse_expression_2(lexer),
        })
    }
    exp_l
}

fn parse_expression_0(lexer: &mut Lexer) -> Box<dyn Expression> {
    match lexer.peek_token().kind {
        TokenType::Vararg => {
            lexer.next_token();
            Box::new(VarargExpression {})
        }
        TokenType::KeywrodNil => {
            lexer.next_token();
            Box::new(NilExpression {})
        }
        TokenType::KeywrodTrue => {
            lexer.next_token();
            Box::new(TrueExpression {})
        }
        TokenType::KeywrodFalse => {
            lexer.next_token();
            Box::new(FalseExpression {})
        }
        TokenType::String => {
            let token = lexer.next_token();
            Box::new(StringExpression { value: token.value })
        }
        TokenType::Number => parse_number_expression(lexer),
        TokenType::KeywrodFunction => {
            lexer.next_token();
            parse_function_defined_expression(lexer)
        }
        _ => parse_prefix_expression(lexer),
    }
}

fn parse_number_expression(lexer: &mut Lexer) -> Box<dyn Expression> {
    let token = lexer.peek_token();
    if token.value.contains('.') {
        lexer.next_token();
        Box::new(FloatExpression {
            value: token.value.parse::<f64>().unwrap(),
        })
    } else {
        lexer.next_token();
        Box::new(IntegerExpression {
            value: token.value.parse::<i64>().unwrap(),
        })
    }
}

pub(crate) fn parse_function_defined_expression(
    lexer: &mut Lexer,
) -> Box<FunctionDefinedExpression> {
    lexer.next_special_token(TokenType::SeparatorOpenParenthesis);
    let param_list = parse_param_list(lexer);
    lexer.next_special_token(TokenType::SeparatorCloseParenthesis);

    let block = parse_block(lexer);
    lexer.next_special_token(TokenType::KeywrodEnd);

    Box::new(FunctionDefinedExpression {
        param_list,
        is_vararg: false,
        block,
    })
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

pub(crate) fn parse_prefix_expression(lexer: &mut Lexer) -> Box<dyn Expression> {
    if lexer.peek_token().kind == TokenType::Identifier {
        Box::new(NameExpression {
            name: lexer.next_identifier_token().value,
        })
    } else {
        let exp = parse_parenthesis_expression(lexer);
        _parse_prefix_expression(lexer, exp)
    }
}

fn _parse_prefix_expression(
    lexer: &mut Lexer,
    mut exp: Box<dyn Expression>,
) -> Box<dyn Expression> {
    loop {
        exp = match lexer.peek_token().kind {
            TokenType::SeparatorOpenBracket => {
                lexer.next_token();
                let key_exp = parse_expression(lexer);
                lexer.next_special_token(TokenType::SeparatorCloseBracket);
                Box::new(TableAccessExpression {
                    prefix_exp: exp,
                    key_exp,
                })
            }
            TokenType::SeparatorDot => {
                lexer.next_token();
                let name = lexer.next_identifier_token();
                let key_exp = StringExpression { value: name.value };
                Box::new(TableAccessExpression {
                    prefix_exp: exp,
                    key_exp: Box::new(key_exp),
                })
            }
            TokenType::SeparatorColon
            | TokenType::SeparatorOpenParenthesis
            | TokenType::SeparatorOpenBrace
            | TokenType::String => parse_function_call_expression(lexer, exp),
            _ => return exp,
        };
    }
}

fn parse_parenthesis_expression(lexer: &mut Lexer) -> Box<dyn Expression> {
    lexer.next_special_token(TokenType::SeparatorOpenParenthesis);
    let exp = parse_expression(lexer);
    lexer.next_special_token(TokenType::SeparatorCloseParenthesis);
    Box::new(ParenthesisExpression { exp })
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
    prefix_exp: Box<dyn Expression>,
) -> Box<FunctionCallExpression> {
    let name_exp = parse_name_expression(lexer);
    let args = parse_args(lexer);

    Box::new(FunctionCallExpression {
        prefix_exp,
        name_exp,
        args,
    })
}

fn parse_name_expression(lexer: &mut Lexer) -> Box<StringExpression> {
    if lexer.peek_token().kind == TokenType::SeparatorColon {
        lexer.next_token();
        let token = lexer.next_identifier_token();
        Box::new(StringExpression { value: token.value })
    } else {
        Box::new(StringExpression {
            value: String::from(""),
        })
    }
}

fn parse_args(lexer: &mut Lexer) -> Vec<Box<dyn Expression>> {
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
            vec![Box::new(StringExpression {
                value: string.value,
            })]
        }
    }
}

#[test]
fn test_simple_expression() {
    let exp = parse_expression(&mut Lexer::new(ChunkStream::new("test.lua", "1 + 2 * 3")));

    print!("expression {:?}", exp)
}
