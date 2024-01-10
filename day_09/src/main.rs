use std::fs;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/test.txt").unwrap();
    let readings = parser::parse_input(&file);
    dbg!(&readings);
}
