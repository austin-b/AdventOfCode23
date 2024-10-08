// https://adventofcode.com/2023/day/9

use std::{collections::VecDeque, fs};

use rayon::prelude::*;

fn main() {
    
    // EXAMPLE
    // let contents = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    // PUZZLE
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut sequences: Vec<VecDeque<i32>> = Vec::new();

    for line in contents.lines() {
        let sequence = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<VecDeque<i32>>();
        sequences.push(sequence);
    }

    println!("{:?}", sequences);

    let sum = sequences.par_iter().map(|sequence| find_first_number(sequence.clone())).sum::<i32>();

    println!("{:?}", sum);
}

fn find_first_number(sequence: VecDeque<i32>) -> i32 {
    let mut differences: Vec<VecDeque<i32>> = Vec::new();
    let mut sequence = sequence.clone();

    differences.push(sequence.clone());

    'top: loop {
        let mut layer: VecDeque<i32> = VecDeque::new();
        let mut found: bool = true;
        for i in 0..sequence.len() - 1 {
            let difference = sequence[i + 1] - sequence[i];

            found = found && difference == 0;

            layer.push_back(difference);
        }

        differences.push(layer.clone());
        sequence = layer.clone();
        // println!("{:?}", differences);
        
        if found {
            break 'top;
        }
    }

    println!("{:?}", differences);

    let mut delta: i32 = 0;
    for i in (0..differences.len()).rev() {
        let layer = &mut differences[i];
        delta = layer[0] - delta;
        layer.push_front(delta);
    }

    differences[0][0]
}