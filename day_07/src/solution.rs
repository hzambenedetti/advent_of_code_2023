
pub fn solution_one(input: &Vec<(&str, usize)>) -> usize{
    //Find each hand's type
    let mut hands = input.iter().map(|(h, bid)|{
        let hand_type = find_type(h);
        (*h, *bid, hand_type)
    }).collect::<Vec<(&str, usize, u8)>>();
    
    //Sort them
    hands.sort();

    0usize
}

fn find_type(hand: &str) -> u8{
    0u8
}
