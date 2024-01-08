
pub fn parse_input(input: &str) -> Vec<(&str, usize)>{
    input.lines().filter_map(|line|{
        parse_line(line)
    }).collect::<Vec<(&str, usize)>>()
}

fn parse_line(input: &str) -> Option<(&str, usize)>{
    if let Some((hand, bid)) = input.split_once(' '){
        if let Ok(num) = bid.parse::<usize>(){
            return Some((hand, num));
        }
    }
    None
}
