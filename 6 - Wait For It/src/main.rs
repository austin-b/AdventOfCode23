// https://adventofcode.com/2023/day/6

fn main() {

    // EXAMPLE
    //let content = "Time:      7  15   30\nDistance:  9  40  200";

    // INPUT
    let content = "Time:        56     97     78     75\nDistance:   546   1927   1131   1139";

    let time = content.lines().nth(0).unwrap().split_whitespace().skip(1).into_iter().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
    let distance = content.lines().nth(1).unwrap().split_whitespace().skip(1).into_iter().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();

    println!("time: {:?}", time);
    println!("distance: {:?}", distance);
    
    let ways = ways_to_win(time, distance);
    println!("ways to win: {:?}", ways);
}

fn ways_to_win(time: i64, distance: i64) -> i64 {
    let mut ways = 0;

    let mut charge_time: i64 = 0;

    // brute force -- can probably figure this out mathematically
    loop {
        charge_time += 1;
        let distance_traveled = distance_traveled(time, charge_time);

        if distance_traveled > distance {
            // println!("distance_traveled: {:?}, charge_time: {:?}", distance_traveled, charge_time);
            ways += 1;
        } else if distance_traveled == 0 {
            break;
        }
    }

    ways
}

/* 
figured this out mathematically:
    if distance_traveled = velocity * time_of_travel
    and time_of_travel = total_time - charge_time
    and velocity = charge_time
    then distance_traveled = charge_time * (total_time - charge_time)
*/
fn distance_traveled(total_time: i64, charge_time: i64) -> i64 {
    charge_time * (total_time - charge_time)
}