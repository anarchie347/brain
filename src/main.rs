use translators::brain32::{CodeBlock, Instruction, Mode};

mod translators;

fn main() {
    //let c = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
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
    println!("{}", s);
}
