pub fn parse_input(input: &str) -> Vec<&[u8]>{
    let mut matrix: Vec<&[u8]> = Vec::new();
    
    input.lines().for_each(|l|{
        matrix.push(l.as_bytes())
    });

    matrix
}
