// https://adventofcode.com/2023/day/12

fn main() {
    
    // EXAMPLE
    let contents = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    let conditions: Vec<(&str, Vec<usize>)> = contents.lines().map(|line: &str| {
        let records: Vec<&str> = line.split(" ").collect();
        let mut groups: Vec<usize> = records[1].split(",").map(|group| group.parse::<usize>().unwrap()).collect();
        groups.sort();
        (records[0], groups)
    }).collect();

    for record in &conditions {
        println!("{:?}", record);
    }

    let arrangements: Vec<usize> = find_number_of_arrangements(&conditions);

    println!("\nsum of arrangements: {:?}", arrangements.iter().sum::<usize>());
}

fn find_number_of_arrangements(conditions: &Vec<(&str, Vec<usize>)>) -> Vec<usize> {
    

    vec![1,2,3]
}
