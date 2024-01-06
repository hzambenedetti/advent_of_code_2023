pub fn solution_one(schem: &Vec<&[u8]>) -> usize{
    let mut sum: usize = 0;
    
    for (l, line) in schem.iter().enumerate(){
        for (c, &elem) in line.iter().enumerate(){
            if (elem > b'9' || elem < b'0') && elem != b'.'{
                sum += get_adj_numbers(schem, l, c).iter().sum::<usize>(); 
            } 
        }
    }

    sum
}

pub fn solution_two(schem: &Vec<&[u8]>) -> usize{
    let mut sum: usize = 0;
    
    for (l, line) in schem.iter().enumerate(){
        for (c, &elem) in line.iter().enumerate(){
            if elem == b'*'{
                let nums = get_adj_numbers(schem, l, c);
                if nums.len() == 2{
                    sum += nums[0]*nums[1];
                }
            } 
        }
    }

    sum
}


fn get_adj_numbers(m: &Vec<&[u8]>, l: usize, c: usize) -> Vec<usize>{
    let mut numbers: Vec<usize> = Vec::new();
    let mut marks: [[u8; 3]; 3] = [[0; 3]; 3];

    let bc: usize;
    let ec: usize;
    let bl: usize;
    let el: usize;

    if c == 0{
        bc = 0;
        ec = c + 1;
    }
    else{
        bc = c-1;
        if c < m[0].len() - 1{
            ec = c + 1;
        }
        else{
            ec = c;
        }
    }
    if l == 0{
        bl = 0;
        el = l + 1;
    }
    else{
        bl = l-1;
        if l < m.len() - 1{
            el = l + 1;
        }
        else{
            el = l;
        }
    }
    
    for (i, line) in m[bl..=el].iter().enumerate(){
        for (j, _) in line[bc..=ec].iter().enumerate(){
            if let Some(x) = get_number(m, bl + i, bc + j){
                if (j > 0 && marks[i][j-1] != 1) || j == 0{
                    numbers.push(x);
                }
                marks[i][j] = 1;
            }
        }
    }
    numbers
}

fn get_number(m: &Vec<&[u8]>, l: usize, c: usize) -> Option<usize>{
    if m[l][c] < b'0' || m[l][c] > b'9'{
        return None;
    }

    let mut b = c;
    let mut e = c;

    while (b as isize -1) >= 0 && m[l][b-1] >= b'0' && m[l][b-1] <= b'9'{ b -= 1};
    while (e+1) < m[l].len() && m[l][e+1] >= b'0' && m[l][e+1] <= b'9'{ e += 1};
    
    let mut number: usize = 0;
    let mut mult: usize = 1;
    m[l][b..=e].iter().rev().for_each(|alg|{
        number += (*alg - b'0') as usize *mult;
        mult *= 10;
    });
    Some(number)
}


