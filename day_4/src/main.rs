use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    isize, usize,
};

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 1),   //up-left
    (1, 0),   //up
    (1, -1),  //up-right
    (0, 1),   //left
    (0, -1),  //right
    (-1, 1),  //bottom-left
    (-1, 0),  //bottom
    (-1, -1), //bottom-right
];

fn main() -> io::Result<()> {
    let file = File::open("input/input.txt").expect("Should have been able to open file.");
    let reader = BufReader::new(file);
    let mut grid_2d: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        let row: Vec<char> = line.chars().collect();
        grid_2d.push(row);
    }

    check_neighbors(&grid_2d);
    Ok(())
}

fn check_neighbors(grid: &Vec<Vec<char>>) {
    let mut total_rolls_removed = 0;
    //println!("{:?}", grid.len());

    for i in 0..grid.len() {
        println!("row {} ", i);
        for (j, char) in grid[i].iter().enumerate() {
            //println!("col {} has char {}", j, char)

            if *char == '@' {
                let mut adjacent_roll_count = 0;
                for (dr, dc) in DIRECTIONS.iter() {
                    let nr = i as i32 + dr;
                    let nc = j as i32 + dc;

                    if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[i].len() as i32 {
                        let neighbor_char = grid[nr as usize][nc as usize];

                        println!("Neighbor of @ at ({}, {}): {}", i, j, neighbor_char);
                        if neighbor_char == '@' {
                            adjacent_roll_count += 1;
                        }
                    }
                }
                if adjacent_roll_count < 4 {
                    total_rolls_removed += 1;
                }
            }
        }
    }

    println!("total @ in grid {}", total_rolls_removed)
}
