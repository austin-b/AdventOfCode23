// https://adventofcode.com/2023/day/1

use std::collections::VecDeque;
use std::fs;
use std::str;

fn main() {
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    //let contents = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    let lines: str::Lines<'_> = contents.lines();

    let mut sum: u32 = 0;

    for line in lines {
        sum += process_line_for_number(line);
    }

    println!("Sum: {}", sum);
}

fn process_line_for_number(line: &str) -> u32 {
    let mut numbers: VecDeque<char> = VecDeque::new();

    println!("new line: {}", line);

    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            numbers.push_back(c);
        } else {
            if c == 'o' {
                if let Some("one") = line.get(i..i + 3) {
                    numbers.push_back('1');
                    println!("one found");
                }
            } else if c == 't' {
                if let Some("two") = line.get(i..i + 3) {
                    numbers.push_back('2');
                    println!("two found");
                }
                if let Some("three") = line.get(i..i + 5) {
                    numbers.push_back('3');
                    println!("three found");
                }
            } else if c == 'f' {
                if let Some("four") = line.get(i..i + 4) {
                    numbers.push_back('4');
                    println!("four found");
                }
                if let Some("five") = line.get(i..i + 4) {
                    numbers.push_back('5');
                    println!("five found");
                }
            } else if c == 's' {
                if let Some("six") = line.get(i..i + 3) {
                    numbers.push_back('6');
                    println!("six found");
                }
                if let Some("seven") = line.get(i..i + 5) {
                    numbers.push_back('7');
                    println!("seven found");
                }
            } else if c == 'e' {
                if let Some("eight") = line.get(i..i + 5) {
                    numbers.push_back('8');
                    println!("eight found");
                }
            } else if c == 'n' {
                if let Some("nine") = line.get(i..i + 4) {
                    numbers.push_back('9');
                    println!("nine found");
                }
            }
        }
    }

    let number: u32;

    // Use first and last numbers if there are more than one,
    // otherwise use the only number to make a 2-digit number
    if numbers.len() > 1 {
        let mut twodigit = String::new();
        twodigit.push(numbers.pop_front().unwrap()); // First number
        twodigit.push(numbers.pop_back().unwrap()); // Last number
        number = twodigit.parse().unwrap();
    } else {
        let onedigit = numbers.pop_front().unwrap(); // Only number
        let mut twodigit = String::new();
        twodigit.push(onedigit);
        twodigit.push(onedigit);
        number = twodigit.parse().unwrap();
    }

    println!("number: {}", number);
    number
}
