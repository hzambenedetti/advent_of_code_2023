use std::fs;

use solution::{solution_one_simul, 
    solution_one,
    solution_two
};


mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day_06.txt").unwrap();
    let pairs = parser::parse_input_one(&input);
    
    let (time, distance) = parser::parse_input_two(&input);
    
    let solve_one_simul = solution_one_simul(&pairs);
    let solve_one = solution_one(&pairs);
    let solve_two = solution_two(time, distance);

    println!("answer one simul: {solve_one_simul}");
    println!("answer one: {solve_one}");
    println!("answer two: {solve_two}");
}
