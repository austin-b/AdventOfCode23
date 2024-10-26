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
    // let contents = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    // PUZZLE
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

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
    println!("record: {:?}", record.0);
    let states = record.0.clone();
    let mut unknowns: Vec<usize> = states.iter().enumerate().filter(|(_, &s)| s == SpringState::Unknown).map(|(i, _)| i).collect();
    let groups = record.1.clone();
    let remaining_broken = groups.iter().sum::<usize>() - states.iter().filter(|&s| s == &SpringState::Damaged).count();
    // println!("remaining_broken: {:?}", remaining_broken);

    let mut arrangements: usize = 0;

    if check_if_complete(&states, &groups) { return 1; } // if the record is already complete, return 1

    // create a cache to prevent duplicate calls to check_if_complete
    let mut cache = HashMap::new();

    // for the test case, we will assume that all unknowns are operational and then replace them with damaged in the while loop
    let base_test_states: Vec<SpringState>  = states.iter().map(|&s| 
        if s == SpringState::Unknown {SpringState::Operational} else {s}).collect::<Vec<SpringState>>();
    let mut test_states: Vec<SpringState> = base_test_states.clone();

    /* non-recursive Heap's algorithm (https://en.wikipedia.org/wiki/Heap's_algorithm) */
    let mut c: Vec<usize> = vec![0; unknowns.len()]; // encoding of the stack state
    let mut i: usize = 0; // first index
    while i < unknowns.len() {
        if c[i] < i {
            if i % 2 == 0 {
                unknowns.swap(0, i);

                // replace the first remaining_broken unknowns with damaged
                for x in 0..remaining_broken {
                    test_states[unknowns[x]] = SpringState::Damaged;
                }
            } else {
                unknowns.swap(c[i], i);
                
                // replace the first remaining_broken unknowns with damaged
                for x in 0..remaining_broken {
                    test_states[unknowns[x]] = SpringState::Damaged;
                }
            }
            // println!("{:?}", test_states);

            // check cache for the test_states
            // if it has already been tested, we do not need to check again
            arrangements += if cache.contains_key(&test_states) {0} else {
                let v = check_if_complete(&test_states, &groups);
                cache.insert(test_states.clone(), v);
                v as usize
            };

            // reset the test_states
            test_states = base_test_states.clone();

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

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