pub fn translate(source : Vec<CodeBlock>) -> String {
    source.iter().flat_map(translate_block).collect()
}


#[derive (Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Mode {
    Data,Working,Raw
}
pub struct CodeBlock {
    pub mode : Mode,
    pub code: Vec<Instruction>
}
pub struct Instruction{
    pub parameter: u8,
    pub name: char
}

fn translate_block(block : &CodeBlock) -> Vec<char> {
    let mode = block.mode;
    let mut translated: Vec<char> = block.code.iter().flat_map(|i| translate_instruction(i, mode)).collect();

    if mode == Mode::Working { //W blocks require wrapping in <<< code >>> to move in/out of W memory
        let mut full = vec!['<'; 4];
        full.append(&mut translated);
        full.append(&mut vec!['>'; 4]);
        return full;      
    }
    translated
}

fn translate_instruction(i: &Instruction, mode : Mode) -> Vec<char> {
    (match mode {
        Mode::Data => translate_instruction_data,
        Mode::Working => translate_instruction_working,
        Mode::Raw => |x : &Instruction| -> Vec<char> {vec![x.name]}
    })(i)
}

fn translate_instruction_data(i: &Instruction) -> Vec<char> {
    String::from(
        match i.name {
        '+' => DATA_ADD,
        '-' => DATA_SUB,
        '[' => DATA_OPEN,
        ']' => DATA_CLOSE,
        ',' => ",",
        '.' => ".",
        '>' => ">>>>>",
        '<' => "<<<<<",
        _ => ""
    }).chars().collect()
}

fn translate_instruction_working(i: &Instruction) -> Vec<char> {
    String::from(
        match i.name {
            '+' => "+",
            '-' => "-",
            '[' => "[",
            ']' => "]",
            ',' => ",",
            '.' => ".",
            '>' => ">>>>>",
            '<' => "<<<<<",
            _ => ""
        }
    ).chars().collect()
}

// More complex substitution values

//basic data operations
const DATA_ADD: &str = "a";
const DATA_SUB: &str = "b";
const DATA_OPEN: &str = "c";
const DATA_CLOSE: &str = "d";

//move

//all start and end at D0, and move to W0
const fn substitution_data_move_from(cell: u8) -> &'static str {
    match cell {
        0 => "<<<<[-]>>>>[<<<<+>>>>-]",
        1 => "<<<<[-]>>>[<<<+>>>-]>",
        2 => "<<<<[-]>>[<<+>>-]>>",
        3 => "<<<<[-]>[<+>-]>>>",
        _ => panic!()
    }
}

macro_rules! gen_if_zero_working {
    ($zero:expr, $non_zero:expr) => {
        concat!(
            ">>>>>[-]",
            ">>>>>[-]-",
            "<<<<< <<<<<",
            "[",
                non_zero,
                ">>>>>",
            "]",
            ">>>>>",
            "+",
            "[",
                "<<<<<",
                zero,
                ">>>>> >>>>> +",
            "]",
            "<<<<< <<<<<",
        );
    };
}
//all start and end in W0, move to Dx
const fn substitution_working_move_to(cell: u8) -> &'static str {
    match cell {
        0 => ">>>>[-]<<<<[>>>>+<<<<-]",
        1 => ">>>[-]<<<[>>>+<<<-]",
        2 => ">>[-]<<[>>+<<-]",
        3 => ">[-]<[>+<-]",
        _ => panic!()
    }
}

