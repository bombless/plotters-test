use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    plot("plotters-doc-data/0.png", (-500..=500).map(|x| x as f32 / 50.0).map(|x| (x, x.sin())),)?;

    let mut y = (-500..=500).map(|x| x as f32 / 50.0).map(f32::sin).collect::<Vec<_>>();
    let x = (-500..=500).map(|x| x as f32 / 50.0);
    fade(450, &mut y[..]);
    plot("plotters-doc-data/1.png", x.zip(y))?;

    Ok(())
}



fn fade(fade_length: usize, buffer: &mut [f32]) {
    for i in 0..fade_length.min(buffer.len()) {
        let g = i as f32 / fade_length as f32;
        buffer[i] *= g;
        let j = buffer.len() - 1 - i;
        buffer[j] *= g;
    }
}

fn plot(filename: &str, data: impl Iterator<Item=(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=sin(x)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(data, &RED,))?
        .label("y = sin(x)")
        .legend(|(x, y)| {
            let style = ShapeStyle {
                color: RED.mix(0.6),
                filled: true,
                stroke_width: 0,
            };
            Circle::new((x + 10, y), 6, style)
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}