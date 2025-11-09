pub fn mean(x: [i64; 5]) -> Option<f64>{

    let sum: i64 = x.iter().sum();

    let count = x.len();

    let mean: Option<f64> = if count > 0 {
        Some(sum as f64 / count as f64)
    } else {
        None
    };
    
    mean

}