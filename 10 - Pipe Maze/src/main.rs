// https://adventofcode.com/2023/day/10

use std::{error::Error, f32::consts::E, thread::current};

#[derive(Debug)]
enum Tiles {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

#[derive(Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn main() {

    // EXAMPLE 1
    let contents = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

    let mut map: Vec<Vec<Tiles>> = Vec::new();
    let mut starting: (usize, usize) = (0, 0);

    for (i, row) in contents.lines().enumerate() {
        let mut row_vec: Vec<Tiles> = Vec::new();
        for (j, c) in row.chars().enumerate() {

            if c == '|' {
                row_vec.push(Tiles::NS);
            } else if c == '-' {
                row_vec.push(Tiles::EW);
            } else if c == '.' {
                row_vec.push(Tiles::G);
            } else if c == 'L' {
                row_vec.push(Tiles::NE);
            } else if c == 'J' {
                row_vec.push(Tiles::NW);
            } else if c == '7' {
                row_vec.push(Tiles::SW);
            } else if c == 'F' {
                row_vec.push(Tiles::SE);
            } else if c == 'S' {
                row_vec.push(Tiles::S);
            }

            if c == 'S' {
                starting = (i, j);
            }
        }
        map.push(row_vec);
    }

    // for row in &map {
    //     println!("{:?}", row);
    // }
    println!("{starting:?}");
}

fn find_path(map: &Vec<Vec<Tiles>>, starting: (usize, usize)) -> Vec<((usize,usize), usize))> {
    let mut current_tile = map[starting.0][starting.1];
    let mut from_direction: Direction;

    
}

fn next_step(current_tile: Tiles, from_direction: Direction) -> Option<((i8, i8), Direction)> {
    match current_tile {
        Tiles::NS => {
            match from_direction {
                Direction::N => Some(((1,0), Direction::N)),
                Direction::S => Some(((-1,0), Direction::S)),
                _ => None,
            }
        }
        Tiles::EW => {
            match from_direction {
                Direction::E => Some(((0,1), Direction::E)),
                Direction::W => Some(((0,-1), Direction::W)),
                _ => None,
            }
        }
        Tiles::NE => {
            match from_direction {
                Direction::N => Some(((0,1), Direction::W)),
                Direction::E => Some(((-1,0), Direction::S)),
                _ => None,
            }
        }
        Tiles::NW => {
            match from_direction {
                Direction::N => Some(((0,-1), Direction::E)),
                Direction::W => Some(((-1,0), Direction::S)),
                _ => None,
            }
        }
        Tiles::SW => {
            match from_direction {
                Direction::S => Some(((0,-1), Direction::E)),
                Direction::W => Some(((1,0), Direction::N)),
                _ => None,
            }
        }
        Tiles::SE => {
            match from_direction {
                Direction::S => Some(((0,1), Direction::W)),
                Direction::E => Some(((1,0), Direction::N)),
                _ => None,
            }
        }
        _ => None,
    }
}