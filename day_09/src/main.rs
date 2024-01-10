use std::fs;

use solution::solution_one;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/day_09.txt").unwrap();
    let mut readings = parser::parse_input(&file);
    
    let solution_one = solution_one(&mut readings);
    
    println!("answer one: {solution_one}");
}
