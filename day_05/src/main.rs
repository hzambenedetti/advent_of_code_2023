use std::fs;

use solution::solution_one;

mod solution;
mod parser;

fn main() {
    let input = fs::read_to_string("input/day_05.txt").unwrap();
    let (seeds, maps) = parser::parse_input(&input);
    
    let solution_one = solution_one(&seeds, &maps);

    println!("answer one: {solution_one}");
}
