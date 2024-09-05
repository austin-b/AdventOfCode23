// https://adventofcode.com/2023/day/5

#[allow(dead_code)]

use std::fs;
use std::ops::RangeInclusive;

fn main() {

    let contents = fs::read_to_string("src/example.txt")
        .expect("Something went wrong reading the file");

    let mut seeds: Vec<usize> = Vec::new();
    let mut seed_to_soil: Vec<(RangeInclusive<usize>, usize)> = Vec::new();
    let mut soil_to_fertilizer: Vec<usize> = Vec::new();
    let mut fertilizer_to_water: Vec<usize> = Vec::new();
    let mut water_to_light: Vec<usize> = Vec::new();
    let mut light_to_temperature: Vec<usize> = Vec::new();
    let mut temperature_to_humidity: Vec<usize> = Vec::new();
    let mut humidity_to_location: Vec<usize> = Vec::new();

    let blocks = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    // each mapping is a block like this:
    // seed-to-soil map:
    // 52 50 48     dest_range_start src_range_start range_length
    // 50 98 2

    for b in blocks {
        if b.contains("seeds") {
            seeds = b.split(": ").collect::<Vec<&str>>()[1].split_whitespace().into_iter().map(|x| x.trim().parse().unwrap()).collect::<Vec<usize>>();
            println!("{:?}", seeds);
        } else if b.contains("seed-to-soil") {
            // this mapping is different because it is not guaranteed to start at 0
            let ranges = b.split(":\r\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r.split_whitespace().into_iter().map(|x| x.trim().parse().unwrap()).collect::<Vec<usize>>();
                // so we create a range and the starting soil index
                seed_to_soil.push((RangeInclusive::new(parts[1], parts[2]), parts[0]));
            }
        }
        // continue mapping the rest of the blocks
        // remember: each mapping is out of order, but as a whole they start from 0 and go up continuously
    }

    if RangeInclusive::new(1, 5).contains(&4) {
        println!("yes");
    }
}
