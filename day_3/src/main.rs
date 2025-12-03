use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_sum: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut best_pair = 0;

        // Compare every digit with all digits after it
        for i in 0..digits.len() - 1 {
            for j in i + 1..digits.len() {
                let pair = digits[i] * 10 + digits[j];
                if pair > best_pair {
                    best_pair = pair;
                }
            }
        }

        println!("Line: {}, best pair: {}", line, best_pair);
        total_sum += best_pair;
    }

    println!("Total output joltage: {}", total_sum);
    Ok(())
}
