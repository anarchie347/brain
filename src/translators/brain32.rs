pub fn parse(source: String) -> Vec<Token> {
    source
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Token::Plus(None, false)),
            '-' => Some(Token::Minus(None, false)),
            '>' => Some(Token::Right),
            '<' => Some(Token::Left),
            ',' => Some(Token::Comma(None)),
            '.' => Some(Token::Dot(None)),
            '[' => Some(Token::Open(Vec::new())),
            ']' => Some(Token::Close(Vec::new())),
            ':' => Some(Token::DebugColon),
            _ => None,
        })
        .collect()
}

pub fn translate(parsed: Vec<Token>) -> String {
    String::from(">>>>>") //Have to prepend with this because translations expect the pointer to be at W after first D cells (Tape starts W D D D D W ... and poijnter needs to be at 2nd W), this is index 5
        + &(parsed
            .iter()
            .map(|t| match t {
                Token::Plus(arg, no_carry) => handle_add(arg, no_carry),
                Token::Minus(arg, no_carry) => handle_sub(arg, no_carry),
                Token::Right => TRANSLATION_RIGHT,
                Token::Left => TRANSLATION_LEFT,
                Token::Comma(arg) => handle_com(arg),
                Token::Dot(arg) => handle_dot(arg),
                Token::Open(args) => handle_open(args),
                Token::Close(args) => handle_close(args),
                Token::DebugColon => ":"
            })
            .collect::<String>())
}

pub enum Token {
    Plus(Option<SingleArg>, bool),
    Minus(Option<SingleArg>, bool),
    Right,
    Left,
    Comma(Option<SingleArg>),
    Dot(Option<SingleArg>),
    Open(Vec<SingleArg>),
    Close(Vec<SingleArg>),
    DebugColon,
}

pub enum SingleArg {
    Zero,
    One,
    Two,
    Three,
}

fn handle_add(arg : &Option<SingleArg>, no_carry : &bool) -> &'static str {
    //nocarry
    if *no_carry {
        return match arg {
            None => TRANSLATION_ADD_0_NOC,
            Some(a) => match a {
                SingleArg::Zero => TRANSLATION_ADD_0_NOC,
                SingleArg::One => TRANSLATION_ADD_1_NOC,
                SingleArg::Two => TRANSLATION_ADD_2_NOC,
                SingleArg::Three => TRANSLATION_ADD_3_NOC
            }
        }
    }

    //carry
    match arg {
        None => TRANSLATION_ADD_FULL,
        Some(a) => match a {
            SingleArg::Zero => TRANSLATION_ADD_0,
            SingleArg::One => TRANSLATION_ADD_1,
            SingleArg::Two => TRANSLATION_ADD_2,
            SingleArg::Three => TRANSLATION_ADD_3
        }
    }
}
fn handle_sub(arg : &Option<SingleArg>, no_carry : &bool) -> &'static str {
    if *no_carry {
        return match arg {
            None => TRANSLATION_SUB_0_NOC,
            Some(a) => match a {
                SingleArg::Zero => TRANSLATION_SUB_0_NOC,
                SingleArg::One => TRANSLATION_SUB_1_NOC,
                SingleArg::Two => TRANSLATION_SUB_2_NOC,
                SingleArg::Three => TRANSLATION_SUB_3_NOC
            }
        }
    }

    //carry
    match arg {
        None => TRANSLATION_SUB_FULL,
        Some(a) => match a {
            SingleArg::Zero => TRANSLATION_SUB_0,
            SingleArg::One => TRANSLATION_SUB_1,
            SingleArg::Two => TRANSLATION_SUB_2,
            SingleArg::Three => TRANSLATION_SUB_3
        }
    }
}

fn handle_com(arg : &Option<SingleArg>) -> &'static str{
    match arg {
        None => TRANSLATION_COM_0,
        Some(a) => match a {
            SingleArg::Zero => TRANSLATION_COM_0,
            SingleArg::One => TRANSLATION_COM_1,
            SingleArg::Two => TRANSLATION_COM_2,
            SingleArg::Three => TRANSLATION_COM_3
        }
    }
}

fn handle_dot(arg : &Option<SingleArg>) -> &'static str{
    match arg {
        None => TRANSLATION_DOT_0,
        Some(a) => match a {
            SingleArg::Zero => TRANSLATION_DOT_0,
            SingleArg::One => TRANSLATION_DOT_1,
            SingleArg::Two => TRANSLATION_DOT_2,
            SingleArg::Three => TRANSLATION_DOT_3
        }
    }
}

fn handle_open(args : &Vec<SingleArg>) -> &'static str {
    let mut has3 = false;
    let mut has2 = false;
    let mut has1 = false;
    let mut has0 = false;
    for a in args {
        match a {
            SingleArg::Three => has3 = true,
            SingleArg::Two => has2 = true,
            SingleArg::One => has1 = true,
            SingleArg::Zero => has0 = true
        };
    }
    match (has3,has2,has1,has0) {
        (true, true, true, true) => TRANSLATION_OPEN_3210,
        (true, true, true, false) => TRANSLATION_OPEN_321,
        (true, true, false, true) => TRANSLATION_OPEN_320,
        (true, true, false, false) => TRANSLATION_OPEN_32,
        (true, false, true, true) => TRANSLATION_OPEN_310,
        (true, false, true, false) => TRANSLATION_OPEN_31,
        (true, false, false, true) => TRANSLATION_OPEN_30,
        (true, false, false, false) => TRANSLATION_OPEN_3,
        (false, true, true, true) => TRANSLATION_OPEN_210,
        (false, true, true, false) => TRANSLATION_OPEN_21,
        (false, true, false, true) => TRANSLATION_OPEN_20,
        (false, true, false, false) => TRANSLATION_OPEN_2,
        (false, false, true, true) => TRANSLATION_OPEN_10,
        (false, false, true, false) => TRANSLATION_OPEN_1,
        (false, false, false, true) => TRANSLATION_OPEN_0,
        (false, false, false, false) => TRANSLATION_OPEN_FULL

    }
}

