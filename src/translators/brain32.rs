pub fn translate(source: Vec<CodeBlock>) -> String {
    source.iter().flat_map(translate_block).collect()
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
const DATA_ADD: &str = "a";
const DATA_SUB: &str = "b";
const DATA_OPEN: &str = "c";
const DATA_CLOSE: &str = "d";
const ZERO: &str = "[-]";

//move

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
    ($x:expr) => {
        compile_error!("arg must be 0,1,2 or 3")
    };
}
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
    ($x:expr) => {
        compile_error!("arg must be 0,1,2 or 3")
    };
}

//----------------------the below macros assume the pointer position is W0 at the start, and will return the pointer to W0
//except i havent actually made this the case yet

//move Dx to W0
macro_rules! data_move_from {
    ($offset:expr) => {
        concat!(
            "<<<<", //go to W0
            ZERO,
            ptr_move_to_data!(offset),
            "[",
            ptr_move_to_working!(offset),
            "+",
            ptr_move_to_data!(offset),
            "-]"
            ptr_move_to_working(offset),
            ">>>>" // go to D0
        )
    };
}
//copy Dx to W0
macro_rules! data_copy_from {
    ($offset:expr) => {
        concat!(
            "<<<<", //go to W0
            ZERO,
            ">>>>>",
            ZERO,
            "<<<<<"
            ptr_move_to_data!(offset),
            "[",
            ptr_move_to_working!(offset),
            "+",
            ">>>>>+<<<<<"
            ptr_move_to_data!(offset),
            "-]", //moves to W0 and W1
            ptr_move_to_working(offset),
            ">>>>>",
            "[",
            "<<<<<",
            ptr_move_to_data(offset),
            "+",
            ptr_move_to_working(offset),
            ">>>>>-",
            "]" //moves from W1 back to Dx
            "<" //go from W1 to D0
        )
    }
}

//move W0 to Dx
macro_rules! working_move_to {
    ($offset:expr) => {
        concat!(
            ZERO,
            ptr_move_to_working(offset),
            "[",
            ptr_move_to_data(offset),
            "+",
            ptr_move_to_working(offset),
            "-]",
            ">>>>" //go to W0
        )
    };
}

//copy W0 to Dx
macro_rules! working_copy_to {
    ($offset:expr) => {
        concat!(
            ">",
            ZERO,
            "<", //Zero W1, go to D0
            ZERO, //Zero D0
            "<<<<" //go to W0
            "[",
            ">>>>>+<<<<<"
            ptr_move_to_data(offset),
            "+",
            ptr_move_to_working(offset),
            "]", //moves to W1 and Dx
            ">>>>>",
            "[<<<<<+>>>>>-]" //moves W1 to W0, finishes W1
            "<" //go to D0
        )
    };
}

//if statement applied on W0, args execeuted at D0
macro_rules! working_if_zero {
    ($zero:expr, $non_zero:expr) => {
        concat!(
            "<<<<" //go to W0
            ">>>>>[-]", //Zero W1
            ">>>>>[-]-", //W2=255
            "<<<<< <<<<<",
            "[",
            ">>>>", //go to D0
            non_zero,
            ">", //go to W1
            "]",
            ">>>>>",
            "+",
            "[",
            "<", //go to D0 from W1
            zero,
            "> >>>>> +", //go to W1 from D0, increment
            "]",
            "<<<<< <", //go to D0 from W2
        );
    };
}

macro_rules! data_if_zero {
    ($offset:expr) => {
        concat!(data_copy_from(offset),)
    };
}
