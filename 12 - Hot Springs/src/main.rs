// https://adventofcode.com/2023/day/12

use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn main() {
    
    // EXAMPLE
    let contents = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    let mut conditions: Vec<(Vec<SpringState>, Vec<usize>)> = contents.lines().map(|line: &str| {
        let records: Vec<&str> = line.split(" ").collect();
        let states: Vec<SpringState> = records[0].chars().map(|spring| {
            match spring {
                '?' => SpringState::Unknown,
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                _ => panic!("Invalid character"),
            }
        }).collect();
        let mut groups: Vec<usize> = records[1].split(",").map(|group| group.parse::<usize>().unwrap()).collect();
        groups.sort();
        (states, groups)
    }).collect();

    // for record in &mut conditions {
    //     println!("complete: {}", check_if_complete(&record.0, &mut record.1));
    //     println!("{:?}", record);
    // }

    println!("\nsum of arrangements: {:?}", find_number_of_arrangements(&conditions[1]));

    // let sum: usize = conditions.par_iter().map(|r| find_number_of_arrangements(r)).sum();

    // println!("\nsum of arrangements: {:?}", sum);
}

// TODO: how to handle duplicate arrangements?
fn find_number_of_arrangements(record: &(Vec<SpringState>, Vec<usize>)) -> usize {
    let states = &record.0;
    let mut unknowns: Vec<usize> = states.iter().enumerate().filter(|(_, &s)| s == SpringState::Unknown).map(|(i, _)| i).collect();

    let groups = &record.1;
    let remaining_broken = groups.iter().sum::<usize>() - states.iter().filter(|&s| s == &SpringState::Damaged).count();

    let mut arrangements: usize = 0;

    if check_if_complete(states, groups) { return 1; } // if the record is already complete, return 1

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
            println!("{:?}", test_states);
            if check_if_complete(&test_states, groups) { 
                println!("found complete"); 
                arrangements += 1; 
            } else {
                test_states = base_test_states.clone();
            }
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

    for state in states {
        match state {
            &SpringState::Damaged => if countable {count += 1},
            &SpringState::Unknown => countable = false,
            &SpringState::Operational => {
                countable = true;
    
                if count > 0 {
                    // println!("found a grouping of {}", count);
                    if let Ok(index) = groups.binary_search(&count) { groups.remove(index); }
                }
    
                count = 0;
            }
        }
    }

    // check for the last group, if available
    if countable && count > 0 {
        println!("found a grouping of {}", count);
        if let Ok(index) = groups.binary_search(&count) { groups.remove(index); }
    }

    groups.is_empty()
}