use std::{env, io::Write};

use brainfuckinterpreter::run;
use translators::brain32::{CodeBlock, Mode, Token};

mod brainfuckinterpreter;
mod translators;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    //let source = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.-.";
    //let source = "+#@";
    let source = read_from_file(&String::from("in.txt"));

    let mut source_vec: Vec<(Mode, String)> = Vec::new();
    source_vec.push((Mode::Raw, String::from_utf8(vec![b'+'; 255]).unwrap()));
    source_vec.push((Mode::Data, source.to_string()));
    let parsed = translators::brain32::parse(source_vec);
    let brainfuck = translators::brain32::translate(parsed);
    write_to_file(&brainfuck, &String::from("out.txt"));
    //println!("{}", brainfuck);
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
