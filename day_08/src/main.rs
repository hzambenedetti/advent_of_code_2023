use std::fs;

mod parser;
mod solution;

fn main() {
    let file = fs::read_to_string("input/day_08.txt").unwrap();
    let count = file.lines().count();
    let (map, dir) = parser::parse_input(&file);
    dbg!(&map);
    println!("Map size: {}", map.len());
    println!("lines: {count}");
    println!("{dir}");
}
