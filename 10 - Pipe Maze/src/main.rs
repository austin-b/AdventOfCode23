// https://adventofcode.com/2023/day/10
//
// For inside node calculation:
// https://web.archive.org/web/20130126163405/http://geomalgorithms.com/a03-_inclusion.html

use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn main() {

    // EXAMPLE 1: simple loop
    // let contents = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

    // EXAMPLE 2: simple loop with unconnected pipes
    // let contents = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF";

    // EXAMPLE 3: slightly more complex loop
    // let contents = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";

    // EXAMPLE 4: simpler loop for finding inside nodes
    // let contents = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

    // EXAMPLE 5: more complex loop for finding inside nodes
    // let contents = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";

    // PUZZLE INPUT
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut map: Vec<Vec<Tiles>> = Vec::new();
    let mut starting: (i32, i32) = (0, 0);

    // turn map into tiles
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
    println!("starting: {starting:?}");

    let path = find_path(&map, starting);

    for (pos, steps) in &path {
        println!("{pos:?} {steps:?}");
    }

    println!("path length: {}", path.len());
    println!("steps: {}", path.len()-1);    // -1 because we start at 0
    println!("steps / 2: {}", path.len()-1 / 2);

    // redraw map for easier processing
    let mut new_map: Vec<Vec<(Tiles, Direction)>> = Vec::new();
    for row in map.iter() {
        let mut new_row: Vec<(Tiles, Direction)> = Vec::new();
        for _ in row.iter() {
            new_row.push((Tiles::G, Direction::N));
        }
        new_map.push(new_row);
    }
    for (pos, direction) in &path {
        new_map[pos.0 as usize][pos.1 as usize] = (map[pos.0 as usize][pos.1 as usize], *direction);
    }

    for row in &map {
        println!("{:?}", row);
    }

    let count = find_number_inside_nodes(&new_map);

    println!("count of inside nodes: {}", count);

    generate_new_map(map.len(), map[0].len(), path);
}

fn find_path(map: &Vec<Vec<Tiles>>, starting: (i32, i32)) -> Vec<((i32, i32), Direction)> {
    let mut path: Vec<((i32, i32), Direction)> = Vec::new();
    let mut current_position: (i32, i32) = starting;
    let mut from_direction: Option<Direction> = None;

        // Go through the available directions and find a valid one from starting position
    // There should only be 2 from the starting position, but we will check all 4 as we don't know initially
    // The path goes in a loop, so it does not matter which direction we start with
    let tests = [((1,0), Direction::N), ((0,-1), Direction::E), ((-1, 0), Direction::S), ((0,1), Direction::W)];
    for (step, direction) in tests {
        let next = next_step(map[(current_position.0+step.0) as usize][(current_position.1+step.1) as usize], direction);
        // println!("next: {next:?}");
        if next.is_some() {
            // push the starting position
            path.push((current_position, direction));

            // move to the next position and push
            current_position = (current_position.0+step.0, current_position.1+step.1);
            from_direction = Some(next.unwrap().1);
            path.push((current_position, from_direction.unwrap()));
            break;
        }
    }

    if from_direction.is_none() {
        panic!("No valid direction found from starting position");
    } 

    let mut from_direction = from_direction.unwrap();

    // println!("current_pos: {current_position:?}, from_dir: {from_direction:?}, steps: {:?}", path.len()-1);

    loop {
        let next = next_step(map[current_position.0 as usize][current_position.1 as usize], from_direction);
        // println!("next: {next:?}");
        if next.is_some() {
            current_position = (current_position.0+next.as_ref().unwrap().0.0, current_position.1+next.as_ref().unwrap().0.1);
            from_direction = next.unwrap().1;
            path.push((current_position, from_direction));
        } else {
            break;
        }
    }

    path
}

// output: what to add to the current position, and the direction you're coming from into new position
// TODO: calculate winding number here?
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

fn find_number_inside_nodes(map: &Vec<Vec<(Tiles, Direction)>>) -> usize {
    let mut count = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, (tile, _)) in row.iter().enumerate() {
            if *tile == Tiles::G && is_left(row, j) == false && is_right(row, j) == false {
                println!("checking node: {i:?} {j:?}");
                if is_inside_node(row, j) {
                    count += 1;
                    println!("inside node: {i:?} {j:?}");
                }
            }
        }
    }

    count
}

// https://web.archive.org/web/20130126163405/http://geomalgorithms.com/a03-_inclusion.html
fn is_inside_node(row: &Vec<(Tiles,Direction)>, column: usize) -> bool {
    let mut is_inside = false;

    let mut wn = 0;

    for i in column+1..row.len() {
        let dir = row[i].1;
        match dir {
            Direction::N => wn += 1,
            Direction::S => wn -= 1,
            Direction::E => wn += 1,
            Direction::W => wn -= 1,
        }
    }  

    if wn != 0 {
        is_inside = true;
    }
    
    println!("winding number: {wn:?}");

    is_inside
}

// check if the row is all G to the left of the column, if so, cannot be an inside node
fn is_left(row: &Vec<(Tiles,Direction)>, column: usize) -> bool {
    let mut is_left = true;

    for i in (0..column).rev() {
        if row[i].0 != Tiles::G {
            is_left = false;
            break;
        }
    }

    is_left
}

// check if the row is all G to the right of the column, if so, cannot be an inside node
fn is_right(row: &Vec<(Tiles,Direction)>, column: usize) -> bool {
    let mut is_right = true;

    for i in column+1..row.len() {
        if row[i].0 != Tiles::G {
            is_right = false;
            break;
        }
    }

    is_right
}

fn generate_new_map(height: usize, width: usize, path: Vec<((i32, i32), Direction)>) {
    let mut map: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for (steps, (pos, _)) in path.iter().enumerate() {
        map[pos.0 as usize][pos.1 as usize] = steps;
    }

    let mut map_string: String = String::new();

    for row in &map {
        for cell in row {
            if *cell == 0 {
                map_string.push_str("..... ");
            } else {
                map_string.push_str(format!("{:05} ", cell).as_str());
            }
        }
        map_string.push_str("\n");
    }

    fs::write("src/output.txt", map_string)
        .expect("Unable to write file");
}