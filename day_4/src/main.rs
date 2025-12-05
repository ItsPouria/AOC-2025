use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 1),   // up-left
    (1, 0),   // up
    (1, -1),  // up-right
    (0, 1),   // left
    (0, -1),  // right
    (-1, 1),  // bottom-left
    (-1, 0),  // bottom
    (-1, -1), // bottom-right
];

fn main() -> io::Result<()> {
    let file = File::open("input/input.txt").expect("Should have been able to open file.");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        grid.push(line.chars().collect());
    }

    let (final_grid, total_removed, iterations) = remove_iteratively(grid);

    println!("Finished after {} iterations.", iterations);
    println!("Total '@'s removed: {}", total_removed);
    println!("Final grid:");
    for row in final_grid {
        println!("{}", row.into_iter().collect::<String>());
    }

    Ok(())
}
fn remove_iteratively(mut grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize, usize) {
    let mut total_removed = 0usize;
    let mut iterations = 0usize;

    loop {
        let (next, removed) = one_pass(&grid);
        iterations += 1;

        if removed == 0 {
            break;
        }

        total_removed += removed;
        grid = next;
    }

    (grid, total_removed, iterations)
}
fn one_pass(grid: &[Vec<char>]) -> (Vec<Vec<char>>, usize) {
    let mut next = grid.to_vec();
    let mut removed_this_pass = 0usize;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                let count = count_adjacent_ats(grid, i, j);
                if count < 4 {
                    next[i][j] = 'X';
                    removed_this_pass += 1;
                }
            }
        }
    }

    (next, removed_this_pass)
}

fn count_adjacent_ats(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut count = 0usize;

    for (dr, dc) in DIRECTIONS.iter() {
        let nr = i as i32 + dr;
        let nc = j as i32 + dc;

        if nr >= 0
            && (nr as usize) < grid.len()
            && nc >= 0
            && (nc as usize) < grid[nr as usize].len()
        {
            if grid[nr as usize][nc as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}
