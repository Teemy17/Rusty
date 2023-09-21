use Rust_HW7::to_polar;
use Rust_HW7::Point;
use csv::{ReaderBuilder, Writer, Trim};
use std::fs::File;
use std::io::Write;

fn load_cartesian_points<R: std::io::Read>(rdr: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);

    let mut cartesian_points = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let x: f64 = record[0].parse().unwrap();
        let y: f64 = record[1].parse().unwrap();
        cartesian_points.push(Point { x, y });
    }

    return cartesian_points
}

fn main() {
    let input_path = "input.csv";
    let file = File::open(input_path).unwrap();
    let cartesian_points = load_cartesian_points(file);
    let polar_points = to_polar(cartesian_points);

    let mut html_table = String::from("<table>\n");
    html_table.push_str("<tr><th>r</th><th>t</th></tr>\n");

    for point in &polar_points {
        html_table.push_str("<tr>");
        html_table.push_str(&format!("<td>{:.2}</td>", point.r));
        html_table.push_str(&format!("<td>{:.2}</td>", point.t));
        html_table.push_str("</tr>\n");
    }

    html_table.push_str("</table>");

    let mut output_file = File::create("output.html").unwrap();
    output_file.write_all(html_table.as_bytes()).unwrap();

    println!("Conversion complete. Output saved as 'output.html'.");
}
