// https://adventofcode.com/2023/day/11

use std::fs;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Galaxy,
}

fn main() {
    
    // EXAMPLE 1
    // let contents = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    // PUZZLE
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut map: Vec<Vec<Cell>> = contents.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Galaxy,
            _ => panic!("Invalid character"),
        }).collect()
    }).collect();

    println!("Before cosmic expansion:");
    for row in map.iter() {
        println!("{:?}", row);
    }

    cosmic_expansion(&mut map);        

    println!("\nAfter cosmic expansion:");
    for row in map.iter() {
        println!("{:?}", row);
    }

    // find galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == Cell::Galaxy {
                galaxies.push((i, j));
            }
        }
    }

    // find pairs
    let pairs = find_pairs(&galaxies);

    println!("\n# of pairs: {}, Pairs: {:?}", pairs.len(), pairs);

    // find shortest paths & sum
    let sum: usize = pairs.par_iter().map(|pair| find_shortest_path_length(*pair)).sum();

    println!("\nSum of shortest paths: {}", sum);
}

fn cosmic_expansion(map: &mut Vec<Vec<Cell>>) {
    let mut new_map = map.clone(); // TODO: can I do this without cloning?

    // needed to update new map to insert correct index
    let mut offset: usize = 0;

    for (i, row) in new_map.clone().iter().enumerate() {
        if row.iter().all(|&cell| cell == Cell::Empty) {
            new_map.insert(i+offset, vec![Cell::Empty; row.len()]);
            offset += 1;
            println!("expanding row {}", i);
        }
    }

    // reset for columns
    offset = 0;

    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] == Cell::Empty) {
            // println!("found empty column at index {}", i);
            for row in new_map.iter_mut() {
                row.insert(i+offset, Cell::Empty);
            }
            offset += 1;
            println!("expanding column {}", i);
        }
    }

    *map = new_map;
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

    let dist = x2.abs_diff(x1) + y2.abs_diff(y1);

    println!("Distance between {:?} and {:?} is {}", (x1,y1), (x2,y2), dist);

    dist
}
