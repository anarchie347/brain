use std::str::Chars;


pub fn translate(source : Vec<CodeBlock>) -> String {
    let result = String::new();
    source.iter().flat_map(translate_block).collect()
}


#[derive (Clone, Copy)]
enum Mode {
    Data,Working,Raw
}
struct CodeBlock {
    mode : Mode,
    code: Vec<Instruction>
}
struct Instruction{
    parameter: u8,
    name: char
}

fn translate_block(block : &CodeBlock) -> Vec<char> {
    let mode = block.mode;
    block.code.iter().flat_map(|i| translate_instruction(i, mode)).collect()
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
        '>' => ">>>>",
        '<' => "<<<<",
        _ => ""
    }).chars().collect()
}

fn translate_instruction_working(i: &Instruction) -> Vec<char> {
    String::from("").chars().collect()
}



const DATA_ADD: &str = "a";
const DATA_SUB: &str = "b";
const DATA_OPEN: &str = "c";
const DATA_CLOSE: &str = "d";