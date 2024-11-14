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
    // let contents = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
    let contents = "???.### 1,1,3";

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
    println!("record: {:?}, groups: {:?}", record.0, record.1);

    let states: &Vec<SpringState> = &record.0;
    let groups: &Vec<usize> = &record.1;

    let unknowns: VecDeque<usize> = states.iter().enumerate().filter(|(_, &s)| s == SpringState::Unknown).map(|(i, _)| i).collect();

    let arrangements = check_permutations(states.clone(), &groups, unknowns);

    println!("arrangements: {:?}", arrangements);
    arrangements
}

fn check_permutations(states: Vec<SpringState>, groups: &Vec<usize>, mut unknowns: VecDeque<usize>) -> usize {
    let mut arrangements: usize = 0;

    let next_unknown = unknowns.pop_front();

    if next_unknown.is_some() {
        // change next unknown to damaged
        let mut states_copy_damaged = states.clone();
        states_copy_damaged[next_unknown.unwrap()] = SpringState::Damaged;
        let new_with_damaged: usize = if check_if_valid(&states_copy_damaged, &groups) { // if not valid, skip this branch
            check_permutations(states_copy_damaged, &groups, unknowns.clone())
        } else {0};

        // change next unknown to operational
        let mut states_copy_operational = states.clone();
        states_copy_operational[next_unknown.unwrap()] = SpringState::Operational;
        let new_with_operational: usize = if check_if_valid(&states_copy_operational, &groups) { // if not valid, skip this branch
            check_permutations(states_copy_operational, &groups, unknowns.clone())
        } else {0};

        // recursively check both permutations
        arrangements = new_with_damaged + new_with_operational;
    } else {
        // if no more unknowns, double check if the arrangement is complete
        // arrangements += if check_if_valid(&states, &groups) {1} else {0};
        // println!("checking: {:?}", states);
        if check_if_valid(&states, &groups) {
            arrangements += 1;
            // println!("complete: {:?}\ngroups: {:?}", states, groups);
        }
        // println!("complete: {:?}", arrangements);
    }
    
    arrangements
}

fn check_if_complete(states: &Vec<SpringState>, groups: &Vec<usize>) -> bool {
    // separates out the groups of damaged springs and checks if the counts match the predefined groups
    states.chunk_by(|a, b| a == b).filter(|x| x[0] == SpringState::Damaged)
        .map(|x| x.len()).eq(groups.iter().cloned())
}

fn check_if_valid(states: &Vec<SpringState>, groups: &Vec<usize>) -> bool {

    let mut valid: bool = false;
    let mut count: usize = 0;
    let mut group_to_check: usize = 0;

    // quick check to make sure we only use the max amount of damaged springs
    // let damaged_springs = states.iter().filter(|&s| s == &SpringState::Damaged).count();
    // if damaged_springs > groups.iter().sum::<usize>() { return false; }

    // println!("checking states: {:?}", states);
    // println!("checking groups: {:?}", groups);

    for state in states {
        match state {
            &SpringState::Damaged => count += 1,
            &SpringState::Unknown => {
                // if we switch to unknown and we have a count, check if the count
                // is less than or equal to the next group
                if count <= groups[group_to_check] {
                    valid = true;
                    break;
                } else { // if the count is greater than the next group, the arrangement is invalid
                    valid = false;
                    // println!("invalid");
                    break;
                }
            },
            &SpringState::Operational => {
                if count > 0 {
                    // if we switch to operational and we have a count, check if the count matches the next group
                    if count == groups[group_to_check] { 
                        if group_to_check == groups.len() - 1 {
                            valid = true;
                            break;
                        } else {
                            group_to_check += 1;
                            valid = true;
                        }
                    } else { // if the count doesn't match the next group, the arrangement is invalid
                        valid = false;
                        break;
                    }
                }
    
                count = 0;
            }
        }
    }

    // TODO: FIX, does not accurately check for final group -- too many permutations are considered valid
    // check for the last group, if available
    if count > 0 && group_to_check == groups.len() - 1{
        if count == groups[group_to_check] { 
            valid = true;
        } else {
            valid = false;
        }
    }

    if valid {println!("states: {:?}\ngroups: {:?}\nvalid: {:?}, last count = {:?}, group_to_check = {:?}", states, groups, valid, count, group_to_check);}
    valid
}