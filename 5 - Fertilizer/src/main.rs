// https://adventofcode.com/2023/day/5
// PART 2

#[allow(dead_code)]
use std::fs;
use std::{
    collections::VecDeque,
    ops::RangeInclusive,
    time::Instant,
};
use rayon::prelude::*;

fn main() {
    let now = Instant::now();

    //let contents = fs::read_to_string("src/example.txt")
    //    .expect("Something went wrong reading the file");

    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let mut seed_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut seed_to_soil: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut water_to_light: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut light_to_temperature: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(RangeInclusive<u64>, u64)> = Vec::new();
    let mut humidity_to_location: Vec<(RangeInclusive<u64>, u64)> = Vec::new();

    println!("contents: {:?}", contents);

    let blocks = contents.split("\n\n").collect::<Vec<&str>>();

    // each mapping is a block like this:
    // seed-to-soil map:
    // 52 50 48     dest_range_start src_range_start range_length
    // 50 98 2

    for b in blocks {
        if b.contains("seeds") {
            let mut seed_nums: VecDeque<u64> = b.split(": ").collect::<Vec<&str>>()[1]
                .split_whitespace()
                .into_iter()
                .map(|x| x.trim().parse().unwrap())
                .collect();

            loop {
                let start = seed_nums.pop_front();
                let length = seed_nums.pop_front();
                if start.is_none() || length.is_none() {
                    break;
                }

                seed_ranges.push(RangeInclusive::new(
                    start.unwrap(),
                    start.unwrap() + length.unwrap(),
                ));
            }

            println!("seed_ranges: {:?}", seed_ranges);
        } else if b.contains("seed-to-soil") {
            // this mapping is different because it is not guaranteed to start at 0
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                // so we create a range and the starting soil index
                seed_to_soil.push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("soil-to-fertilizer") {
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                soil_to_fertilizer
                    .push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("fertilizer-to-water") {
            let ranges = b.split("\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                fertilizer_to_water
                    .push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("water-to-light") {
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                water_to_light.push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("light-to-temperature") {
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                light_to_temperature
                    .push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("temperature-to-humidity") {
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                temperature_to_humidity
                    .push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        } else if b.contains("humidity-to-location") {
            let ranges = b.split(":\n").collect::<Vec<&str>>()[1];
            for r in ranges.lines() {
                let parts = r
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<u64>>();
                humidity_to_location
                    .push((RangeInclusive::new(parts[1], parts[1] + parts[2]), parts[0]));
            }
        }
    }

    // println!("seed_to_soil: {:?}", &seed_to_soil);
    // println!("soil_to_fertilizer: {:?}", &soil_to_fertilizer);
    // println!("fertilizer_to_water: {:?}", &fertilizer_to_water);
    // println!("water_to_light: {:?}", &water_to_light);
    // println!("light_to_temperature: {:?}", &light_to_temperature);
    // println!("temperature_to_humidity: {:?}", &temperature_to_humidity);
    // println!("humidity_to_location: {:?}", &humidity_to_location);

    // par_iter turns this into parallel processing
    let lowest: u64 = seed_ranges.par_iter().map(|range| {
        seed_range_to_location(
            range.clone(),
            seed_to_soil.clone(),
            soil_to_fertilizer.clone(),
            fertilizer_to_water.clone(),
            water_to_light.clone(),
            light_to_temperature.clone(),
            temperature_to_humidity.clone(),
            humidity_to_location.clone(),
        )
    }).min().unwrap();

    println!("lowest location: {:?}", lowest);

    println!("Time: {:.2?}", now.elapsed());
}

fn seed_range_to_location(
    range: RangeInclusive<u64>,
    seed_to_soil: Vec<(RangeInclusive<u64>, u64)>,
    soil_to_fertilizer: Vec<(RangeInclusive<u64>, u64)>,
    fertilizer_to_water: Vec<(RangeInclusive<u64>, u64)>,
    water_to_light: Vec<(RangeInclusive<u64>, u64)>,
    light_to_temperature: Vec<(RangeInclusive<u64>, u64)>,
    temperature_to_humidity: Vec<(RangeInclusive<u64>, u64)>,
    humidity_to_location: Vec<(RangeInclusive<u64>, u64)>,
) -> u64 {
    let (low, high) = range.clone().into_inner();
    println!("processing range: {} - {}", low, high);

    let mut lowest: u64 = 0;

    for seed in range {
        let mut soil = 0;
        for (range, dest) in &seed_to_soil {
            if range.contains(&seed) {
                soil = dest.clone() + (seed - range.start());
                break;
            }
        }
        // Any source numbers that aren't mapped correspond to the same destination number.
        if soil == 0 {
            soil = seed.clone();
        }

        let mut fertilizer = 0;
        for (range, dest) in &soil_to_fertilizer {
            if range.contains(&soil) {
                fertilizer = dest.clone() + (soil - range.start());
                break;
            }
        }
        if fertilizer == 0 {
            fertilizer = soil.clone();
        }

        let mut water = 0;
        for (range, dest) in &fertilizer_to_water {
            if range.contains(&fertilizer) {
                water = dest.clone() + (fertilizer - range.start());
                break;
            }
        }
        if water == 0 {
            water = fertilizer.clone();
        }

        let mut light = 0;
        for (range, dest) in &water_to_light {
            if range.contains(&water) {
                light = dest.clone() + (water - range.start());
                break;
            }
        }
        if light == 0 {
            light = water.clone();
        }

        let mut temperature = 0;
        for (range, dest) in &light_to_temperature {
            if range.contains(&light) {
                temperature = dest.clone() + (light - range.start());
                break;
            }
        }
        if temperature == 0 {
            temperature = light.clone();
        }

        let mut humidity = 0;
        for (range, dest) in &temperature_to_humidity {
            if range.contains(&temperature) {
                humidity = dest.clone() + (temperature - range.start());
                break;
            }
        }
        if humidity == 0 {
            humidity = temperature.clone();
        }

        let mut location = 0;
        for (range, dest) in &humidity_to_location {
            if range.contains(&humidity) {
                location = dest.clone() + (humidity - range.start());
                break;
            }
        }
        if location == 0 {
            location = humidity.clone();
        }

        if lowest == 0 || location < lowest {
            lowest = location;
        }

        // println!("seed: {}, soil: {}, fertilizer: {}, water: {}, light: {}, temperature: {}, humidity: {}, location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
    
        if lowest == 0 || location < lowest {
            lowest = location;
        }
    }

    println!("lowest for {} - {}: {}", low, high, lowest);

    lowest
}
