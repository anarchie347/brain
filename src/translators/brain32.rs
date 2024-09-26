pub fn translate(source: Vec<CodeBlock>) -> String {
    let mut full = vec!['>'; 4]; //wraps code in arrows to go to D0 to start,
    full.append(&mut source.iter().flat_map(translate_block).collect());
    full.append(&mut vec!['<'; 4]);
    full.iter().collect()
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
    pub code: Vec<Instruction>,
}
pub struct Instruction {
    pub parameter: u8,
    pub name: char,
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

fn translate_instruction(i: &Instruction, mode: Mode) -> Vec<char> {
    (match mode {
        Mode::Data => translate_instruction_data,
        Mode::Working => translate_instruction_working,
        Mode::Raw => |x: &Instruction| -> Vec<char> { vec![x.name] },
    })(i)
}

fn translate_instruction_data(i: &Instruction) -> Vec<char> {
    String::from(match i.name {
        '+' => DATA_ADD,
        '-' => DATA_SUB,
        '[' => DATA_OPEN,
        ']' => DATA_CLOSE,
        ',' => ",",
        '.' => ".",
        '>' => ">>>>>",
        '<' => "<<<<<",
        _ => "",
    })
    .chars()
    .collect()
}

fn translate_instruction_working(i: &Instruction) -> Vec<char> {
    String::from(match i.name {
        '+' => "+",
        '-' => "-",
        '[' => "[",
        ']' => "]",
        ',' => ",",
        '.' => ".",
        '>' => ">>>>>",
        '<' => "<<<<<",
        _ => "",
    })
    .chars()
    .collect()
}

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
use crate::ptr_move_to_working;
#[macro_export]
macro_rules! ptr_move_to_working {
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
use crate::ptr_move_to_data;
#[macro_export]
macro_rules! ptr_move_to_data {
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

//----------------------the below macros assume the pointer position is W0 at the start, and will return the pointer to W0
//except i havent actually made this the case yet

//move Dx to W0
use crate::data_move_from;
#[macro_export]
macro_rules! data_move_from {
    ($offset:tt) => {
        concat!(
            "<<<<", //go to W0
            "[-]", //Zero W0
            ptr_move_to_data!($offset),
            "[",
            ptr_move_to_working!($offset),
            "+",
            ptr_move_to_data!($offset),
            "-]"
            ptr_move_to_working!($offset),
            ">>>>" // go to D0
        )
    };
}
//copy Dx to W0
use crate::data_copy_from;
#[macro_export]
macro_rules! data_copy_from {
    ($offset:tt) => {
        concat!(
            "<<<<", //go to W0
            "[-]",
            ">>>>>",
            "[-]",
            "<<<<<", //zero W0, W1, end in W0
            ptr_move_to_data!($offset),
            "[",
            ptr_move_to_working!($offset),
            "+",
            ">>>>>+<<<<<",
            ptr_move_to_data!($offset),
            "-]", //moves to W0 and W1
            ptr_move_to_working!($offset),
            ">>>>>",
            "[",
            "<<<<<",
            ptr_move_to_data!($offset),
            "+",
            ptr_move_to_working!($offset),
            ">>>>>-",
            "]", //moves from W1 back to Dx
            "<"  //go from W1 to D0
        )
    };
}

//move W0 to Dx
use crate::working_move_to;
#[macro_export]
macro_rules! working_move_to {
    ($offset:tt) => {
        concat!(
            "[-]",
            ptr_move_to_working!($offset),
            "[",
            ptr_move_to_data!($offset),
            "+",
            ptr_move_to_working!($offset),
            "-]",
            ">>>>" //go to W0
        )
    };
}

//copy W0 to Dx
use crate::working_copy_to;
#[macro_export]
macro_rules! working_copy_to {
    ($offset:tt) => {
        concat!(
            ">",
            "[-]",
            "<", //Zero W1, go to D0
            "[-]", //Zero D0
            "<<<<" //go to W0
            "[",
            ">>>>>+<<<<<"
            ptr_move_to_data!($offset),
            "+",
            ptr_move_to_working!($offset),
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
        concat!(data_copy_from!($offset), working_if_zero!($zero, $non_zero))
    };
}

use crate::data_zero_count_single_from;
#[macro_export]
macro_rules! data_zero_count_single_from {
    ($offset:tt) => {
        concat!(
            data_copy_from!($offset),
            "<<<<",
            ptr_move_to_data!($offset), //go to Dx
            "[",
            ptr_move_to_data!($offset),
            ">>>>>+<<<<<", //increment W1
            ptr_move_to_working!($offset),
            "[-]", //zero Dx
            ptr_move_to_data!($offset),
            ">>>>",                    //go back to D0
            working_move_to!($offset), //move W0 to Dx
        )
    };
}
