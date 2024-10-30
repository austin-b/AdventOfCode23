// https://adventofcode.com/2023/day/12

use std::{collections::VecDeque, fs};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn main() {
    
    // EXAMPLE
    let contents = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    // PUZZLE
    // let contents: String = fs::read_to_string("src/input.txt")
    //     .expect("Something went wrong reading the file");

    let conditions: Vec<(Vec<SpringState>, Vec<usize>)> = contents.lines().map(|line: &str| {
        let records: Vec<&str> = line.split(" ").collect();
        let states: Vec<SpringState> = records[0].chars().map(|spring| {
            match spring {
                '?' => SpringState::Unknown,
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                _ => panic!("Invalid character"),
            }
        }).collect();
        let states = unfold_states(states);
        let groups: Vec<usize> = records[1].split(",").map(|group| group.parse::<usize>().unwrap()).collect::<Vec<usize>>().repeat(5);
        (states, groups)
    }).collect();
    
    let sum: usize = conditions.par_iter().map(|r| find_number_of_arrangements(r)).sum();

    println!("\nsum of arrangements (total): {:?}", sum);
}

fn unfold_states(mut states: Vec<SpringState>) -> Vec<SpringState> {
    states.push(SpringState::Unknown);
    let mut states = states.repeat(5);
    states.pop();
    states
}

fn find_number_of_arrangements(record: &(Vec<SpringState>, Vec<usize>)) -> usize {
    println!("record: {:?}", record.0);

    let states: &Vec<SpringState> = &record.0;
    let groups: &Vec<usize> = &record.1;

    let unknowns: VecDeque<usize> = states.iter().enumerate().filter(|(_, &s)| s == SpringState::Unknown).map(|(i, _)| i).collect();

    let arrangements = check_permutations(states.clone(), groups.clone(), unknowns);

    println!("arrangements: {:?}", arrangements);
    arrangements
}

fn check_permutations(states: Vec<SpringState>, groups: Vec<usize>, mut unknowns: VecDeque<usize>) -> usize {
    let mut arrangements: usize = 0;

    let next_unknown = unknowns.pop_front();

    if next_unknown.is_some() {
        // change next unknown to damaged
        let mut states_copy_damaged = states.clone();
        states_copy_damaged[next_unknown.unwrap()] = SpringState::Damaged;

        // change next unknown to operational
        let mut states_copy_operational = states.clone();
        states_copy_operational[next_unknown.unwrap()] = SpringState::Operational;

        // recursively check both permutations
        arrangements = check_permutations(states_copy_damaged, groups.clone(), unknowns.clone()) + check_permutations(states_copy_operational, groups.clone(), unknowns.clone());
    } else {
        // if no more unknowns, check if the arrangement is complete
        arrangements += if check_if_complete(&states, &groups) {1} else {0};
    }
    
    arrangements
}

fn check_if_complete(states: &Vec<SpringState>, groups: &Vec<usize>) -> bool {
    // separates out the groups of damaged springs and checks if the counts match the predefined groups
    states.chunk_by(|a, b| a == b).filter(|x| x[0] == SpringState::Damaged)
        .map(|x| x.len()).eq(groups.iter().cloned())
}