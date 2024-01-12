
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

    let mut loop_map = vec![vec!['.';map[0].len()+2];map.len()+2];

    let mut loc = start;
    let mut prev = start;
    loop{
        loop_map[loc.0+1][loc.1+1] = map[loc.0][loc.1];
        let next = det_next(map, loc, prev);
        prev = loc;
        loc = next;
        if loc == start{break;}
    }

    let mut enclosed: usize = 0;

    for (i, line) in loop_map.iter_mut().enumerate(){
        let mut start = 'x';
        let mut aux = 0;
        for (j, p) in line.iter_mut().enumerate(){
            if *p == 'x' {
                let c = map[i][j];
                if start == 'x'{
                    if c== 'J' ||c== '7'||c== '|'{
                        start = c;
                    }
                }
                else{
                    if c =='L' ||c =='F'||c=='|'{
                       enclosed += aux;
                    }
                    start = 'x';
                    aux = 0;
                }
            }
            else if start != 'x'{
                aux += 1;
                *p = 'I';
            }
        }
    }
    print_vec(&loop_map);

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

fn print_vec(vec: &Vec<Vec<char>>){
    for line in vec.iter(){
        print!("\n");
        for c in line.iter(){
            print!("{c}");
        }
    }
    println!("\n");
}

fn floyd_warshall(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    const INF: usize = usize::MAX/2;
    let size = graph.len() * graph.len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; size]; size];
    
    for k in 0..dist.len(){
        for i in 0..dist.len(){
            for j in 0..dist.len(){
                if graph[i][k] + graph[k][j] < dist[i][j]{
                    dist[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    dist
} 


