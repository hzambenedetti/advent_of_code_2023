use std::fs;

use solution::{solution_one, solution_two};

mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("./input/day_04.txt").unwrap();
    let mut cards = parser::parse_input(&input);
    dbg!(&cards);

    // let mut last_id = 0usize;
    // for (id, _, _) in cards.iter(){
    //     if *id != last_id + 1{
    //        println!("Error: \nlast_id: {last_id}\nid: {id}"); 
    //     }
    //     last_id = *id;
    // }
    
    println!("length: {}", cards.len());
    let solution_one = solution_one(&mut cards);
    let solution_two = solution_two(&mut cards);
    
    println!("answer one: {solution_one}");
    println!("answer two: {solution_two}");
}
