// https://adventofcode.com/2023/day/3

use std::fs;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // enable backtraces

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    // let contents = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    // we know the size of the array
    let mut c_array = [[' '; 140]; 140];

    for line in contents.lines().enumerate() {
        for c in line.1.chars().enumerate() {   
            c_array[line.0][c.0] = c.1;
        }    
    }

    // println!("{:?}", c_array);

    let mut numbers: Vec<u32> = Vec::new();

    let mut symbol_found: bool = false;
    let mut num: Vec<char> = Vec::new();

    for i in 0..140 {
        for j in 0..140 {
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

    let mut sum: u32 = 0;

    for n in numbers {
        sum += n;
    }

    println!("Sum: {}", sum);
}

fn check_neighbors(c_array: &[[char; 140]; 140], i: usize, j: usize) -> bool {
    let xd = [-1,  0, 1, 0, -1,  1, 1, -1];     // template neighbor arrays
    let yd = [ 0, -1, 0, 1,  1, -1, 1, -1];

    for k in 0..8 {
        let x = i as i32 + xd[k];
        let y = j as i32 + yd[k];

        if x >= 0 && x < 140 && y >= 0 && y < 140 {
            let c = c_array[x as usize][y as usize];
            if !c.is_numeric() && c != '.' {
                println!("Symbol: {}", c);
                return true;
            }
        }
    }

    false
}