
use plotters::prelude::*;



use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn visualize(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("resultGraph.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE).ok();
    let root = root.margin(10, 10, 10, 10);
    let vectors = read_csv_std(filename);
    let points: Vec<(f64,f64)> = vectors.0.iter().zip(vectors.1.iter()).map(|(&x,&y)| (x,y)).collect();
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f64..2f64, -7f64..1f64)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        points.clone(),
        &RED,
    ))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        points.clone(),
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    root.present()?;
    Ok(())
}

fn read_csv_std(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let mut x_vec = Vec::new();
    let mut u_vec = Vec::new();

    // 1. Otwieramy plik (jeśli istnieje)
    let path = Path::new(filename);
    let file = File::open(&path).expect("Nie można otworzyć pliku CSV");

    // 2. Używamy BufReader dla wydajności
    let reader = io::BufReader::new(file);

    // 3. Iterujemy po liniach
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Błąd odczytu linii");

        // Pomijamy nagłówek (pierwszą linię: "x,u(x)")
        if index == 0 {
            continue;
        }

        // 4. Dzielimy linię po przecinku
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() >= 2 {
            // 5. Parsujemy tekst na liczby f64
            let x_val: f64 = parts[0].trim().parse().expect("To nie jest liczba (x)");
            let u_val: f64 = parts[1].trim().parse().expect("To nie jest liczba (u)");

            x_vec.push(x_val);
            u_vec.push(u_val);
        }
    }

    (x_vec, u_vec)
}