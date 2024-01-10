
pub fn solution_one(readings: &mut Vec<Vec<i32>>) -> i32{
    let mut sum: i32 = 0;
    readings.iter_mut().for_each(|r|{
        sum += predict_next(r);
    });
    sum
}

fn predict_next(reading: &mut Vec<i32>) -> i32{
    if reading.iter().sum::<i32>() == 0{return 0;}
    
    let last = *reading.last().unwrap();
    for i in 0..reading.len() - 1{
        reading[i] = reading[i+ 1] - reading[i];
    }
    reading.pop();
    last + predict_next(reading)
}
