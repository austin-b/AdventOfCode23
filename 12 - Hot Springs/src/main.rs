// https://adventofcode.com/2023/day/12

use std::{collections::HashMap, fs};
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
        let groups: Vec<usize> = records[1].split(",").map(|group| group.parse::<usize>().unwrap()).collect();
        (states, groups)
    }).collect();

    // for record in &mut conditions {
    //     println!("complete: {}", check_if_complete(&record.0, &mut record.1));
    //     println!("{:?}", record);
    // }

    // let mut sum: usize = 0;
    // for record in &conditions {
    //     let r = find_number_of_arrangements(record);
    //     println!("sum of arrangements for line {:?}: {:?}\n", record.0, r);
    //     sum += r;
    // }
    
    let sum: usize = conditions.par_iter().map(|r| find_number_of_arrangements(r)).sum();

    println!("\nsum of arrangements (total): {:?}", sum);
}

fn find_number_of_arrangements(record: &(Vec<SpringState>, Vec<usize>)) -> usize {
    let states: &Vec<SpringState> = &record.0;
    let groups: &Vec<usize> = &record.1;
    let mut arrangements: usize = 0;
    for (loc, state) in states.clone().into_iter().enumerate() {
        if state == SpringState::Unknown {
            let mut states_copy_damaged = states.clone();
            states_copy_damaged[loc] = SpringState::Damaged;
            let mut states_copy_operational = states.clone();
            states_copy_operational[loc] = SpringState::Operational;
            arrangements = find_number_of_arrangements(&(states_copy_damaged, groups.clone())) + find_number_of_arrangements(&(states_copy_operational, groups.clone()));
        }
    }
    arrangements += if check_if_complete(&states, &groups) {1} else {0};
    arrangements
}

fn check_if_complete(states: &Vec<SpringState>, groups: &Vec<usize>) -> bool {

    let mut countable: bool = true;
    let mut count: usize = 0;

    let mut groups = groups.clone();

    // quick check to make sure we only use the max amount of damaged springs
    let damaged_springs = states.iter().filter(|&s| s == &SpringState::Damaged).count();
    if damaged_springs > groups.iter().sum::<usize>() { return false; }

    for state in states {
        match state {
            &SpringState::Damaged => if countable {count += 1},
            &SpringState::Unknown => countable = false,
            &SpringState::Operational => {
                countable = true;
    
                if count > 0 {
                    if count == groups[0] { 
                        groups.remove(0);
                        //println!("found a grouping of {}", count);
                    }
                }
    
                count = 0;
            }
        }
    }

    // check for the last group, if available
    if countable && count > 0 {
        if count == groups[0] { 
            groups.remove(0);
            //println!("found a grouping of {}", count);
        }
    }

    groups.is_empty()
}