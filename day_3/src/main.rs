use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Pick exactly `target_len` digits from `line` (in order) to form the largest possible number.
/// Returns None if the line has fewer than `target_len` digits.
fn pick_largest_subsequence_number(line: &str, target_len: usize) -> Option<u128> {
    // Convert the line into digits (ignore any non-digit characters).
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    let total_len = digits.len();
    if total_len < target_len {
        return None;
    }

    let mut result_digits: Vec<u8> = Vec::with_capacity(target_len);

    // Where we start looking for the next digit
    let mut start_index: usize = 0;

    // We will pick `target_len` digits, one by one
    for already_picked_count in 0..target_len {
        // How many digits are still needed including this one?
        let still_needed = target_len - already_picked_count;

        // The furthest index we can look at now without running out later.
        let last_allowed_index = total_len - still_needed;

        // Search from start_index..=last_allowed_index for the largest digit.
        let mut best_digit: u8 = 0;
        let mut best_index: usize = start_index;

        let mut i = start_index;
        while i <= last_allowed_index {
            let d = digits[i];
            if d > best_digit {
                best_digit = d;
                best_index = i;
                if d == 9 {
                    // 9 is the best possible digit; no need to keep searching this window.
                    break;
                }
            }
            i += 1;
        }

        // Record the chosen digit and advance the start for the next search
        result_digits.push(best_digit);
        start_index = best_index + 1;
    }

    // Turn the chosen digits into a number
    let mut value: u128 = 0;
    for d in result_digits {
        value = value * 10 + d as u128;
    }
    Some(value)
}

fn main() -> io::Result<()> {
    // Part 2: we must pick exactly 12 digits per line.
    let target_len = 12;

    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_sum: u128 = 0;

    for (line_no, line_res) in reader.lines().enumerate() {
        let line = line_res?;
        match pick_largest_subsequence_number(&line, target_len) {
            Some(best) => {
                println!(
                    "Line {}: best {}-digit joltage: {}",
                    line_no + 1,
                    target_len,
                    best
                );
                total_sum += best;
            }
            None => {
                eprintln!(
                    "Line {} has fewer than {} digits; skipping.",
                    line_no + 1,
                    target_len
                );
            }
        }
    }

    println!("Total output joltage: {}", total_sum);
    Ok(())
}
