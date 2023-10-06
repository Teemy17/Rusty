use Rust_HW8::{cal_average_area_min_max, loadfile};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let input_filename = "layers2_1.csv"; 

    let layers = loadfile(input_filename)?;

    let min_max_average_areas = cal_average_area_min_max(&layers);

    let output_filename = "average_areas_max_min.html";

    let mut output_file = File::create(output_filename)?;

    writeln!(output_file, "<table>")?;

    writeln!(
        output_file,
        "<tr><th>Layer Name</th><th>Average Area</th><th>Maximum Area</th><th>Minimum Area</th></tr>"
    )?;

    for (name, avg, max, min) in &min_max_average_areas {
        writeln!(
            output_file,
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            name, avg, max, min
        )?;
    }

    writeln!(output_file, "</table>")?;

    println!("Average areas (with Min/Max) saved to {}", output_filename);

    Ok(())
}