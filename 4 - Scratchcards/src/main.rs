// https://adventofcode.com/2023/day/4

use core::num;
#[allow(unused_imports, dead_code)]

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
    let mut total_len = 0;

    let mut num_cards = [0; 6]; // debugging
    loop {
        let card = cards.pop_front();

        // debugging
        // if total_len > 12 {
        //     break;
        // }

        // debugging
        let id = card.as_ref().unwrap().id-1;
        num_cards[id as usize] = num_cards[id as usize] + 1;

        if card.is_some() && cards.len() > 0 {

            total_len += 1;

            println!("{:?}", card); // debugging

            let num_matches = number_of_matches(&card.unwrap());

            println!("Matches: {}", num_matches);

            let mut cards_to_add: Vec<Card> = Vec::new();

            // create new cards
            for i in 0..num_matches {
                println!("i: {}", i);
                let new_card = Card {
                    id: cards[i as usize].id,
                    winning_numbers: cards[i as usize].winning_numbers.clone(),
                    numbers: cards[i as usize].numbers.clone(),
                };

                println!("Pushing: {:?}", new_card);

                cards_to_add.push(new_card);
            }

            // reverse the order of the cards to add so we prepend correctly
            //cards_to_add.reverse();

            // add new cards to the front of the deck
            for card in cards_to_add {
                //cards.push_front(card);
                cards.push_back(card);
            }

            println!("Cards:");
            for card in &cards {
                println!("{:?}", card)
            }
            println!("total length: {}", total_len);

        } else if card.is_some() && cards.len() == 0 {    // we know it will never require processing the last card, so we can end here
            total_len += 1;
            break;
        }
    }

    println!("Total cards: {}", total_len);
    println!("Card counts: {:?}", num_cards);
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