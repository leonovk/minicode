use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

pub fn get_lines(file_path: &String) -> Vec<String> {
    let mut result = Vec::new();
    let file = File::open(file_path).expect("err file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        result.push(line.expect("err line"));
    }

    result
}

pub fn get_content(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn write_content_to_file(content: &String, file_path: &String) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::files::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_lines() {
        let path = "tests/test_file.txt".to_string();
        assert_eq!(
            get_lines(&path),
            vec!["first".to_string(), "second".to_string()]
        );
    }

    #[test]
    fn test_get_content() {
        let path = "tests/test_file.txt".to_string();
        assert_eq!(get_content(&path), "first\nsecond".to_string());
    }
}
