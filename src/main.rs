use std::{env, io::Write};

use translators::brain32::{CodeBlock, Instruction, Mode};

mod translators;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let c = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    //let c = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.-.";
    let c = "+";
    let block = CodeBlock {
        mode: Mode::Data,
        code: c
            .chars()
            .map(|c| Instruction {
                parameter: 0,
                name: c,
            })
            .collect(),
    };
    let mut v: Vec<CodeBlock> = Vec::new();
    v.push(block);
    let s = translators::brain32::translate(v);
    write_to_file(&s, &args[1]);
    println!("{}", s);
}

fn write_to_file(s: &String, path: &String) {
    let mut output = std::fs::File::create(path).unwrap();
    output.write_all(s.as_bytes());
}
