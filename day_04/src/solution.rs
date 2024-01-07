
pub fn solution_one(input: &mut Vec<(usize, Vec<usize>, Vec<usize>)>) -> u32{
    let mut total: u32 = 0;
    
    for (_, win, nums) in input.iter_mut(){
        win.sort();
        nums.sort();
        
        let mut exp: u32 = 0;
        for val in win.iter(){
            if nums.binary_search(val).is_ok(){
                exp += 1;
            } 
        }

        if exp != 0{
            total += 2u32.pow(exp - 1);
        }
    }

    total    
}

pub fn solution_two(input: &mut Vec<(usize, Vec<usize>, Vec<usize>)>) -> u32{
    let mut count: Vec<usize> = vec![1; input.len() + 1];

    for (id, win, nums) in input.iter_mut(){
        win.sort();
        nums.sort();
        
        let mut points: usize = 0;
        for val in win.iter(){
            if nums.binary_search(val).is_ok(){
                points += 1;
            } 
        }
        
        if points > 0{
            let b = (*id + 1).min(count.len());
            let e = (*id+points).min(count.len());
            
            let count_id = count[*id];
            count[b..=e].iter_mut().for_each(|x|{
               *x += count_id; 
            });
        }

    }
    
    count.iter().sum::<usize>() as u32 - 1
}
