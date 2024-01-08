pub fn parse_input(input: &str) ->Vec<(usize, usize)>{
    
    let mut lines = input.lines().filter_map(|line|{
        if line.is_empty(){
           return None; 
        }
        Some(line)
    });

    let time = parse_line(lines.next().unwrap()).unwrap();
    let distance = parse_line(lines.next().unwrap()).unwrap();
    
    time
        .iter()
        .zip(distance.iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}

fn parse_line(input: &str) -> Option<Vec<usize>>{
    if let Some((_, values)) = input.split_once(':'){
        return Some(values.split(' ').filter_map(|x|{
            x.parse::<usize>().ok()
        }).collect())
    }

    None
}
