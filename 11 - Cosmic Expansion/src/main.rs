// https://adventofcode.com/2023/day/11

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Galaxy,
}

fn main() {
    
    // EXAMPLE 1
    let contents = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    let mut map: Vec<Vec<Cell>> = contents.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Galaxy,
            _ => panic!("Invalid character"),
        }).collect()
    }).collect();

    cosmic_expansion(&mut map);        

    for row in map.iter() {
        println!("{:?}", row);
    }

}

// TODO: not working properly
fn cosmic_expansion(map: &mut Vec<Vec<Cell>>) {
    let mut new_map = map.clone();

    // TODO: any rows or columns that contain no galaxies should all actually be twice as big

    for (i, row) in new_map.clone().iter().enumerate() {
        if row.iter().all(|&cell| cell == Cell::Empty) {
            new_map.insert(i, vec![Cell::Empty; row.len()]);
        }
    }

    for i in 0..new_map[0].len() {
        if new_map.iter().all(|row| row[i] == Cell::Empty) {
            for row in new_map.iter_mut() {
                row.insert(i, Cell::Empty);
            }
        }
    }

    *map = new_map;
}