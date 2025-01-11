pub fn parse(source: String) -> Vec<Token> {
    source
        .chars()
        .map(|c| match c {
            '+' => Token::Plus(None),
            '-' => Token::Minus(None),
            '>' => Token::Right,
            '<' => Token::Left,
            ',' => Token::Comma(SingleArg::Zero),
            '.' => Token::Dot(SingleArg::Zero),
            '[' => Token::Open(Vec::new()),
            ']' => Token::Close(Vec::new()),
        })
        .collect()
}

pub fn translate(source: Vec<Token>) -> String {
    source
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
        })
        .collect()
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
