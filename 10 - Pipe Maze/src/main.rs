// https://adventofcode.com/2023/day/10

use std::{error::Error, f32::consts::E, path, thread::current};

#[derive(Clone, Copy, Debug)]
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
    let mut starting: (i32, i32) = (0, 0);

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
                starting = (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
        map.push(row_vec);
    }

    // for row in &map {
    //     println!("{:?}", row);
    // }
    println!("{starting:?}");

    let path = find_path(&map, starting);

    for (pos, steps) in path {
        println!("{pos:?} {steps:?}");
    }
}

fn find_path(map: &Vec<Vec<Tiles>>, starting: (i32, i32)) -> Vec<((i32, i32), usize)> {
    let mut path: Vec<((i32, i32), usize)> = Vec::new();
    let mut current_position: (i32, i32) = starting;
    let mut from_direction: Option<Direction> = None;
    let mut steps: usize = 0;

    // Go through the available directions and find a valid one
    // There should only be 2 from the starting position, but we will check all 4 as we don't know initially
    // The path goes in a loop, so it does not matter which direction we start with
    let tests = [((1,0), Direction::N), ((0,-1), Direction::E), ((-1, 0), Direction::S), ((0,1), Direction::W)];
    for (step, direction) in tests {
        let next = next_step(map[(current_position.0+step.0) as usize][(current_position.1+step.1) as usize], direction);
        println!("{next:?}");
    }

    if from_direction.is_none() {
        panic!("No valid direction found from starting position");
    } else { println!("{current_position:?} {from_direction:?} {steps:?}"); }

    path
}

// output: what to add to the current position, and the direction you're coming from into new position
fn next_step(current_tile: Tiles, from_direction: Direction) -> Option<((i32, i32), Direction)> {
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
                Direction::E => Some(((0,-1), Direction::E)),
                Direction::W => Some(((0,1), Direction::W)),
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