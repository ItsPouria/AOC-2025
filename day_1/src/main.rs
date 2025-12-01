use std::fs;

fn main() {
    let mut position: i32 = 50;
    let max_value = 100;
    let mut zero_position = 0;
    let instructions =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file.");
    for line in instructions.lines() {
        let direction = &line[0..1];
        let number: i32 = line[1..].parse().expect("Expected digits.");

        if direction == "L" {
            position = (position - number).rem_euclid(max_value);
        } else {
            position = (position + number).rem_euclid(max_value);
        }

        if position == 0 {
            zero_position += 1;
        }
    }

    println!("The zero position was met {} times", zero_position)
}
