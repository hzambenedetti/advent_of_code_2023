use std::fs;

use solution::solution;

mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day_11.txt").unwrap();

    let map = parser::parse_input(&input);
    
    let solution_one = solution(&map, 1);
    let solution_two = solution(&map, 2);

    println!("answer one: {solution_one}");
    println!("answer two: {solution_two}");
}
