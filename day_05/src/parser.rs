#[derive(Debug)]
pub struct Map{
    pub id: usize,
    pub src: String,
    pub dest: String,
    pub mapping: Vec<(usize, usize, usize)>,
}



pub fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>){
    let mut split_input = input.split("\n\n");
    
    let seeds: Vec<usize> = parse_seeds(split_input.next().unwrap()).unwrap();

    let mut maps: Vec<Map> = Vec::new();
    split_input.for_each(|map|{
        if let Some(m) = parse_map(map){
            maps.push(m);
        }
    });

    (seeds, maps)
}


fn parse_seeds(input: &str) -> Option<Vec<usize>>{
    if let Some((_, seeds)) = input.split_once(":"){
        return Some(
            seeds.split(' ').filter_map(|v| v.parse().ok()).collect()
        );
    }
    None
}

fn parse_map(input: &str) -> Option<Map>{
    let mut lines = input.lines();
    let info = lines.next().unwrap();
    let (src, dest) = parse_map_info(info).unwrap();
    
    let mut mapping: Vec<(usize, usize, usize)> = Vec::new();

    for line in lines{
        if line.len() > 0{
            let mut vals: [usize; 3] = [0; 3];
            line.split(' ').enumerate().for_each(|(i, x)|{
                vals[i] = x.parse().unwrap();
            });
            mapping.push((
                vals[1],
                vals[0],
                vals[2]
            ));
        }
    }
    
    mapping.sort_by_key(|a| a.0);

    Some(
        Map{
            id: 0,
            src,
            dest,
            mapping,
        }   
    )
}

fn parse_map_info(input: &str) -> Option<(String, String)>{
    if let Some((info, _)) = input.split_once(' '){
       if let Some((src, dest)) = info.split_once("-to-")   {
            return Some((src.to_string(), dest.to_string()));
        } 
    }
    None
}


