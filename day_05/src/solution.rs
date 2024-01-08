use crate::parser::Map;

pub fn solution_one(seeds: &Vec<usize>, maps: &Vec<Map>) -> usize{
    let mut min: usize = usize::MAX;
    
    for &seed in seeds.iter(){
        let mut mapped_seed = seed;
        for map in maps.iter(){
            for &(src, dest, len) in map.mapping.iter(){
                if mapped_seed >= src && mapped_seed <= src + len - 1{
                    mapped_seed = dest + (mapped_seed - src);
                    break;
                }
            }
        }
        min = min.min(mapped_seed);
    }

    min
}
