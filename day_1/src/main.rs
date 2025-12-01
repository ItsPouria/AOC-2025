use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Failed to read input file");
    let rotations: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let mut position: i32 = 50;
    let dial_size: i32 = 100;
    let mut count_zero: i64 = 0;

    for rotation in rotations {
        let direction = &rotation[0..1];
        let steps: i32 = rotation[1..].parse().unwrap();

        let increment = if direction == "R" { 1 } else { -1 };

        for _ in 0..steps {
            position = (position + increment).rem_euclid(dial_size);
            if position == 0 {
                count_zero += 1;
            }
        }
    }

    println!("Password using method 0x434C49434B: {}", count_zero);
}
