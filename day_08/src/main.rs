use std::fs;

use solution::solution_one;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/day_08.txt").unwrap();
    let (map, dir) = parser::parse_input(&file);

    let solution_one = solution_one(dir, &map);

    println!("answer one: {solution_one}");

}
