// https://adventofcode.com/2023/day/2

use std::fs;
use std::str;

#[derive(Debug)]
struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

fn process_line(line: &str) -> Game {
    let game_split: Vec<&str> = line.split(": ").collect();
    let id: u32 = game_split[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

    let sets: Vec<&str> = game_split[1].split("; ").collect();

    let mut max_blue: u32 = 0;
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;

    for set in sets {
        let colors: Vec<&str> = set.split(", ").collect();

        for color in colors {
            let color_split: Vec<&str> = color.split(" ").collect();
            let count: u32 = color_split[0].parse().unwrap();
            let color: &str = color_split[1];

            if color == "blue" {
                if count > max_blue {
                    max_blue = count;
                }
            } else if color == "red" {
                if count > max_red {
                    max_red = count;
                }
            } else if color == "green" {
                if count > max_green {
                    max_green = count;
                }
            }
        }
    }

    Game {
        id: id,
        max_blue: max_blue,
        max_red: max_red,
        max_green: max_green,
    }
}

fn main() {
    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    // let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let lines: str::Lines<'_> = contents.lines();

    let blue_cubes: u32 = 14;
    let red_cubes: u32 = 12;
    let green_cubes: u32 = 13;

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let game: Game = process_line(line);

        // verify that the game is possible with the cubes we have
        if game.max_blue <= blue_cubes && game.max_red <= red_cubes && game.max_green <= green_cubes {
            games.push(game);
        }
    }

    let mut sum: u32 = 0;

    for game in games {
        sum += game.id;
    }

    println!("Sum: {}", sum);
}
