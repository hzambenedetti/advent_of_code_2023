use std::fs;

use solution::{solution_one, solution_two};

mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("./input/day_04.txt").unwrap();
    let mut cards = parser::parse_input(&input);
    dbg!(&cards);
    
    cards.iter_mut().for_each(|(_, win, nums)|{
        win.sort_unstable();
        nums.sort_unstable();
    });

    println!("length: {}", cards.len());
    let solution_one = solution_one(&cards);
    let solution_two = solution_two(&cards);
    
    println!("answer one: {solution_one}");
    println!("answer two: {solution_two}");
}
