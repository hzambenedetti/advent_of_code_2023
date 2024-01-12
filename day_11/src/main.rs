use std::fs;

use solution::solution_one;

mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day_11.txt").unwrap();

    let map = parser::parse_input(&input);
    
    let solution_one = solution_one(&map);

    println!("answer one: {solution_one}");
}
