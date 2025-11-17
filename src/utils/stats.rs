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

pub fn vector_minus_scalar(x: [i64; 5], scalar: i64) -> [i64; 5] {

    let result: [i64; 5] = x.map(|val| val - scalar);

    result

}

pub fn dot_product(x: [i64; 5], y: [i64; 5]) -> i64 {

    let product: i64 = x.iter()
                        .zip(y.iter())
                        .map(|(a, b)| a * b)
                        .sum();

    product

}

pub fn round_number(num: f64, decimal_places: u32) -> f64 {

    let factor = 10f64.powi(decimal_places as i32);

    (num * factor).round() / factor

}