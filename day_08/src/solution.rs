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

pub fn solution_two(dir: &str, nodes: &HashMap<&str, (&str, &str)>) -> usize{
    let mut dir = dir.chars().cycle();
    let mut locations = get_starters(nodes);
    let mut steps: usize = 0;
    let mut finished: usize = 0;

    dbg!(&locations);
    
    while finished != locations.len(){
        finished = 0;
        if let Some(d) = dir.next(){
            locations.iter_mut().for_each(|loc|{
                if let Some(&(l, r)) = nodes.get(loc){
                    match d{
                        'L' => *loc = l,
                        _ => *loc = r,
                    }

                    if let Some(last) = loc.chars().last(){
                        if last == 'Z'{
                            finished += 1;
                        }
                    }
                }
            });
            steps += 1;
        } 
    }

    steps
}

fn get_starters<'a>(nodes: &HashMap<&'a str, (&str, &str)>) -> Vec<&'a str>{
    nodes.keys()
        .filter_map(|&k|{
            if let Some(last) = k.chars().last(){
                if last == 'A'{
                    return Some(k);
                }
            }
            None
        }).collect::<Vec<&str>>()
}