fn handle_close(args : &Vec<SingleArg>) -> &'static str {
    let mut has3 = false;
    let mut has2 = false;
    let mut has1 = false;
    let mut has0 = false;
    for a in args {
        match a {
            SingleArg::Three => has3 = true,
            SingleArg::Two => has2 = true,
            SingleArg::One => has1 = true,
            SingleArg::Zero => has0 = true
        };
    }
    match (has3,has2,has1,has0) {
        (true, true, true, true) => TRANSLATION_CLOSE_3210,
        (true, true, true, false) => TRANSLATION_CLOSE_321,
        (true, true, false, true) => TRANSLATION_CLOSE_320,
        (true, true, false, false) => TRANSLATION_CLOSE_32,
        (true, false, true, true) => TRANSLATION_CLOSE_310,
        (true, false, true, false) => TRANSLATION_CLOSE_31,
        (true, false, false, true) => TRANSLATION_CLOSE_30,
        (true, false, false, false) => TRANSLATION_CLOSE_3,
        (false, true, true, true) => TRANSLATION_CLOSE_210,
        (false, true, true, false) => TRANSLATION_CLOSE_21,
        (false, true, false, true) => TRANSLATION_CLOSE_20,
        (false, true, false, false) => TRANSLATION_CLOSE_2,
        (false, false, true, true) => TRANSLATION_CLOSE_10,
        (false, false, true, false) => TRANSLATION_CLOSE_1,
        (false, false, false, true) => TRANSLATION_CLOSE_0,
        (false, false, false, false) => TRANSLATION_CLOSE_FULL

    }
}

// Unless otherwise annotated, the following hardcoded conversions are generated using the following python scripts, the algorithms used are explained in the python source code: https://github.com/anarchie347/brain-translation-generation-scripts

//integers at the end of the constant names refer to the column, so ADD_1 means +1 to the (256^1) column, i.e. add 256

const TRANSLATION_ADD_FULL: &str = "<+<+<+<+<[-]+>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<-<-<-<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<[>>-<-<[-]]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>-<[-]]>>>>>[-]]";
const TRANSLATION_ADD_0: &str = TRANSLATION_ADD_FULL;
const TRANSLATION_ADD_1: &str = "<<+<+<+<[-]+>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<-<-<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>-<[-]]>>>>>[-]]";
const TRANSLATION_ADD_2: &str = "<<<+<+>>>>[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<->>>>]";
const TRANSLATION_ADD_3: &str = "<<<<+>>>>"; // simple, no carry logic needed. Script not used

//No carry, just add to the individual cell
// simple so not script-generated
const TRANSLATION_ADD_0_NOC: &str = "<+>";
const TRANSLATION_ADD_1_NOC: &str = "<<+>>";
const TRANSLATION_ADD_2_NOC: &str = "<<<+>>>";
const TRANSLATION_ADD_3_NOC: &str = "<<<<+>>>>"; //same as regular add3


const TRANSLATION_SUB_FULL: &str = "<<<<<[-]+>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<+<+<+<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<[>>+<+<[-]]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>+<[-]]>>>>>[-]]<-<-<-<->>>>";
const TRANSLATION_SUB_0: &str = TRANSLATION_SUB_FULL;
const TRANSLATION_SUB_1: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<+<+<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>+<[-]]>>>>>[-]]<<-<-<->>>>";
const TRANSLATION_SUB_2: &str = "[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<+>>>>[-]]<<<-<-";
const TRANSLATION_SUB_3: &str = "<<<<->>>>"; // simple, no carry logic needed. Script not used

//No carry, just sub from the individual cell
//simple so not script generated
const TRANSLATION_SUB_0_NOC: &str = "<->";
const TRANSLATION_SUB_1_NOC: &str = "<<->>";
const TRANSLATION_SUB_2_NOC: &str = "<<<->>>";
const TRANSLATION_SUB_3_NOC: &str = "<<<<->>>>"; //same as regular sub3

// dot and comma not script generated as very simple
const TRANSLATION_COM_0: &str = "<,>";
const TRANSLATION_COM_1: &str = "<<,>>";
const TRANSLATION_COM_2: &str = "<<<,>>>";
const TRANSLATION_COM_3: &str = "<<<<,>>>>";

const TRANSLATION_DOT_0: &str = "<.>";
const TRANSLATION_DOT_1: &str = "<<.>>";
const TRANSLATION_DOT_2: &str = "<<<.>>>";
const TRANSLATION_DOT_3: &str = "<<<<.>>>>";


const TRANSLATION_RIGHT: &str = ">>>>>"; //cells are grouped in blocks of 5

const TRANSLATION_LEFT: &str = "<<<<<"; //cells are grouped in blocks of 5

const TRANSLATION_OPEN_FULL: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";

const TRANSLATION_CLOSE_FULL: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";

//all possible cases of cells to include for brackets
const TRANSLATION_OPEN_3210: &str = TRANSLATION_OPEN_FULL;
const TRANSLATION_OPEN_321: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_320: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_32: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_310: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_31: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_30: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_3: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_210: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_21: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_20: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_2: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_10: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_1: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";
const TRANSLATION_OPEN_0: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";

const TRANSLATION_CLOSE_3210: &str = TRANSLATION_CLOSE_FULL;
const TRANSLATION_CLOSE_321: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_320: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_32: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_310: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_31: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_30: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_3: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_210: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_21: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_20: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_2: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_10: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_1: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
const TRANSLATION_CLOSE_0: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";