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

    for record in &mut conditions {
        check_and_remove_completed_groups(&record.0, &mut record.1);
        println!("{:?}", record);
    }

    // let sum: usize = conditions.par_iter().map(|r| find_number_of_arrangements(r)).sum();

    // println!("\nsum of arrangements: {:?}", sum);
}

// TODO: implement some version of Heap's algorithm?
fn find_number_of_arrangements(record: &(Vec<SpringState>, Vec<usize>)) -> usize {
    let mut states = &record.0;
    let mut groups = &record.1;



    1
}

fn check_and_remove_completed_groups(states: &Vec<SpringState>, groups: &mut Vec<usize>) {

    let mut countable: bool = true;
    let mut count: usize = 0;

    for state in states {
        match state {
            &SpringState::Damaged => if countable {count += 1},
            &SpringState::Unknown => countable = false,
            &SpringState::Operational => {
                countable = true;
    
                if count > 0 {
                    println!("found a grouping of {}", count);
                    if let Ok(index) = groups.binary_search(&count) { groups.remove(index); }
                }
    
                count = 0;
            }
        }
    }

    if countable && count > 0 {
        println!("found a grouping of {}", count);
        if let Ok(index) = groups.binary_search(&count) { groups.remove(index); }
    }
}