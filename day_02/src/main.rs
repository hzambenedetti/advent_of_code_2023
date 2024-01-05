use std::fs;

mod solutions;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn main() {
    let file = fs::read_to_string("./src/day_02.txt").unwrap();
    let games = solutions::parse_file(&file);

    println!("part one: {};", solutions::solution_one(&games, (RED, GREEN, BLUE)));
    println!("part two: {};", solutions::solution_two(&games));
}
