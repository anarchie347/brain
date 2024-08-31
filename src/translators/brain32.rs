pub fn translate(source : Vec<CodeBlock>) -> String {
    source.iter().flat_map(translate_block).collect()
}


#[derive (Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Data,Working,Raw
}
pub struct CodeBlock {
    mode : Mode,
    code: Vec<Instruction>
}
pub struct Instruction{
    parameter: u8,
    name: char
}

fn translate_block(block : &CodeBlock) -> Vec<char> {
    let mode = block.mode;
    let mut translated: Vec<char> = block.code.iter().flat_map(|i| translate_instruction(i, mode)).collect();

    if mode == Mode::Working { //W blocks require wrapping in <<< code >>> to move in/out of W memory
        let mut full = vec!['<'; 3];
        full.append(&mut translated);
        full.append(&mut vec!['>'; 3]);
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
        '>' => ">>>>",
        '<' => "<<<<",
        _ => ""
    }).chars().collect()
}

fn translate_instruction_working(_i: &Instruction) -> Vec<char> {
    String::from("").chars().collect()
}



const DATA_ADD: &str = "a";
const DATA_SUB: &str = "b";
const DATA_OPEN: &str = "c";
const DATA_CLOSE: &str = "d";