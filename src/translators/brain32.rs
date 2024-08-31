
pub fn translate(source : Vec<CodeBlock>) -> String {


    "".to_string()
}


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

fn translate_character(c: char, mode : Mode) -> String {
    (match mode {
        Mode::Data => translate_character_data,
        Mode::Working => translate_character_working,
        Mode::Raw => |x : char| -> String {String::from(x)}
    })(c)
}

fn translate_character_data(c: char) -> String {
    String::from(
        match c {
        '+' => data_add,
        '-' => data_sub,
        '[' => data_open,
        ']' => data_close,
        ',' => ",",
        '.' => ".",
        '>' => ">>>>",
        '<' => "<<<<",
        _ => ""
    })
}

fn translate_character_working(c: char) -> String {
    String::from("")
}



const data_add: &str = "a";
const data_sub: &str = "b";
const data_open: &str = "c";
const data_close: &str = "d";