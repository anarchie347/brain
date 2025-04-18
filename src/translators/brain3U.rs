use super::brain32::{self, Token};

pub fn translate(parsed : Vec<TokenType>) -> Vec<brain32::Token> {
    parsed.into_iter().flat_map(|tt| match tt {
        TokenType::Instruction(i) => vec![i],
        TokenType::Call(m) => match m {
            Method::Zero => TRANSLATION_METHOD_ZERO
        }.to_vec()
    }).collect();
}

pub enum TokenType {
    Instruction(brain32::Token),
    Call(Method)
}

pub enum Method {
    Zero
}

const TRANSLATION_METHOD_ZERO : &[brain32::Token] = &[
    brain32::Token::Open(&[brain32::SingleArg::Three]),
    brain32::Token::Minus(Some(brain32::SingleArg::Three), true),
    brain32::Token::Close(&[brain32::SingleArg::Three]),

    brain32::Token::Open(&[brain32::SingleArg::Two]),
    brain32::Token::Minus(Some(brain32::SingleArg::Two), true),
    brain32::Token::Close(&[brain32::SingleArg::Two]),

    brain32::Token::Open(&[brain32::SingleArg::One]),
    brain32::Token::Minus(Some(brain32::SingleArg::One), true),
    brain32::Token::Close(&[brain32::SingleArg::One]),

    brain32::Token::Open(&[brain32::SingleArg::Zero]),
    brain32::Token::Minus(Some(brain32::SingleArg::Zero), true),
    brain32::Token::Close(&[brain32::SingleArg::Zero])
];