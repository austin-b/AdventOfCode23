// https://adventofcode.com/2023/day/3

use std::fs;
use std::env;

const SIZE: i32 = 140;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // enable backtraces

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    // let contents = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    // we know the size of the array
    let mut c_array = [[' '; SIZE as usize]; SIZE as usize];

    for line in contents.lines().enumerate() {
        for c in line.1.chars().enumerate() {   
            c_array[line.0][c.0] = c.1;
        }    
    }

    // println!("{:?}", c_array);

    let mut numbers: Vec<u32> = Vec::new();

    /* PART 1
    let mut symbol_found: bool = false;
    let mut num: Vec<char> = Vec::new();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if c_array[i][j].is_numeric() {
                num.push(c_array[i][j]);

                if !symbol_found {
                    symbol_found = check_neighbors(&c_array, i, j);
                }
            } else {
                if num.len() > 0 && symbol_found {
                    let num_str: String = num.iter().collect();
                    let num_int: u32 = num_str.parse().unwrap();
                    println!("Number: {}", num_int);
                    numbers.push(num_int);
                } 

                num.clear();
                symbol_found = false;
            }
        }
    }
    */

    /* PART 2 */
    for i in 0..SIZE {
        for j in 0..SIZE {
            if c_array[i as usize][j as usize] == '*' {
                if let Some(n) = find_gear_ratio(&c_array, i, j) {
                    numbers.push(n);
                }
            }
        }
    }

    let mut sum: u32 = numbers.into_iter().reduce(|acc, e| acc + e).unwrap(); // sum all numbers

    println!("Sum: {}", sum);
}

fn check_neighbors(c_array: &[[char; SIZE as usize]; SIZE as usize], i: i32, j: i32) -> bool {
    let xd = [-1,  0, 1, 0, -1,  1, 1, -1];     // template neighbor arrays
    let yd = [ 0, -1, 0, 1,  1, -1, 1, -1];

    for k in 0..8 {
        let x = i as i32 + xd[k];
        let y = j as i32 + yd[k];

        if x >= 0 && x < SIZE && y >= 0 && y < SIZE {
            let c = c_array[x as usize][y as usize];
            if !c.is_numeric() && c != '.' {
                println!("Symbol: {}", c);
                return true;
            }
        }
    }

    false
}

fn find_gear_ratio(c_array: &[[char; SIZE as usize]; SIZE as usize], i: i32, j: i32) -> Option<u32> {
    let xd = [-1,  0, 1, 0, -1,  1, 1, -1];     // template neighbor arrays
    let yd = [ 0, -1, 0, 1,  1, -1, 1, -1];

    let mut nums: Vec<u32> = Vec::new();

    // check all neighbors
    for k in 0..8 {
        let x = i + xd[k];
        let y = j + yd[k];

        if x >= 0 && x < SIZE && y >= 0 && y < SIZE {
            let c = c_array[x as usize][y as usize];
            if c.is_numeric() {
                // println!("Number: {}, x: {}, y: {}", c, x, y);
                nums.push(find_whole_number(c_array[x as usize], y)); // find the whole number
                // check if the number is already in the list and exactly two numbers
            }
        }
    }

    // remove duplicates
    nums.sort();
    nums.dedup();

    if nums.len() == 2 {
        println!("Found {:?} for {}, {}", nums, i, j);
        Some(nums.iter().product()) // return the product of the two numbers
    } else {
        None
    }
}

fn find_whole_number(row: [char; SIZE as usize], i: i32) -> u32 {
    let mut num: Vec<char> = Vec::new();

    let mut x = i;

    // check left
    while x >= 0 && row[x as usize].is_numeric() {
        num.push(row[x as usize]);
        x -= 1;
    }

    num.reverse();

    // check right
    x = i + 1;
    while x < SIZE && row[x as usize].is_numeric() {
        num.push(row[x as usize]);
        x += 1;
    }

    println!("Number found: {:?}", num);

    let num_str: String = num.iter().collect();
    num_str.parse().unwrap()
}