use std::{fs::File, io::Read, error::Error};

use plotters::prelude::*;
use rand::prelude::*;
use sim::{Key, Layout, Speeds, sim};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = vec![];
    
    let mut rng = thread_rng();
    let mut keys = Key::all();
   
    let speeds = Speeds::init();
    let mut txt = String::new();
    File::open("text/text.txt")?.read_to_string(&mut txt)?;

    for i in 0..1000 {
        keys.shuffle(&mut rng);
        let layout = Layout::new(keys);
        data.push(sim(&layout, &speeds, &txt) as u32);
        println!("sim #{}", i);
        if i%20 == 5 {
            draw_plot(&data)?;
        }
    }

    
    Ok(())
}

fn draw_plot(data: &Vec<u32>) -> Result<(), Box<dyn Error>> {
    let mut buckets = [0; 20];
    for speed in data {
        let bucket = speed / 10000;
        buckets[bucket as usize - 5] += 1;
    }

    let points = buckets.into_iter().enumerate().map(|(i, x)| ((i * 10000 + 50000) as f64, (x as f64) / (data.len() as f64) * 100.)).collect::<Vec<_>>();

    let root = BitMapBackend::new("plots/hist_layouts.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(50)
        .caption("Histogram", ("Arial", 50).into_font())
        .x_label_area_size(80)
        .y_label_area_size(120)
        .build_cartesian_2d(50_000f64..250_000f64, 0f64..100f64)?;

    chart.configure_mesh()
        .x_desc("Typing time (less is better)")
        .label_style(("Arial", 30).into_font())
        .y_desc("Percentage of layouts")
        .max_light_lines(1)
        .draw()?;

    chart.draw_series(LineSeries::new(points, BLACK).point_size(5)).unwrap();
    // chart.draw_series(LineSeries::new([30., 40., 50., 60., 30.], ShapeStyle { color: RGBAColor , filled: todo!(), stroke_width: todo!() }));

    Ok(())
}