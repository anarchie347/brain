pub fn translate(source: Vec<CodeBlock>) -> String {
    let mut full = vec!['>'; 4]; //wraps code in arrows to go to D0 to start,
    full.append(&mut source.iter().flat_map(translate_block).collect());
    full.append(&mut vec!['<'; 4]);
    full.iter().collect()
}

pub fn parse(source: Vec<(Mode, String)>) -> Vec<CodeBlock> {
    source
        .iter()
        .map(|(mode, section)| CodeBlock {
            mode: *mode,
            code: parse_section(section),
        })
        .collect()
}

fn parse_section(section: &String) -> Vec<Token> {
    section.chars().map(|c| Token::parse(&c)).collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Mode {
    Data,
    Working,
    Raw,
}
pub struct CodeBlock {
    pub mode: Mode,
    pub code: Vec<Token>,
}
pub enum Token {
    Plus,
    Minus,
    Open,
    Close,
    Comma,
    Dot,
    Right,
    Left,
    Preserve(char),
}
impl Token {
    fn parse(c: &char) -> Token {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '[' => Token::Open,
            ']' => Token::Close,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '>' => Token::Right,
            '<' => Token::Left,
            x => Token::Preserve(*x),
        }
    }
}

fn translate_block(block: &CodeBlock) -> Vec<char> {
    let mode = block.mode;
    let mut translated: Vec<char> = block
        .code
        .iter()
        .flat_map(|i| translate_instruction(i, mode))
        .collect();
    if mode == Mode::Working {
        //W blocks require wrapping in <<< code >>> to move in/out of W memory
        let mut full = vec!['<'; 4];
        full.append(&mut translated);
        full.append(&mut vec!['>'; 4]);
        return full;
    }
    translated
}

fn translate_instruction(t: &Token, mode: Mode) -> Vec<char> {
    (match mode {
        Mode::Data => translate_token_data,
        Mode::Working => translate_token_working,
        Mode::Raw => translate_token_raw,
    })(t)
}

fn translate_token_data(t: &Token) -> Vec<char> {
    match t {
        Token::Plus => String::from(DATA_ADD),
        Token::Minus => String::from(DATA_SUB),
        Token::Open => String::from(DATA_OPEN),
        Token::Close => String::from(DATA_CLOSE),
        Token::Comma => String::from(","),
        Token::Dot => String::from("."),
        Token::Right => String::from(">>>>>"),
        Token::Left => String::from("<<<<<"),
        Token::Preserve(x) => x.to_string(),
    }
    .chars()
    .collect()
}

fn translate_token_working(t: &Token) -> Vec<char> {
    match t {
        Token::Plus => String::from("+"),
        Token::Minus => String::from("-"),
        Token::Open => String::from("["),
        Token::Close => String::from("]"),
        Token::Comma => String::from(","),
        Token::Dot => String::from("."),
        Token::Right => String::from(">>>>>"),
        Token::Left => String::from("<<<<<"),
        Token::Preserve(x) => x.to_string(),
    }
    .chars()
    .collect()
}

fn translate_token_raw(t: &Token) -> Vec<char> {
    match t {
        Token::Plus => String::from("+"),
        Token::Minus => String::from("-"),
        Token::Open => String::from("["),
        Token::Close => String::from("]"),
        Token::Comma => String::from(","),
        Token::Dot => String::from("."),
        Token::Right => String::from(">"),
        Token::Left => String::from("<"),
        Token::Preserve(x) => x.to_string(),
    }
    .chars()
    .collect()
}

//ERROR FOUND for ADD/SUB:
// IF cannot be nested, as they overwrite values in W, that may be needed by their containing IF statement to exit

// More complex substitution values
//basic data operations
const DATA_ADD: &str = concat!(
    "+",
    data_if_zero!(
        0,
        concat!(
            "<+>",
            data_if_zero!(1, concat!("<<+>>", data_if_zero!(2, "<<<+>>>", "")), "")
        ),
        ""
    )
);
const DATA_SUB: &str = concat!(
    data_if_zero!(
        0,
        concat!(
            data_if_zero!(1, concat!(data_if_zero!(2, "<<<->>>", ""), "<<->>"), ""),
            "<->"
        ),
        ""
    ),
    "-"
);

const DATA_OPEN: &str = concat!(
    data_zero_count_single_from!(0),
    data_zero_count_single_from!(1),
    data_zero_count_single_from!(2),
    data_zero_count_single_from!(3), //count 0s in W1
    ">[<",                           //go to W1
);
const DATA_CLOSE: &str = concat!(
    data_zero_count_single_from!(0),
    data_zero_count_single_from!(1),
    data_zero_count_single_from!(2),
    data_zero_count_single_from!(3), //count 0s in W1
    ">]<",                           //go to W1
);

//move
use crate::ptr_move_working_to_data;
#[macro_export]
macro_rules! ptr_move_working_to_data {
    (0) => {
        ">>>>"
    };
    (1) => {
        ">>>"
    };
    (2) => {
        ">>"
    };
    (3) => {
        ">"
    };
}
use crate::ptr_move_data_to_working;
#[macro_export]
macro_rules! ptr_move_data_to_working {
    (0) => {
        "<<<<"
    };
    (1) => {
        "<<<"
    };
    (2) => {
        "<<"
    };
    (3) => {
        "<"
    };
}

