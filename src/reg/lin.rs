use crate::utils::stats::{self, round_number};

pub fn fit_model(arr_x: [i64; 5], arr_y: [i64; 5]) -> (f64, f64) {

    let x_mean: Option<f64> = stats::mean(arr_x);

    let y_mean: Option<f64> = stats::mean(arr_y);

    let x_by_mean: [i64; 5] = stats::vector_minus_scalar(arr_x, x_mean.unwrap() as i64);

    let y_by_mean: [i64; 5] = stats::vector_minus_scalar(arr_y, y_mean.unwrap() as i64);

    let dot_prod_xy: i64 = stats::dot_product(x_by_mean, y_by_mean);

    let dot_prod_xx: i64 = stats::dot_product(x_by_mean, x_by_mean);

    let slope: f64 = dot_prod_xy as f64 / dot_prod_xx as f64;
    
    let intercept: f64 = y_mean.unwrap() - slope * x_mean.unwrap();

    println!("Coeficiente angular (slope): {:?}", slope);

    println!("Coeficiente linear (intercept): {:?}", intercept);

    (round_number(slope, 2), round_number(intercept, 2))
}