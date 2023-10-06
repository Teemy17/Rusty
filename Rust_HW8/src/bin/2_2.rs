use Rust_HW8::{cal_average_area, loadfile, save_average_areas};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input_filename = "layers2_1.csv";

    let layers = loadfile(input_filename)?;

    let average_areas = cal_average_area(&layers);

    let output_filename = "average_areas.csv";

    let output_file = File::create(output_filename)?;

    save_average_areas(output_file, average_areas)?;

    Ok(())
}

