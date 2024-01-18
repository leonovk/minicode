use crate::files::get_lines;
use crate::parser::parse;
use crate::opcode::OpCode::*;

pub fn run(file: String) {
    let lines = get_lines(&file);
    let result = parse(&lines);

    for i in result {
        match i {
            Create(s) => println!("{}", s),
            Mov(s) => println!("{}", s)
        }
    }
}
