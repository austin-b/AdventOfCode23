// https://adventofcode.com/2023/day/11

use std::fs;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty = 0,
    Galaxy = 1,
}

// this is separate from the main function so that the memory can be freed
fn parse_galaxies() -> Vec<(usize, usize)> {

    // EXAMPLE 1
    let contents = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    // PUZZLE
    // let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
        
    let mut map: Vec<Vec<Cell>> = contents.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Galaxy,
            _ => panic!("Invalid character"),
        }).collect()
    }).collect();

    // println!("Before cosmic expansion:");
    // for row in map.iter() {
    //     println!("{:?}", row);
    // }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    // find galaxies
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == Cell::Galaxy {
                galaxies.push((i, j));
            }
        }
    }

    let new_galaxies: Vec<(usize, usize)> = apply_cosmic_expansion(map, galaxies);
    
    new_galaxies
}

fn main() {

    // find galaxies
    let galaxies: Vec<(usize, usize)> = parse_galaxies();

    // find pairs
    let pairs: Vec<((usize, usize),(usize, usize))> = find_pairs(&galaxies);

    println!("\n# of pairs: {}", pairs.len());

    // find shortest paths & sum
    let sum: usize = pairs.par_iter().map(|pair| find_shortest_path_length(*pair)).sum();

    println!("\nSum of shortest paths: {}", sum);
}

fn apply_cosmic_expansion(map: Vec<Vec<Cell>>, galaxies: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let galaxies_clone = galaxies.clone();
    let mut galaxy_pairs: Vec<((usize, usize), (usize, usize))> = galaxies.into_iter().zip(galaxies_clone.into_iter()).collect();

    let expansion_factor: usize = 1_000_000;

    // rows
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&cell| cell == Cell::Empty) {
            for pair in &mut galaxy_pairs {
                let ((x1, _), (x2, _)) = pair;
                if *x1 > i { *x2 += expansion_factor-1;}
            }
            // println!("expanding row {}", i);
        }
    }

    println!("rows expanded");

    // columns
    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] == Cell::Empty) {
            
            // println!("expanding column {}", i);
        }
    }

    println!("columns expanded");

    galaxy_pairs.into_iter().unzip::<(usize, usize),(usize,usize),Vec<(usize,usize)>,Vec<(usize,usize)>>().1.clone()
}

fn find_pairs(galaxies: &Vec<(usize, usize)>) -> Vec<((usize, usize),(usize, usize))> {
    let mut pairs = Vec::new();

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }

    pairs
}

fn find_shortest_path_length(pair: ((usize, usize),(usize, usize))) -> usize {
    let ((x1, y1), (x2, y2)) = pair;

    x2.abs_diff(x1) + y2.abs_diff(y1)
}
