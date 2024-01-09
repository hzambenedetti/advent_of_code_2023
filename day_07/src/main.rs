use std::fs;

use solution::solution;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/day_07.txt").unwrap();
    let input = parser::parse_input(&file);
    
    let solution_one = solution(&input, 2);

    println!("answer one: {solution_one}");
}
