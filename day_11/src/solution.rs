pub fn solution_one(input: &Vec<Vec<char>>) -> usize{
    let mut empty_lines = vec![true; input.len()];
    let mut empty_columns = vec![true; input[0].len()];
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    
    for (i, l) in input.iter().enumerate(){
        for (j, &c) in l.iter().enumerate(){
            if c == '#'{
                galaxies.push((i, j));
                empty_lines[i] = false;
                empty_columns[j] = false;
            }
        }
    }

    let mut total = 0;
    for (i, src) in galaxies.iter().enumerate(){
        for tgt in galaxies[(i+1)..].iter(){
            let gi = src.0.max(tgt.0);
            let si = src.0.min(tgt.0);
            let gj = src.1.max(tgt.1);
            let sj = src.1.min(tgt.1);

            let mut distance = gi-si + gj-sj;
            empty_lines[si..gi].iter()
                .for_each(|&empty|{
                    if empty{distance += 1;}
                });
            empty_columns[sj..gj].iter()
                .for_each(|&empty|{
                    if empty{distance += 1;}
                });
            total += distance;
        }
    }

    total
}
