use super::brain32::{self, Token};

pub fn translate(parsed : Vec<TokenType>) -> Vec<brain32::Token> {
    parsed.into_iter().flat_map(|tt| match tt {
        TokenType::Instruction(i) => vec![i],
        TokenType::Call(m) => match m {
            Method::Zero => TRANSLATION_METHOD_ZERO
        }
    }).collect()
}

pub enum TokenType {
    Instruction(brain32::Token),
    Call(Method)
}

pub enum Method {
    Zero
}

const TRANSLATION_METHOD_ZERO : Vec<brain32::Token> = vec![];