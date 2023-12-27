//////////////////////////////////////// SOLUTIONS ////////////////////////////////////////////////

pub fn solution_pt01(line: &str) -> Option<usize>{
    let mut num: usize = 0;
    
    for &char in line.as_bytes().iter(){
        if char >= 48 && char <= 57{
            num += 10*(char -48) as usize;
            break;
        }
    }
    
    for &char in line.as_bytes().iter().rev(){
        if char >= 48 && char <= 57{
            num += (char - 48) as usize;
            break;
        }
    }

    if num > 0{
        return Some(num);
    }

    None
}

pub fn solution_pt02(line: &str) -> Option<usize>{
    const NUMBERS: [&str; 20] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                         "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
                    ];
    let mut position: [Option<(usize, usize)>; 20] = [None; 20];

    for (i, num) in NUMBERS.iter().enumerate(){
        position[i]= substring(line, num)
    }
    
    let mut first = (0, usize::MAX);
    let mut last = (0, 0);

    for (i, &pos) in position.iter().enumerate(){
        match pos{
            Some(p) =>{
                if p.0 < first.1{
                    first = (i % 10, p.0);
                }
                if p.1 >= last.1{
                    last = (i % 10, p.1);
                }
            },
            None => (),
        };
    }
    
    let num = first.0 * 10 + last.0;
    Some(num)
}

///////////////////////////// AUXILIARY FUNCTIONS /////////////////////////////////////

fn substring(str: &str, pat: &str) -> Option<(usize, usize)>{
    let pat: Vec<char> = pat.chars().collect();
    let str: Vec<char> = str.chars().collect();
    
    let mut pos: (usize, usize) = (usize::MAX, usize::MAX);

    for (i, &char) in str.iter().enumerate(){
        if char == pat[0]{
            let mut found = true;
            for j in 1..pat.len(){
                if i+j >= str.len() || str[i + j] != pat[j]{
                    found = false;
                    break;
                }
            }
            if found{
                pos.1 = i;
                if pos.0 == usize::MAX{
                    pos.0 = i;
                }
            }
        } 
    }
    
    if pos != (usize::MAX, usize::MAX){
        return Some(pos);
    }
    None
}   
