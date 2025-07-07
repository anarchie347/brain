use std::{env, io::Write};

use brainfuckinterpreter::run;

use crate::translators::{brain32::{self, translate}, brain3U::{self, TokenType}};

mod brainfuckinterpreter;
mod translators;

fn main() {

    let brainfuck = brain32::translate(brain3U::translate(vec![brain3U::Call(brain3U::Zero)]));
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
