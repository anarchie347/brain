pub fn parse(source: String) -> Vec<Token> {
    source
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Token::Plus(None)),
            '-' => Some(Token::Minus(None)),
            '>' => Some(Token::Right),
            '<' => Some(Token::Left),
            ',' => Some(Token::Comma(SingleArg::Zero)),
            '.' => Some(Token::Dot(SingleArg::Zero)),
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
                Token::Plus(_) => TRANSLATION_ADD_FULL,
                Token::Minus(_) => TRANSLATION_SUB_FULL,
                Token::Right => TRANSLATION_RIGHT,
                Token::Left => TRANSLATION_LEFT,
                Token::Comma(_) => "<,>",
                Token::Dot(_) => "<.>",
                Token::Open(_) => TRANSLATION_OPEN_FULL,
                Token::Close(_) => TRANSLATION_CLOSE_FULL,
                Token::DebugColon => ":"
            })
            .collect::<String>())
}

pub enum Token {
    Plus(Option<SingleArg>),
    Minus(Option<SingleArg>),
    Right,
    Left,
    Comma(SingleArg),
    Dot(SingleArg),
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

// Unless otherwise annotated, the following hardcoded conversions are generated using the following python scripts, the algorithms used are explained in the python source code: https://github.com/anarchie347/brain-translation-generation-scripts

const TRANSLATION_ADD_FULL: &str = "<+<+<+<+<[-]+>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<-<-<-<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<[>>-<-<[-]]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>-<[-]]>>>>>[-]]";

const TRANSLATION_SUB_FULL: &str = "<<<<<[-]+>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<+<+<+<[-]>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<[>>+<+<[-]]>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<[>+<[-]]>>>>>[-]]<-<-<-<->>>>";

const TRANSLATION_RIGHT: &str = ">>>>>"; //cells are grouped in blocks of 5

const TRANSLATION_LEFT: &str = "<<<<<"; //cells are grouped in blocks of 5

const TRANSLATION_OPEN_FULL: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<[>>>>>";

const TRANSLATION_CLOSE_FULL: &str = "<<<<<[-]>>>>>[-]>>>>>[-]<<<<<<[>+>>>>>+<<<<<<-]>>>>>>[<<<<<<+>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<[>>+>>>>>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<[>>>+>>>>>+<<<<<<<<-]>>>>>>>>[<<<<<<<<+>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]][-]>>>>>[-]<<<<<<<<<[>>>>+>>>>>+<<<<<<<<<-]>>>>>>>>>[<<<<<<<<<+>>>>>>>>>-]<<<<<[<<<<<+>>>>>[-]]<<<<<]>>>>>";
