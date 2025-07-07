use super::brain32::{self, Token};
use std::rc::Rc;

// implement clone for the Tokens
// then prob get rid of this method, go with a procedural approach rather than functional, using a vec that is appended to


pub fn translate(parsed : Vec<Rc<TokenType>>) -> Vec<Rc<brain32::Token>> {
    parsed.into_iter().flat_map(|tt| match tt {
        TokenType::Instruction(i) => &[i],
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