// https://adventofcode.com/2023/day/9

fn main() {
    
    // EXAMPLE
    let contents = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let sequence = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sequences.push(sequence);
    }

    println!("{:?}", sequences);

    for sequence in sequences {
        let next_number = find_next_number(sequence);
        println!("{:?}", next_number);
    }
}

fn find_next_number(sequence: Vec<i32>) -> i32 {
    let mut next_number = 0;

    let mut differences: Vec<Vec<i32>> = Vec::new();
    let mut sequence = sequence.clone();

    'top: loop {
        let mut layer: Vec<i32> = Vec::new();
        let mut found: bool = true;
        for i in 0..sequence.len() - 1 {
            let difference = sequence[i + 1] - sequence[i];

            found = found && difference == 0;

            layer.push(difference);
        }

        if !found {
            differences.push(layer.clone());
            sequence = layer.clone();
            println!("{:?}", differences);
        } else {
            break 'top;
        }
    }

    loop {
        if let layer: Vec<i32> = differences.pop() {
        
        }
    }

    println!("{:?}", differences);

    next_number
}