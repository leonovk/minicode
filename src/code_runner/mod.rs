use crate::files::get_lines;
use crate::interpreter::exegete;
use crate::parser::parse;

pub fn run(file: String, args: Vec<String>) {
    let lines = get_lines(&file);
    let result = parse(&lines);
    exegete(result, args);
}
