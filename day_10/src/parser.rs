
pub fn parse_input(input: &str) -> Vec<Vec<char>>{
    input
        .lines()
        .filter_map(|l|{
            if l.len() > 0{
                Some(l.chars().collect::<Vec<char>>())
            } else{
                None
            }
        }).collect::<Vec<Vec<char>>>()
}

