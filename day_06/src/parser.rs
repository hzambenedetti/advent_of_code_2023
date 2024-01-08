

pub fn parse_input_one(input: &str) ->Vec<(usize, usize)>{
    let mut lines = input.lines().filter_map(|line|{
        if line.is_empty(){
           return None; 
        }
        Some(line)
    });

    let time = parse_line_one(lines.next().unwrap()).unwrap();
    let distance = parse_line_one(lines.next().unwrap()).unwrap();
    
    time
        .iter()
        .zip(distance.iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}

pub fn parse_input_two(input: &str) ->(usize, usize){
    let mut lines = input.lines().filter_map(|line|{
        if line.is_empty(){
           return None; 
        }
        Some(line)
    });

    let time = parse_line_two(lines.next().unwrap()).unwrap();
    let distance = parse_line_two(lines.next().unwrap()).unwrap();
    
    (time, distance)
}


fn parse_line_one(input: &str) -> Option<Vec<usize>>{
    if let Some((_, values)) = input.split_once(':'){
        return Some(values.split(' ').filter_map(|x|{
            x.parse::<usize>().ok()
        }).collect())
    }

    None
}

fn parse_line_two(input: &str) -> Option<usize>{
    if let Some((_, values)) = input.split_once(':'){
        let mut total: usize = 0;
        let mut mult: usize = 1;

        values.chars().rev().for_each(|c|{
            if let Some(x) = c.to_digit(10){
                total += mult * x as usize;
                mult *= 10;
            }
        });

        return Some(total);
    }
    None
}
