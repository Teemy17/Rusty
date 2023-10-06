use Rust_HW8::{cal_average_area, loadfile};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let input_filename = "layers2_1.csv";

    let layers = loadfile(input_filename)?;

    let average_areas = cal_average_area(&layers);

    let output_filename = "average_areas.html";

    let mut output_file = File::create(output_filename)?;

    writeln!(output_file, "<table>")?;

    writeln!(output_file, "<tr><th>Layer Name</th><th>Average Area</th></tr>")?;

    for (name, average) in &average_areas {
        writeln!(output_file, "<tr><td>{}</td><td>{}</td></tr>", name, average)?;
    }

    writeln!(output_file, "</table>")?;

    println!("Average areas saved to {}", output_filename);

    Ok(())  
}