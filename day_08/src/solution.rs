use std::collections::HashMap;

pub fn solution_one(dir: &str, nodes: &HashMap<&str, (&str, &str)>) -> u128{
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

pub fn solution_two(dir: &str, nodes: &HashMap<&str, (&str, &str)>) -> u128{
    let locations = get_starters(nodes);
    let mut timestamps: Vec<u128> = Vec::with_capacity(locations.len());
    
    locations.iter().for_each(|start|{
        let mut dir = dir.chars().cycle();
        let mut steps = 0;
        let mut loc = start;
        loop{
            let dir = dir.next().unwrap();

            if let Some((l, r)) = nodes.get(loc){
                match dir{
                    'L' => loc = l,
                    _ => loc = r,
                }
                steps += 1;

                if loc.chars().last() == Some('Z'){
                    timestamps.push(steps);
                    break;
                }
            }
        }
    });
    
    dbg!(&timestamps);
    calc_lcm(&timestamps)
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

pub fn find_cycles(nodes: &HashMap<&str, (&str, &str)>, dir: &str,start: &str) -> Vec<usize>{
    let mut timestamp: Vec<usize> = Vec::with_capacity(10);
    let mut dir = dir.chars().cycle();
    
    let mut loc = start;
    let mut steps: usize = 0;
    while timestamp.len() < 10{
        if let Some(d) = dir.next(){
            if let Some((l, r)) = nodes.get(loc){
                match d{
                    'L' => loc = l,
                    _ => loc = r,
                }
                steps += 1;
               
                if loc.chars().last() == Some('Z'){
                    timestamp.push(steps);
                }
                
            }
        }
    }
    timestamp
    
}

fn calc_lcm(array: &Vec<u128>) -> u128{
    let mut lcm = 1;
    let max = *array.iter()
        .max()
        .unwrap();
    let mut abacate = array.clone();
    
    for div in 2..=max{
        let mut keep = true;
        while keep{
            keep = false;
            abacate.iter_mut().for_each(|e|{
                if *e % div == 0{
                    keep = true;
                    *e = *e/div;
                }
            });
            if keep{
                lcm = lcm * (div as u128);
            }
        }
    }

    lcm
}
