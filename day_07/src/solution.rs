use std::cmp::Ordering;

// ============================= SOLUTION ONE =============================

pub fn solution(input: &Vec<(&str, usize)>, solve: usize) -> usize{
    //Find each hand's type
    let mut hands = input.iter().map(|(h, bid)|{
        let transformed_hand;
        let hand_type;
        if solve == 2{
            transformed_hand = transform_hand_two(h);
            hand_type = find_type_two(&transformed_hand);
        }
        else{
            transformed_hand = transform_hand_one(h);
            hand_type = find_type_one(&transformed_hand);
        }

        (transformed_hand, *bid, hand_type)
    }).collect::<Vec<(Vec<u8>, usize, u8)>>();
    
    //Sort them
    hands.sort_by(|a, b|{
        if a.2 > b.2{return Ordering::Greater;}
        if a.2 < b.2{return Ordering::Less;}
        
        let zipped = a.0.iter().zip(b.0.iter());
        for (x, y) in zipped{
            if x.cmp(y) != Ordering::Equal{
                return x.cmp(y);        
            }
        }
        Ordering::Equal
    });

    let mut total: usize = 0;
    hands
        .iter()
        .zip(1..=hands.len())
        .for_each(|((_h, b, _t), i)| total += i*b);
    total
}

fn find_type_one(hand: &Vec<u8>) -> u8{
    let mut card_ocurr: [u8; 15] = [0; 15];
    let mut reps: [u8; 6] = [0; 6];

    hand.iter().for_each(|&x| card_ocurr[x as usize] += 1);
    card_ocurr[2..].iter().for_each(|&x| reps[x as usize] += 1);

    if reps[5] == 1{return 6;} //Five of a Kind
    if reps[4] == 1{return 5;} //Four of a Kind
    if reps[3] == 1 && reps[2] == 1{return 4;} //Full House
    if reps[3] == 1{return 3;} //Three of a Kind
    if reps[2] == 2{return 2;} //Two pair
    if reps[2] == 1{return 1;} //One pair
    return 0; //High Card
}


fn transform_hand_one(hand: &str) -> Vec<u8>{
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

// ============================= SOLUTION TWO =============================

fn transform_hand_two(hand: &str) -> Vec<u8>{
    hand.chars().map(|c|{
        match c{
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            _ => c.to_digit(10).unwrap() as u8,
        }
    }).collect::<Vec<u8>>()
}

fn find_type_two(hand: &Vec<u8>) -> u8{
    let mut card_ocurr: [u8; 15] = [0; 15];
    let mut reps: [u8; 6] = [0; 6];

    hand.iter().for_each(|&x| card_ocurr[x as usize] += 1);
    
    //Doesn't take the jokers into account
    card_ocurr[2..].iter().for_each(|&x| reps[x as usize] += 1);
    
    let jokers = card_ocurr[1];
    let hand_type;
    if reps[5] == 1{
        hand_type = 6;
    }
    else if reps[4] == 1{
        if jokers == 1{hand_type = 6;}
        else{hand_type = 5;}
    }
    else if reps[3] == 1{
        if jokers == 2{hand_type = 6;}
        else if jokers == 1{hand_type = 5;}
        else if reps[2] == 1{hand_type = 4;}
        else{hand_type = 3;}
    }
    else if reps[2] == 2{
        if jokers == 1{hand_type = 4;}
        else{hand_type = 2;}
    }
    else if reps[2] == 1{
        if jokers == 3{hand_type = 6;}
        else if jokers == 2{hand_type = 5;}
        else if jokers == 1{hand_type = 3;}
        else{hand_type = 1};
    }
    else if reps[1] > 0{
        if jokers == 4{hand_type = 6;}
        else if jokers == 3{hand_type = 5;}
        else if jokers == 2{hand_type = 3;}
        else if jokers == 1{hand_type = 1;}
        else{hand_type = 0;}
    }
    else{
        hand_type = 6;
    }
    
    hand_type
    
}
