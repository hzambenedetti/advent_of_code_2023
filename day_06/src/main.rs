use std::fs;

mod parser;
mod solution;

fn main() {
    let input = fs::read_to_string("input/test.txt").unwrap();
    let pairs = parser::parse_input(&input);

    dbg!(&pairs);
}
