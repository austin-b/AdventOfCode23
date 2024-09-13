// https://adventofcode.com/2023/day/9

use std::fs;
use rayon::prelude::*;

fn main() {
    
    // EXAMPLE
    // let contents = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    // PUZZLE
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let sequence = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sequences.push(sequence);
    }

    println!("{:?}", sequences);

    let sum = sequences.par_iter().map(|sequence| find_next_number(sequence.clone())).sum::<i32>();

    println!("{:?}", sum);
}

fn find_next_number(sequence: Vec<i32>) -> i32 {
    let mut differences: Vec<Vec<i32>> = Vec::new();
    let mut sequence = sequence.clone();

    differences.push(sequence.clone());

    'top: loop {
        let mut layer: Vec<i32> = Vec::new();
        let mut found: bool = true;
        for i in 0..sequence.len() - 1 {
            let difference = sequence[i + 1] - sequence[i];

            found = found && difference == 0;

            layer.push(difference);
        }

        differences.push(layer.clone());
        sequence = layer.clone();
        // println!("{:?}", differences);
        
        if found {
            break 'top;
        }
    }

    let mut delta: i32 = 0;
    for i in (0..differences.len()).rev() {
        let layer = &mut differences[i];
        delta = layer[layer.len()-1] + delta;
        layer.push(delta);
    }

    // println!("{:?}", differences);

    differences[0][differences[0].len()-1]
}