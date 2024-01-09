
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

fn parse_hand(hand: &str) -> Vec<u8>{
    hand.chars().map(|c|{
        match c{
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as u8,
        }
    }).collect::<Vec<u8>>()
}
