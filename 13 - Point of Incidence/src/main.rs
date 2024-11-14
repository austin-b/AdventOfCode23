// https://adventofcode.com/2023/day/13

use std::fs;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum LavaIsland {
    Ash,
    Rock,
}

fn main() {
    
    // EXAMPLE 1
    let vertical_example = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
    // EXAMPLE 2
    let horizontal_example = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
    // PUZZLE
    // let puzzle: String = fs::read_to_string("src/input.txt")
    //     .expect("Something went wrong reading the file");
    let contents = vertical_example;

    let patterns: Vec<Vec<LavaIsland>> = contents.lines().map(|line: &str| {
        line.chars().map(|c| {
            match c {
                '#' => LavaIsland::Rock,
                '.' => LavaIsland::Ash,
                _ => panic!("Invalid character"),
            }
        }).collect()
    }).collect();

    println!("patterns: {:?}", patterns);
    println!("vertical symmetry: {:?}", check_vertical_symmetry(&patterns));
}

// check first row
// if symmetry is found, check the same row going down
// if found all the way, return the number of columns to the left of each vertical line of reflection
fn check_vertical_symmetry(pattern: &Vec<Vec<LavaIsland>>) -> usize {
    let mut left_columns: usize = 0;

    let mut first_row = pattern[0].clone();
    let mut row_symmetry = 0;

    // 0: don't remove, 1: remove left, 2: remove right
    let mut remove_left_or_right = 0;

    // check first row
    if first_row.len() % 2 == 0 {
        // if even, divide in half and check each half for symmetry
        let (left, right) = first_row.split_at(first_row.len() / 2);
        if left == right { row_symmetry = left.len(); }
    } else {
        // if not even, remove one off of each end (individually) and check again
        let first_row_without_left = &first_row[1..];
        let (left, right) = first_row_without_left.split_at(first_row_without_left.len() / 2);
        if left == right { 
            row_symmetry = left.len();
            remove_left_or_right = 1;
        }

        if row_symmetry == 0 { // if not found, check the right side -- no need to double computation if found
            let first_row_without_right = &first_row[..first_row.len() - 1];
            let (left, right) = first_row_without_right.split_at(first_row_without_right.len() / 2);
            if left == right { 
                row_symmetry = left.len(); 
                remove_left_or_right = 2;
            }
        }
    }

    left_columns = row_symmetry;

    // check the same row going down
    if left_columns > 0 {
        for row in pattern.iter().skip(1) {
            if remove_left_or_right == 1 {
                let without_left = &row[1..];
                let (left, right) = without_left.split_at(row_symmetry);
                if left != right { left_columns = 0; }
            } else if remove_left_or_right == 2 {
                let without_right = &row[..row.len() - 1];
                let (left, right) = without_right.split_at(row_symmetry);
                if left != right { left_columns = 0; }
            } else {
                let (left, right) = row.split_at(row_symmetry);
                if left != right { left_columns = 0; }
            }
        }
    }
    
    left_columns
}

// check first column
// if symmetry is found, check the same column going right
// if found all the way, return the number of rows above each horizontal line of reflection * 100
fn check_horizontal_symmetry() -> usize {
    let mut above_rows: usize = 0;
    
    above_rows
}