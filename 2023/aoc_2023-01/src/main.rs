use std::fs;

fn main() {
    let file_path = "input";
    let input = fs::read_to_string(file_path).unwrap();
    let numbers = input.lines().map(|line| {
        extract_numbers(line)
    });
    let mut number: usize;
    number = 42;
    number += 1;
    println!("{number}");
}

fn extract_numbers(line: &str) {
}
