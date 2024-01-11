
// ============================ SOLUTIONS ============================ //

pub fn solution_one(map: &Vec<Vec<char>>) -> usize{
    //Find Starting Position
    let start = find_start(map).unwrap();

    //Go through the cycle and count steps
    
    let mut steps: usize = 0;
    let mut loc = start;
    let mut prev = start;
    while loc != start || steps == 0{
        let next = det_next(map, loc, prev);
        prev = loc;
        loc = next;
        steps += 1;
    } 
    
    steps/2
}

pub fn solution_two(map: &Vec<Vec<char>>) -> usize{
    let start = find_start(map).unwrap();

    let mut loop_map = vec![vec![false;map[0].len()];map.len()];

    let mut loc = start;
    let mut prev = start;
    loop{
        loop_map[loc.0][loc.1] = true;
        let next = det_next(map, loc, prev);
        prev = loc;
        loc = next;
        if loc == start{break;}
    }

    let mut count = false;
    let mut enclosed: usize = 0;

    for line in loop_map.iter(){
        for &p in line.iter(){
            if p {count = !count;}
            else if count{ enclosed += 1;}
        }
    }

    enclosed
}


// ============================ AUXILIARY FUNCTIONS ============================ //

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)>{
    for (i, line) in map.iter().enumerate(){
        for (j, &char) in line.iter().enumerate(){
            if char == 'S'{return Some((i, j))}
        }
    }
    None
}

fn det_next(map: &Vec<Vec<char>>, 
    loc: (usize, usize),
    prev: (usize, usize)) -> (usize, usize){

    match map[loc.0][loc.1]{
        'L' =>{
            if prev.0 < loc.0{(loc.0, loc.1+1)}
            else{(loc.0-1, loc.1)}
        },
        '|' =>{
            if prev.0 < loc.0{(loc.0+1, loc.1)}
            else{(loc.0-1, loc.1)}
        },
        '-' =>{
            if prev.1 < loc.1{(loc.0, loc.1+1)}
            else{(loc.0, loc.1-1)}
        },
        'J' =>{
            if prev.0 < loc.0{(loc.0, loc.1-1)}
            else{(loc.0-1, loc.1)}
        },
        '7' =>{
            if prev.0 > loc.0{(loc.0, loc.1-1)}
            else{(loc.0+1, loc.1)}
        },
        'F' =>{
            if prev.0 > loc.0{(loc.0, loc.1+1)}
            else{(loc.0+1, loc.1)}
        },
        'S' =>{
            let r = map[loc.0][loc.1 + 1];
            if r == '-' ||r == 'J' || r == '7'{
                return (loc.0, loc.1+1);
            }

            let l = map[loc.0][loc.1-1];
            if l=='-' ||l=='L' ||l=='F'{
                return (loc.0, loc.1-1);
            }
            
            if loc.0 < map.len() - 1{
                return(loc.0 + 1, loc.1);
            }
            (loc.0 -1, loc.1)
        },
        _ => (0, 0)
    }
}


