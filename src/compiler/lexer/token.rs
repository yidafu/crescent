#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Token {
    pub kind: TokenType,
    pub value: String,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum TokenType {
    Eof,
    Vararg,
    SeparatorSemicolon,
    SeparetorComma,
    SeparatorDot,
    SeparatorColon,
    SeparatorLabel,
    SeparatorOpenParenthesis,
    SeparatorCloseParenthesis,
    SeparatorOpenBracket,
    SeparatorCloseBracket,
    SeparatorOpenBrace,
    SeparatorCloseBrace,
    OperatorAssign,
    OperatorMinus,
    OperatorWave,
    OperatorPlus,
    OperatorMultiply,
    OperatorDivide,
    OperatorIDivide,
    OperatorPow,
    OperatorMod,
    OperatorBand,
    OperatorBor,
    OperatorLen, // #
    OperatorShr,
    OperatorShl,
    OperatorConcat,
    OperatorLt,
    OperatorLe,
    OperatorGt,
    OperatorGe,
    OperatorEq,
    OperatorAnd,
    OperatorOr,
    OperatorNot,
    OperatorNotEqual,
    KeywrodBreak,
    KeywrodDo,
    KeywrodElse,
    KeywrodElseIf,
    KeywrodEnd,
    KeywrodFalse,
    KeywrodFor,
    KeywrodFunction,
    KeywrodGoto,
    KeywrodIf,
    KeywrodIn,
    KeywrodLocal,
    KeywrodNil,
    KeywrodRepeat,
    KeywrodReturn,
    KeywrodThen,
    KeywrodTrue,
    KeywrodUntil,
    KeywrodWhile,
    Identifier,
    Number,
    String,
    OperatorUnm,
    OperatorSub,
    OperatorBxor,
    OperatorBnot,
}

// impl Copy for Token {
//     fn clone(&self) -> Token {
//         // *self
//         Token {
//             kind: self.kind,
//             value: self.value.to_string(),
//         }
//     }
// }

impl Token {
    pub fn eof_token() -> Token {
        Token {
            kind: TokenType::Eof,
            value: String::from(""),
        }
    }

    pub fn semi_token() -> Token {
        Token {
            kind: TokenType::SeparatorSemicolon,
            value: String::from(";"),
        }
    }

    pub fn comma_token() -> Token {
        Token {
            kind: TokenType::SeparetorComma,
            value: String::from(","),
        }
    }

    pub fn vararg_token() -> Token {
        Token {
            kind: TokenType::Vararg,
            value: String::from("..."),
        }
    }

    pub fn concat_token() -> Token {
        Token {
            kind: TokenType::OperatorConcat,
            value: String::from(".."),
        }
    }

    pub fn dot_token() -> Token {
        Token {
            kind: TokenType::SeparatorDot,
            value: String::from("."),
        }
    }

    pub fn colon_token() -> Token {
        Token {
            kind: TokenType::SeparatorColon,
            value: String::from(":"),
        }
    }

    pub fn label_token() -> Token {
        Token {
            kind: TokenType::SeparatorLabel,
            value: String::from("::"),
        }
    }

    pub fn open_paren_token() -> Token {
        Token {
            kind: TokenType::SeparatorOpenParenthesis,
            value: String::from("("),
        }
    }

    pub fn close_paren_token() -> Token {
        Token {
            kind: TokenType::SeparatorCloseParenthesis,
            value: String::from(")"),
        }
    }

    pub fn open_bracket_token() -> Token {
        Token {
            kind: TokenType::SeparatorOpenParenthesis,
            value: String::from("["),
        }
    }

    pub fn close_bracket_token() -> Token {
        Token {
            kind: TokenType::SeparatorCloseParenthesis,
            value: String::from("]"),
        }
    }

    pub fn open_brace_token() -> Token {
        Token {
            kind: TokenType::SeparatorOpenBrace,
            value: String::from("{"),
        }
    }

    pub fn close_brace_token() -> Token {
        Token {
            kind: TokenType::SeparatorCloseBrace,
            value: String::from("}"),
        }
    }

    pub fn assign_token() -> Token {
        Token {
            kind: TokenType::OperatorAssign,
            value: String::from("="),
        }
    }

    pub fn equal_token() -> Token {
        Token {
            kind: TokenType::OperatorAssign,
            value: String::from("=="),
        }
    }

    pub fn plus_token() -> Token {
        Token {
            kind: TokenType::OperatorPlus,
            value: String::from("+"),
        }
    }

    pub fn minus_token() -> Token {
        Token {
            kind: TokenType::OperatorMinus,
            value: String::from("-"),
        }
    }

    pub fn mul_token() -> Token {
        Token {
            kind: TokenType::OperatorMultiply,
            value: String::from("*"),
        }
    }

    pub fn div_token() -> Token {
        Token {
            kind: TokenType::OperatorDivide,
            value: String::from("/"),
        }
    }

    pub fn idiv_token() -> Token {
        Token {
            kind: TokenType::OperatorIDivide,
            value: String::from("//"),
        }
    }

