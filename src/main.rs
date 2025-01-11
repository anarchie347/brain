use std::{env, io::Write};

use brainfuckinterpreter::run;

mod brainfuckinterpreter;
mod translators;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    //let source = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.-.";
    //let source = "+#@";

    let source = read_from_file(&String::from("in.txt"));

    let parsed = translators::brain32::parse(source);
    let brainfuck = translators::brain32::translate(parsed);
    write_to_file(&brainfuck, &String::from("out.txt"));
    run(&brainfuck);
}

fn write_to_file(s: &String, path: &String) {
    let mut output = std::fs::File::create(path).unwrap();
    output.write_all(s.as_bytes());
}

fn read_from_file(path: &String) -> String {
    let contents = std::fs::read(path).expect("Coundlt read file");
    String::from_utf8(contents).expect("Couldnt convert file contents to string")
}
