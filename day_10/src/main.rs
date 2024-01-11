use std::fs;

use solution::{solution_two, solution_one};

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/test.txt").unwrap();
    let map = parser::parse_input(&file);

    let solution_one = solution_one(&map);
    let solution_two = solution_two(&map);

    println!("answer one: {solution_one}");
    println!("answer two: {solution_two}");
}
