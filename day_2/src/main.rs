use std::fs;

fn main() {
    let input =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file.");
    part_one(&input);
    part_two(&input);
}

fn part_two(input: &String) {
    let mut sum = 0;
    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let start: i64 = a.parse().unwrap();
        let end: i64 = b.parse().unwrap();
        for num in start..=end {
            let s = num.to_string();
            for pattern_len in 1..=s.len() / 2 {
                if !s.len().is_multiple_of(pattern_len) {
                    continue;
                }
                let pattern = &s[0..pattern_len];
                if (pattern_len..s.len())
                    .step_by(pattern_len)
                    .all(|start_idx| &s[start_idx..start_idx + pattern_len] == pattern)
                {
                    sum += num;
                    break;
                }
            }
        }
    }
    println!("The solution to part two is: {}", sum)
}

fn part_one(input: &String) {
    let ranges = input.split(',');
    let mut invalid_id_count = 0;
    for range in ranges {
        let limits: Vec<&str> = range.trim().split("-").collect();

        let upper_limit: usize = limits[0].parse().unwrap();
        let lower_limit: usize = limits[1].parse().unwrap();

        for number in upper_limit..=lower_limit {
            let id = number.to_string();
            if id.len() % 2 != 0 {
                continue;
            }
            let (left, right) = id.split_at(id.len() / 2);
            if left == right {
                invalid_id_count += number;
            }
        }
    }

    println!("The solution for part one is: {}", invalid_id_count)
}
