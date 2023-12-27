use std::fs;

mod solution;

fn main() {
    let input = fs::read_to_string("./src/day_01.txt").unwrap();
    let mut total: usize = 0;

    for line in input.lines(){
        match solution::solution_pt02(line){
            Some(x) => {
                total += x;
                println!("{x}");
            },
            None => (),
        };
    }
    
    println!("total: {total}");
}
