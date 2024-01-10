use std::collections::HashMap;

pub fn solution_one(dir: &str, nodes: &HashMap<&str, (&str, &str)>) -> usize{
    let mut dir = dir.chars().cycle();
    
    let mut steps = 0;
    let mut location = "AAA";
    while location != "ZZZ"{
        if let Some(dir) = dir.next(){
            if let Some((l, r)) = nodes.get(location){
                match dir{
                    'L' => location = l,
                    _ => location = r,
                }
                steps += 1;
            }
        }
    }

    steps
}
