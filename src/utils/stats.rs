pub fn mean(x: &[i64]) -> Option<f64>{

    let sum: i64 = x.iter().sum();

    let count = x.len();

    let mean: Option<f64> = if count > 0 {
        Some(sum as f64 / count as f64)
    } else {
        None
    };
    
    mean

}

pub fn vector_minus_scalar(x: &[i64], scalar: i64) -> Vec<i64> {
    x.iter().map(|&val| val - scalar).collect()
}

pub fn dot_product(x: &[i64], y: &[i64]) -> i64 {

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