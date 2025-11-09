use crate::utils::stats;

pub fn fit_model(arr_x: [i64; 5], _arr_y: [f64; 5]){

    let x_mean: Option<f64> = stats::mean(arr_x);

    println!("Media de x: {:?}", x_mean);
}