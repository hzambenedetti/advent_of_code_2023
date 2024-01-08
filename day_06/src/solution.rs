

// Solves the first part by solving the quadratic equation
// that describes the travelled distance in funtion of the held time
pub fn solution_one(input: &Vec<(usize, usize)>) -> usize{
    input.iter().map(|&(t, rec)|{
        let t_one = (-(t as f64) + ((t*t - 4*rec) as f64).sqrt())/(-2.0f64);
        let t_two = (-(t as f64) - ((t*t - 4*rec) as f64).sqrt())/(-2.0f64);

        let t_one = (t_one + 1.0).floor() as usize;
        let t_two = (t_two - 1.0).ceil() as usize;

        t_two - t_one + 1
    }).product::<usize>()
}

// Solves the first part by inputing each instant dt in the interval (0, T)
// and counting how many intants dt resulted in a distance greater than
// the current record;
pub fn solution_one_simul(input: &Vec<(usize, usize)>) -> usize{
    let mut possibilities: Vec<usize> = Vec::with_capacity(input.len());
    for &(t, rec) in input.iter(){
        let mut counter: usize = 0;
        for i in 1..(t as i32){
            if -i*i + i*(t as i32) > rec as i32{
                counter += 1;
            }
        }
        possibilities.push(counter);
    }
    possibilities.iter().product::<usize>()
}


pub fn solution_two(t: usize, rec: usize) -> usize{
    let t_one = (-(t as f64) + ((t*t - 4*rec) as f64).sqrt())/(-2.0f64);
    let t_two = (-(t as f64) - ((t*t - 4*rec) as f64).sqrt())/(-2.0f64);

    let t_one = (t_one + 1.0).floor() as usize;
    let t_two = (t_two - 1.0).ceil() as usize;

    t_two - t_one + 1
}
