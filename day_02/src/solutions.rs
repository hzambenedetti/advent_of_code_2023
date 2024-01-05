#[derive(Debug)]
pub struct Play{
    pub red: usize,
    pub green: usize,
    pub blue: usize
}

type Game = Vec<Play>;

//============================================ SOLUTIONS =======================================//

pub fn solution_one(games: &[(usize, Game)], (r, g, b): (usize, usize, usize)) -> usize{
    let mut total: usize = 0;
    for (id, game) in games.iter(){
        let mut add: bool = true;
        for play in game.iter(){
            if play.red > r || play.green > g || play.blue > b{
                add = false;
                break;
            } 
        }
        if add{
            total += id;
        }
    }
    total
}

pub fn solution_two(games: &[(usize, Game)]) -> usize{
    let mut total: usize = 0;

    for (_, game) in games.iter(){
        let mut r_min = usize::MIN;
        let mut g_min = usize::MIN;
        let mut b_min = usize::MIN;

        for play in game.iter(){
            r_min = r_min.max(play.red);
            g_min = g_min.max(play.green);
            b_min = b_min.max(play.blue);
        }

        total += r_min * g_min *b_min;
    }
    total
}

//============================================ AUXILIARY FUNCTIONS =======================================//

pub fn parse_file(file: &str) -> Vec<(usize, Game)>{
    let mut games: Vec<(usize, Game)> = Vec::new();
    
    file.lines().for_each(|line|{
        if let Some(x) = parse_line(line){
            games.push(x);
        }
    });

    games
}


fn parse_line(line: &str) -> Option<(usize, Game)>{
    let line = line.split(':').collect::<Vec<&str>>();

    
    let game;
    match line.get(1){
        Some(&g) => game = g,
        None => return None,
    };

    let mut plays: Game = Vec::new();
    
    for play in game.split(';'){
        if let Some(p) = parse_play(play){
            plays.push(p);
        }
    }

    if game.is_empty(){
        return None;
    }

    if let Some(id) = parse_game_id(line[0]){
        return Some((id, plays));
    }
    None
}

fn parse_play(play: &str) -> Option<Play>{
    let mut play_ret = Play{red: 0, green: 0, blue: 0};
    
    for color in play.split(','){
        let parts = color.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 2{
            continue;
        }

        let num;
        match parts[0].parse::<usize>(){
            Ok(x) => num = x,
            _ => continue,
        }

        match parts[1]{
            "red" => play_ret.red = num,
            "green" => play_ret.green = num,
            "blue" => play_ret.blue = num,
            _ => continue,
        }
    }    
    
    Some(play_ret)
}

fn parse_game_id(string: &str) -> Option<usize>{
    if let Some(num) = string.split_whitespace().last(){
        return num.parse::<usize>().ok();
    } 
    None
}


