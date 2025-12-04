use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input/input.txt").expect("Should have been able to open file.");
    let reader = BufReader::new(file);
    let mut grid_2d: Vec<Vec<char>> = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        println!("{:?}", line);

        let row: Vec<char> = line.chars().collect();
        grid_2d.push(row);
    }

    println!("{:?}", grid_2d);

    Ok(())
}
