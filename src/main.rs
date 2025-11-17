mod reg;
mod utils;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ages: [i64; 5] = [22, 25, 28, 30, 46];

    let incomes: [i64; 5] = [1500, 1770, 2750, 2400, 4500];

    println!("Idades: {:?}", ages);

    let (slp, intcp)= reg::lin::fit_model(ages, incomes);

    let root = BitMapBackend::new("src/plots/image.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("y = {}x + {}", slp, intcp), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(60)
        .build_cartesian_2d(10f32..50f32, 0f32..10000f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (10..50).map(|x| x as f32).map(|x| (x, (slp * x as f64 + intcp) as f32)),
            &RED,
        ))? 
        .label(format!("y = {}x + {}", slp, intcp))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
