use std::fs;

mod solution;
mod parser;

fn main() {
    let file = fs::read_to_string("./input/day_03.txt").unwrap();
    let schem = parser::parse_input(&file);
    let sum = solution::solution_two(&schem);
    println!("answer: {sum}");
}
