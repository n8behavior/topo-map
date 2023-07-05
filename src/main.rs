use csv::ReaderBuilder;
use std::error::Error;
use std::io::{stdout, Write};
use termion::color;

struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() -> std::io::Result<()> {
    let data = read_csv("elevation_data.csv").expect("Unable to read CSV");
    let (min, max) = min_max(&data);
    let gradient = create_gradient();
    print_heatmap(&data, min, max, &gradient)
}

fn read_csv(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row: Vec<u8> = record.iter().map(|x| x.parse().unwrap()).collect();
        data.push(row);
    }

    Ok(data)
}

fn min_max(data: &[Vec<u8>]) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = u8::MIN;

    for row in data {
        for &value in row {
            min = min.min(value);
            max = max.max(value);
        }
    }

    (min, max)
}

fn create_gradient() -> Vec<Rgb> {
    let mut gradient = Vec::new();
    for value in 0..=255 {
        gradient.push(Rgb {
            red: value,
            green: value,
            blue: value,
        });
    }
    gradient
}

fn print_heatmap(data: &[Vec<u8>], min: u8, max: u8, gradient: &[Rgb]) -> std::io::Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();
    let range = (max - min) as f32;
    for row in data {
        for &value in row {
            // Compute ratio within 0 to 1
            let ratio = (value - min) as f32 / range; 
            // Scale to 0..255
            let scaled_value = (ratio * 255.0).round() as u8; 
            //Get the right color for the relative elevation
            let color = &gradient[scaled_value as usize];
            write!(
                handle,
                "{}  ",
                color::Rgb(color.red, color.green, color.blue).bg_string()
            )?;
        }
        writeln!(handle)?;
    }
    Ok(())
}