use crate::ptr_move_D0_to_data;
#[macro_export]
macro_rules! ptr_move_D0_to_data {
    (0) => {
        ""
    };
    (1) => {
        "<"
    };
    (2) => {
        "<<"
    };
    (3) => {
        "<<<"
    };
}

use crate::ptr_move_data_to_D0;
#[macro_export]
macro_rules! ptr_move_data_to_D0 {
    (0) => {
        ""
    };
    (1) => {
        ">"
    };
    (2) => {
        ">>"
    };
    (3) => {
        ">>>"
    };
}

//----------------------the below macros assume the pointer position is W0 at the start, and will return the pointer to W0
//except i havent actually made this the case yet

//move Dx to W0
use crate::data_move_to_working;
#[macro_export]
macro_rules! data_move_to_working {
    ($offset:tt) => {
        concat!(
            "<<<<", //go to W0
            "[-]", //Zero W0
            ptr_move_working_to_data!($offset),
            "[",
            ptr_move_data_to_working!($offset),
            "+",
            ptr_move_working_to_data!($offset),
            "-]"
            ptr_move_data_to_D0!($offset),
        )
    };
}
//copy Dx to W0
use crate::data_copy_to_working;
#[macro_export]
macro_rules! data_copy_to_working {
    ($offset:tt) => {
        concat!(
            "<<<<", //go to W0
            "[-]",
            ">>>>>",
            "[-]",
            "<<<<<", //zero W0, W1, end in W0
            ptr_move_working_to_data!($offset),
            "[",
            ptr_move_data_to_working!($offset),
            "+",
            ">>>>>+<<<<<",
            ptr_move_working_to_data!($offset),
            "-]", //moves to W0 and W1
            ptr_move_data_to_working!($offset),
            ">>>>>", //go W1
            "[",
            "<<<<<",
            ptr_move_working_to_data!($offset),
            "+",
            ptr_move_data_to_working!($offset),
            ">>>>>-",
            "]", //moves from W1 back to Dx
            "<"  //go from W1 to D0
        )
    };
}

//move W0 to Dx
use crate::working_move_to_data;
#[macro_export]
macro_rules! working_move_to_data {
    ($offset:tt) => {
        concat!(
            ptr_move_D0_to_data!($offset),
            "[-]",
            ptr_move_data_to_working!($offset),
            "[",
            ptr_move_working_to_data!($offset),
            "+",
            ptr_move_data_to_working!($offset),
            "-]",
            ">>>>" //go to D0
        )
    };
}

//copy W0 to Dx
use crate::working_copy_to_data;
#[macro_export]
macro_rules! working_copy_to_data {
    ($offset:tt) => {
        concat!(
            ">",
            "[-]",
            "<", //Zero W1, go to D0
            ptr_move_D0_to_data!($offset),
            "[-]", //Zero Dx
            ptr_move_data_to_working($offset),
            "[",
            ">>>>>+<<<<<", //increment W1
            ptr_move_working_to_data!($offset),
            "+",
            ptr_move_data_to_working!($offset),
            "-",
            "]", //moves to W1 and Dx
            ">>>>>",
            "[<<<<<+>>>>>-]" //moves W1 to W0, finishes W1
            "<" //go to D0
        )
    };
}

//if statement applied on W0, args execeuted at D0
use crate::working_if_zero;
#[macro_export]
macro_rules! working_if_zero {
    ($zero:expr, $non_zero:expr) => {
        concat!(
            "<<<<",      //go to W0
            ">>>>>[-]",  //Zero W1
            ">>>>>[-]-", //W2=255
            "<<<<< <<<<<",
            "[",
            ">>>>", //go to D0
            $non_zero,
            ">", //go to W1
            "]",
            ">>>>>",
            "+",
            "[",
            "<", //go to D0 from W1
            $zero,
            "> >>>>> +", //go to W1 from D0, increment
            "]",
            "<<<<< <", //go to D0 from W2
        )
    };
}

//args executed at D0
use crate::data_if_zero;
#[macro_export]
macro_rules! data_if_zero {
    ($offset:tt, $zero:expr, $non_zero:expr) => {
        concat!(
            data_copy_to_working!($offset),
            working_if_zero!($zero, $non_zero)
        )
    };
}

use crate::data_zero_count_single_from;
#[macro_export]
macro_rules! data_zero_count_single_from {
    ($offset:tt) => {
        concat!(
            data_copy_to_working!($offset),
            ptr_move_D0_to_data!($offset),
            "[",
            ptr_move_data_to_D0!($offset),
            ">+<", //increment W1
            ptr_move_D0_to_data!($offset),
            "[-]]", //zero Dx, exit loop
            ptr_move_data_to_D0!($offset),
            working_move_to_data!($offset), //move W0 to Dx
        )
    };
}
