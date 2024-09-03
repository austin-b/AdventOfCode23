// https://adventofcode.com/2023/day/4

use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

fn main() {
    let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // let contents: String = fs::read_to_string("src/input.txt")
    //     .expect("Something went wrong reading the file");

    let mut cards: VecDeque<Card> = VecDeque::new();

    // process input
    for line in contents.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let numbers = parts[1].split(" | ").collect::<Vec<&str>>();

        let card = Card {
            id: parts[0].split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap(),
            winning_numbers: numbers[0].split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.trim().parse().unwrap()).collect::<Vec<i32>>(),
            numbers: numbers[1].split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.trim().parse().unwrap()).collect::<Vec<i32>>(),
        };

        // println!("{:?}", card);
        cards.push_back(card);
    }

    /* PART 1
    let mut total_points = 0;

    for card in &cards {
        total_points += get_points(card);
    }

    println!("Total points: {}", total_points);
    */

    /* PART 2  */
    loop {

        let card = cards.pop_front();

        if card.is_some() && cards.len() > 0 {
            println!("{:?}", card);
        } else {
            break;
        }

    }

    for card in &cards {
        println!("{:?}", card);
    }

    println!("Total cards: {}", cards.len());
}

fn get_points(card: &Card) -> i32 {
    let mut points = 0;
    
    for number in &card.numbers {
        if card.winning_numbers.contains(number) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

fn number_of_matches(card: &Card) -> i32 {
    let mut matches = 0;

    for number in &card.numbers {
        if card.winning_numbers.contains(number) {
            matches += 1;
        }
    }

    matches
}