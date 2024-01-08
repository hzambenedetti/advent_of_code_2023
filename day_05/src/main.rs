use std::fs;

mod solution;
mod parser;

fn main() {
    let input = fs::read_to_string("input/test.txt").unwrap();
    let (seeds, maps) = parser::parse_input(&input);

    dbg!(&seeds);
    dbg!(&maps);
}
