use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Joltage {
    jol: u32,
    pos: usize,
}

fn main() {
    let file = File::open("input/input.txt").expect("Should have been able to open the file.");
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    let mut combined: String;
    let mut max1: u32 = 0;
    const MAX_DIGITS: i32 = 2;

    for line in reader.lines() {
        let nums_in_line: Vec<u32> = line
            .unwrap()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        for number in nums_in_line {
            if number > max1 {
                max1 = number;
            }
        }
    }
}
