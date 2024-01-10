use std::collections::HashMap;

pub fn parse_input(input: &str) -> (HashMap<&str, (&str, &str)>, &str){
    let mut lines = input.lines().filter_map(|l|{
        if l.len() > 0{ return Some(l);}
        None
    });
    
    let directions = lines.next().unwrap();
    
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.for_each(|line|{
        if let Some((key, value)) = parse_line(line){
            map.insert(key, value);
        }
    });
    
    (map, directions)
}

fn parse_line(input: &str) -> Option<(&str, (&str, &str))>{
    let rem: &[char] = &[' ', ')', '('];
    if let Some((node, adj)) = input.split_once('='){
        if let Some((l, r)) = adj.split_once(','){
            return Some((
                node.trim(),
                (l.trim_matches(rem),r.trim_matches(rem))
            ));
        }
    }
    None
}


