use std::fs;

mod solution;
mod parser;

fn main() {
    let file = fs::read_to_string("./input/day_03.txt").unwrap();
    let schem = parser::parse_input(&file);
    let solve_one = solution::solution_one(&schem);
    let solve_two = solution::solution_two(&schem);
    
    println!("answer one: {solve_one}");
    println!("answer two: {solve_two}");
}
