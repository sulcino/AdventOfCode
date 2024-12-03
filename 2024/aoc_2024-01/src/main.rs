use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let left: i32 = parts[0].parse().unwrap();
            let right: i32 = parts[1].parse().unwrap();

            left_nums.push(left);
            right_nums.push(right);
        }
    }

    left_nums.sort();
    right_nums.sort();

    let mut sum_of_differences = 0;

    for (left, right) in left_nums.iter().zip(right_nums.iter()) {
        let difference = (right - left).abs();
        sum_of_differences += difference;
    }

    println!("Sum of differences: {}", sum_of_differences);

    Ok(())
}
