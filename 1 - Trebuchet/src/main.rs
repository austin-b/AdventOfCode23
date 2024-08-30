use std::fs; // Filesystem
use std::str; // String
use std::collections::VecDeque; // Vector

fn main() {

    let contents: String = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

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
    for c in line.chars() {
        if c.is_numeric() {
            numbers.push_back(c);
        }
    }

    let number: u32;

    // Use first and last numbers if there are more than one,
    // otherwise use the only number to make a 2-digit number
    if numbers.len() > 1 {
        let mut twodigit = String::new();
        twodigit.push(numbers.pop_front().unwrap()); // First number
        twodigit.push(numbers.pop_back().unwrap());  // Last number
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