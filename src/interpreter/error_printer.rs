pub fn print_error(file: &String, line: usize, error: &String) {
    println!("File -> {}", file);
    println!("Line # {}", line);
    println!("Error: {}", error);
}
