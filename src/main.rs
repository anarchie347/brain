use std::{env, io::Write};

use brainfuckinterpreter::run;
use translators::brain32::{CodeBlock, Mode, Token};

mod brainfuckinterpreter;
mod translators;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    //let source = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.-.";
    let source = "+?";

    let mut source_vec: Vec<(Mode, String)> = Vec::new();
    source_vec.push((Mode::Data, source.to_string()));
    let parsed = translators::brain32::parse(source_vec);
    let brainfuck = translators::brain32::translate(parsed);
    write_to_file(&brainfuck, &args[1]);
    //println!("{}", brainfuck);
    run(&brainfuck);
}

fn write_to_file(s: &String, path: &String) {
    let mut output = std::fs::File::create(path).unwrap();
    output.write_all(s.as_bytes());
}
