pub fn parse_input(input: &str) -> Vec<(usize ,Vec<usize>, Vec<usize>)>{
    let mut cards: Vec<(usize ,Vec<usize>, Vec<usize>)> = Vec::new();

    input.lines().for_each(|line|{
        if let Some(card) = parse_line(line){
            cards.push(card);
        }
    });

    cards
}

fn parse_line(line: &str) -> Option<(usize, Vec<usize>, Vec<usize>)>{
    
    let mut winning: Vec<usize> = Vec::new();
    let mut card_nums: Vec<usize> = Vec::new();
    
    let (tag, numbers);
    match line.split_once(':'){
        Some(x) =>{
            (tag, numbers) = x;
        },
        None => return None,
    }

    let (win_str, nums_str);
    match numbers.split_once('|'){
        Some(x) =>{
            (win_str, nums_str) = x;
        },
        None => return None
    }

    win_str.split(' ').for_each(|num|{
       if let Some(x) = num.parse::<usize>().ok(){
            winning.push(x);
        } 
    });
    
    nums_str.split(' ').for_each(|num|{
        if let Some(x) = num.parse::<usize>().ok(){
            card_nums.push(x);
        } 
    });
    
    let card_id;
    match parse_card_id(tag){
        Some(id) => card_id = id,
        None => return None,
    }
    
    Some((card_id, winning, card_nums))
}

fn parse_card_id(tag: &str) -> Option<usize>{
    if let Some(id) = tag.split(' ').last(){
        if let Some(num) = id.parse::<usize>().ok(){
            return Some(num);
        }
    } 
    None
}