    pub fn pow_token() -> Token {
        Token {
            kind: TokenType::OperatorPow,
            value: String::from("^"),
        }
    }

    pub fn mod_token() -> Token {
        Token {
            kind: TokenType::OperatorMod,
            value: String::from("%"),
        }
    }

    pub fn band_token() -> Token {
        Token {
            kind: TokenType::OperatorBand,
            value: String::from("&"),
        }
    }

    pub fn bor_token() -> Token {
        Token {
            kind: TokenType::OperatorBor,
            value: String::from("|"),
        }
    }

    pub fn len_token() -> Token {
        Token {
            kind: TokenType::OperatorLen,
            value: String::from("#"),
        }
    }

    pub fn wave_token() -> Token {
        Token {
            kind: TokenType::OperatorWave,
            value: String::from("~"),
        }
    }

    pub fn not_eqaul_token() -> Token {
        Token {
            kind: TokenType::OperatorNotEqual,
            value: String::from("~="),
        }
    }

    pub fn gt_token() -> Token {
        Token {
            kind: TokenType::OperatorGt,
            value: String::from(">"),
        }
    }

    pub fn ge_token() -> Token {
        Token {
            kind: TokenType::OperatorGe,
            value: String::from(">="),
        }
    }

    pub fn shr_token() -> Token {
        Token {
            kind: TokenType::OperatorShr,
            value: String::from(">>"),
        }
    }

    pub fn lt_token() -> Token {
        Token {
            kind: TokenType::OperatorLt,
            value: String::from("<"),
        }
    }

    pub fn le_token() -> Token {
        Token {
            kind: TokenType::OperatorLe,
            value: String::from("<="),
        }
    }

    pub fn shl_token() -> Token {
        Token {
            kind: TokenType::OperatorShl,
            value: String::from("<<"),
        }
    }

    /**
     * TODO: string escape
     */
    pub fn string_token(value: &str) -> Token {
        Token {
            kind: TokenType::String,
            value: String::from(value),
        }
    }

    pub fn number_token(value: &str) -> Token {
        Token {
            kind: TokenType::Number,
            value: String::from(value),
        }
    }

    pub fn identifier_token(value: &str) -> Token {
        Token {
            kind: TokenType::Identifier,
            value: String::from(value),
        }
    }

    pub fn break_token() -> Token {
        Token {
            kind: TokenType::KeywrodBreak,
            value: String::from("break"),
        }
    }

    pub fn do_token() -> Token {
        Token {
            kind: TokenType::KeywrodDo,
            value: String::from("do"),
        }
    }

    pub fn else_token() -> Token {
        Token {
            kind: TokenType::KeywrodElse,
            value: String::from("else"),
        }
    }

    pub fn elseif_token() -> Token {
        Token {
            kind: TokenType::KeywrodElseIf,
            value: String::from("elseif"),
        }
    }

    pub fn end_token() -> Token {
        Token {
            kind: TokenType::KeywrodEnd,
            value: String::from("end"),
        }
    }

    pub fn false_token() -> Token {
        Token {
            kind: TokenType::KeywrodFalse,
            value: String::from("false"),
        }
    }

    pub fn for_token() -> Token {
        Token {
            kind: TokenType::KeywrodFor,
            value: String::from("for"),
        }
    }

    pub fn function_token() -> Token {
        Token {
            kind: TokenType::KeywrodFunction,
            value: String::from("function"),
        }
    }

    pub fn goto_token() -> Token {
        Token {
            kind: TokenType::KeywrodGoto,
            value: String::from("goto"),
        }
    }

    pub fn if_token() -> Token {
        Token {
            kind: TokenType::KeywrodIf,
            value: String::from("if"),
        }
    }

    pub fn in_token() -> Token {
        Token {
            kind: TokenType::KeywrodIn,
            value: String::from("in"),
        }
    }

    pub fn local_token() -> Token {
        Token {
            kind: TokenType::KeywrodLocal,
            value: String::from("local"),
        }
    }

    pub fn nil_token() -> Token {
        Token {
            kind: TokenType::KeywrodNil,
            value: String::from("nil"),
        }
    }

    pub fn repeat_token() -> Token {
        Token {
            kind: TokenType::KeywrodRepeat,
            value: String::from("repeat"),
        }
    }

    pub fn return_token() -> Token {
        Token {
            kind: TokenType::KeywrodReturn,
            value: String::from("return"),
        }
    }

    pub fn then_token() -> Token {
        Token {
            kind: TokenType::KeywrodThen,
            value: String::from("then"),
        }
    }

    pub fn true_token() -> Token {
        Token {
            kind: TokenType::KeywrodTrue,
            value: String::from("true"),
        }
    }

    pub fn until_token() -> Token {
        Token {
            kind: TokenType::KeywrodUntil,
            value: String::from("until"),
        }
    }

    pub fn while_token() -> Token {
        Token {
            kind: TokenType::KeywrodWhile,
            value: String::from("while"),
        }
    }
}
