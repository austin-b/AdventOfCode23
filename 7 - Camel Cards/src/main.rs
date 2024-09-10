// https://adventofcode.com/2023/day/7

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
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

fn main() {
    let content = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    let mut hands: Vec<Hand> = Vec::new();

    for line in content.lines() {
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

    for hand in hands {
        println!("{:?} {:?} {:?}", hand.cards, hand.bid, hand.hand_type);
    }

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

 HandType::HighCard   
}