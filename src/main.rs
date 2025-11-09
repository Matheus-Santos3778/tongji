mod reg;
mod utils;

fn main() {

    let ages: [i64; 5] = [22, 25, 28, 30, 36];

    let incomes: [f64; 5] = [1500.0, 1770.0, 2500.0, 2400.0, 3000.0];

    println!("Idades: {:?}", ages);

    reg::lin::fit_model(ages, incomes);
}
