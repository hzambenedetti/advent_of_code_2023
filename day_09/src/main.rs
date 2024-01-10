use std::fs;

use solution::solution;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/day_09.txt").unwrap();
    let mut readings = parser::parse_input(&file);
    
    let solve = 2;
    let solution = solution(&mut readings, solve);
    
    println!("answer {solve}: {solution}");
}
