use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(file_path: &String) -> Vec<String> {
    let mut result = Vec::new();
    let file = File::open(file_path).expect("err file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        result.push(line.expect("err line"))
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::files::get_lines;

    #[test]
    fn test_get_lines() {
        let path = "test/test_file.txt".to_string();
        assert_eq!(get_lines(&path), vec!["first".to_string(), "second".to_string()]);
    }
}
