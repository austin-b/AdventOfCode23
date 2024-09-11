// https://adventofcode.com/2023/day/7

use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: [Card; 5],
    bid: i32,
    hand_type: HandType,
}

// Implement comparison for Hand as we need more than default
impl Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        }

        // compare cards if types are the same
        for card in self.cards.iter().zip(other.cards.iter()) {
            if card.0 > card.1 {
                return std::cmp::Ordering::Greater;
            } else if card.0 < card.1 {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }
}

fn main() {
    // let contents = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut hands: Vec<Hand> = Vec::new();

    for line in contents.lines() {
        let hand_info = line.split_whitespace().collect::<Vec<&str>>();
        let cards = parse_card(&hand_info[0].to_string());
        let bid = hand_info[1].parse::<i32>().unwrap();
        let hand_type = find_type(&cards);

        hands.push(Hand {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
        })
    }

    for hand in &hands {
        println!("{:?} {:?} {:?}", hand.cards, hand.bid, hand.hand_type);
    }

    // sort hands by bid (lowest to highest)
    hands.sort_by(|a, b| a.cmp(&b));

    println!("\nSorted hands:");
    for hand in &hands {
        println!("{:?} {:?} {:?}", hand.cards, hand.bid, hand.hand_type);
    }

    let mut total_winnings = 0;
    for i in 0..hands.len() {
        total_winnings += hands[i].bid * (i as i32 + 1);
    }

    println!("\nTotal winnings: {}", total_winnings);
}

fn parse_card(cards: &String) -> [Card; 5] {
    let mut card_array: [Card; 5] = [Card::Two; 5];

    for (i, c) in cards.chars().enumerate() {
       if c == '2' {
           card_array[i] = Card::Two;
       } else if c == '3' {
           card_array[i] = Card::Three;
       } else if c == '4' {
           card_array[i] = Card::Four;
       } else if c == '5' {
           card_array[i] = Card::Five;
       } else if c == '6' {
           card_array[i] = Card::Six;
       } else if c == '7' {
           card_array[i] = Card::Seven;
       } else if c == '8' {
           card_array[i] = Card::Eight;
       } else if c == '9' {
           card_array[i] = Card::Nine;
       } else if c == 'T' {
           card_array[i] = Card::Ten;
       } else if c == 'J' {
           card_array[i] = Card::Jack;
       } else if c == 'Q' {
           card_array[i] = Card::Queen;
       } else if c == 'K' {
           card_array[i] = Card::King;
       } else if c == 'A' {
           card_array[i] = Card::Ace;
       }
    }

    card_array
}

fn find_type(cards: &[Card; 5]) -> HandType {

    let mut card_counts: Vec<(Card, i8)> = Vec::new();

    // Count labels
    for card in cards {
        let mut found = false;
        for (c, count) in card_counts.iter_mut() {
            if card == c {
                *count += 1;
                found = true;
                break;
            }
        }

        if !found {
            card_counts.push((*card, 1));
        }
    }

    // Sort by card type in descending order
    card_counts.sort_by(|a, b| b.0.cmp(&a.0));

    // Check for Jacks
    if card_counts[card_counts.len()-1].0 == Card::Jack {
        card_counts[0].1 += card_counts[card_counts.len()-1].1;
    }

    // Sort by count in descending order
    card_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Check for hand types
    let first_card_count = card_counts[0].1;
    if first_card_count == 5 {
        return HandType::FiveOfAKind;
    } else if first_card_count == 4 {
        return HandType::FourOfAKind;
    } else if first_card_count == 3 {
        let second_card_count = card_counts[1].1;
        if second_card_count == 2 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    } else if first_card_count == 2 {
        let second_card_count = card_counts[1].1;
        if second_card_count == 2 {
            return HandType::TwoPairs;
        } else {
            return HandType::OnePair;
        }
    }

    HandType::HighCard   // Default
}