use std::fs;

fn main() {
    let input =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file.");

    let ranges = input.split(',');
    let mut invalid_id_count = 0;
    for range in ranges {
        let limits: Vec<&str> = range.trim().split("-").collect();
        //println!("{:?}", limits)

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

    println!("{}", invalid_id_count)
}
