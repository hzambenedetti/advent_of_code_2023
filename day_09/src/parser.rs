pub fn parse_input(input: &str) -> Vec<Vec<i32>>{
    let lines = input.lines().filter_map(|l|{
        if l.len() > 0{return Some(l);}
        None
    });

    lines.filter_map(|line|{
        parse_line(line)
    }).collect::<Vec<Vec<i32>>>()
}


fn parse_line(input: &str) -> Option<Vec<i32>>{
    let readings = input.split(' ')
        .filter_map(|x|{x.parse::<i32>().ok()})
        .collect::<Vec<i32>>();

    if readings.is_empty(){None} 
    else{Some(readings)}
}
