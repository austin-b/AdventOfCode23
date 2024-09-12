// https://adventofcode.com/2023/day/8
// TODO: per the Internet, bruteforcing this will take days if not weeks or months: https://www.reddit.com/r/adventofcode/comments/18dgbhq/2023_day_8_part_2_this_must_be_the_way/
//       will have to come back to this one later

use std::fs;
use std::collections::HashMap;

fn main() {
    // EXAMPLE 1
    // let contents = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

    // EXAMPLE 2
    // let contents = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    // EXAMPLE 3
    // let contents = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let pieces: Vec<&str> = contents.split("\n\n").collect();
    // println!("{:?}", pieces);
    let directions: &str = pieces[0];
    let nodes_str: Vec<&str> = pieces[1].split("\n").collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_nodes: Vec<&str> = Vec::new();

    for node in nodes_str {
        let node_info: Vec<&str> = node.split(" = ").collect();
        let node_name: &str = node_info[0];
        let node_children: Vec<&str> = node_info[1].split(", ").collect();
        let node_left: &str = node_children[0].trim_matches(|c| c == '(' || c == ')');
        let node_right: &str = node_children[1].trim_matches(|c| c == '(' || c == ')');

        if node_name.ends_with('A') {
            current_nodes.push(node_name);
        }

        nodes.insert(node_name, (node_left, node_right));
    }

    println!("Directions: {:?}", directions);
    println!("Nodes: {:?}", nodes);
    println!("Starting nodes: {:?}", current_nodes);

    let mut step_count: i32 = 0;

    // Directions can be repeated, so we loop until we find the end node
    'top: loop {
        for direction in directions.chars() {
            let mut next_step_nodes: Vec<&str> = Vec::new();
            let mut found: bool = true; // starts with true and will be set to false if any node doesn't end with Z by the &&

            for node in current_nodes.iter() {
                let node = nodes.get(*node).unwrap();
                let new_node: &str;

                // println!("{:?}: {:?} {:?}", node, node.0, node.1);

                if direction == 'R' {
                    new_node = node.1;
                } else {
                    new_node = node.0;
                }

                found = found && new_node.ends_with('Z');

                next_step_nodes.push(new_node);
            }

            current_nodes = next_step_nodes;

            step_count += 1;

            if step_count % 1000000 == 0 {
                println!("{:?}", step_count);
            }

            if found {
                break 'top;
            }
        }
    }

    println!("{:?}", step_count);
    println!("{:?}", current_nodes);
}
