use crate::files::get_lines;
use crate::parser::parse;

pub fn run(file: String) {
    let lines = get_lines(&file);
    let result = parse(&lines);
}
