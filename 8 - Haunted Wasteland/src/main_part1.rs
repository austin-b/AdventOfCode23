// https://adventofcode.com/2023/day/8

use std::fs;
use std::collections::HashMap;

fn main() {
    // EXAMPLE 1
    // let contents = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

    // EXAMPLE 2
    // let contents = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let pieces: Vec<&str> = contents.split("\n\n").collect();
    let directions: &str = pieces[0];
    let nodes_str: Vec<&str> = pieces[1].split("\n").collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for node in nodes_str {
        let node_info: Vec<&str> = node.split(" = ").collect();
        let node_name: &str = node_info[0];
        let node_children: Vec<&str> = node_info[1].split(", ").collect();
        let node_left: &str = node_children[0].trim_matches(|c| c == '(' || c == ')');
        let node_right: &str = node_children[1].trim_matches(|c| c == '(' || c == ')');

        nodes.insert(node_name, (node_left, node_right));
    }

    println!("Directions: {:?}", directions);
    println!("Nodes: {:?}", nodes);

    let mut current_node: &str = "AAA";
    let mut step_count: i32 = 0;
    let mut found: bool = false;

    // Directions can be repeated, so we loop until we find the end node
    loop {
        for direction in directions.chars() {
            let node = nodes.get(current_node).unwrap();

            println!("{:?}: {:?} {:?}", current_node, node.0, node.1);

            if direction == 'R' {
                current_node = node.1;
            } else {
                current_node = node.0;
            }

            if current_node == "ZZZ" {
                found = true;
            }

            step_count += 1;
        }

        if found {
            break;
        }
    }

    println!("{:?}", step_count);
}
