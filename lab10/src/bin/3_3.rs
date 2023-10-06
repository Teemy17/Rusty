use rand::Rng;
use std::{fs::File, io::Write};
use clap::{App, Arg};
use std::error::Error;
use lab10::{RangeConfig, CirclePara, Point, Locate, gen_point_list, locate_n_point};
use svg::Document;
use svg::node::element::Circle;
use std::io::Result;

fn create_svg(c: &CirclePara, points: &[Point], locations: &[Locate]) -> Document {
    let mut doc = Document::new().set("viewBox", (0, 0, 500, 500));

    // Draw the circle
    let circle_element = Circle::new()
        .set("cx", c.center_x)
        .set("cy", c.center_y)
        .set("r", c.r)
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 2);
    doc = doc.add(circle_element);

    // Draw points and color them based on their location
    for (point, location) in points.iter().zip(locations.iter()) {
        let color = match location {
            Locate::Inside(_) => "green",
            Locate::Outside(_) => "red",
        };
        let point_element = Circle::new()
            .set("cx", point.x)
            .set("cy", point.y)
            .set("r", 3.0) // Adjust the radius as needed
            .set("fill", color);
        doc = doc.add(point_element);
    }

    doc
}

fn main() -> Result<()> {
    let matches = App::new("Point Location SVG Generator")
        .version("1.0")
        .author("Your Name")
        .arg(Arg::with_name("x")
             .short("x")
             .takes_value(true)
             .required(true)
             .help("X coordinate of the circle center"))
        .arg(Arg::with_name("y")
             .short("y")
             .takes_value(true)
             .required(true)
             .help("Y coordinate of the circle center"))
        .arg(Arg::with_name("r")
             .short("r")
             .takes_value(true)
             .required(true)
             .help("Radius of the circle"))
        .arg(Arg::with_name("n")
             .short("n")
             .takes_value(true)
             .required(true)
             .help("Number of points to generate"))
        .arg(Arg::with_name("x_min")
             .long("x_min")
             .takes_value(true)
             .required(true)
             .help("Minimum x value for point generation"))
        .arg(Arg::with_name("x_max")
             .long("x_max")
             .takes_value(true)
             .required(true)
             .help("Maximum x value for point generation"))
        .arg(Arg::with_name("y_min")
             .long("y_min")
             .takes_value(true)
             .required(true)
             .help("Minimum y value for point generation"))
        .arg(Arg::with_name("y_max")
             .long("y_max")
             .takes_value(true)
             .required(true)
             .help("Maximum y value for point generation"))
        .get_matches();

    let center_x = matches.value_of("x").unwrap().parse::<f64>().expect("Invalid x coordinate");
    let center_y = matches.value_of("y").unwrap().parse::<f64>().expect("Invalid y coordinate");
    let r = matches.value_of("r").unwrap().parse::<f64>().expect("Invalid radius");
    let num_points = matches.value_of("n").unwrap().parse::<i64>().expect("Invalid number of points");

    let circle = CirclePara {
        center_x,
        center_y,
        r,
    };

    let range_config = RangeConfig {
        x_min: matches.value_of("x_min").unwrap().parse::<f64>().expect("Invalid x_min"),
        x_max: matches.value_of("x_max").unwrap().parse::<f64>().expect("Invalid x_max"),
        y_min: matches.value_of("y_min").unwrap().parse::<f64>().expect("Invalid y_min"),
        y_max: matches.value_of("y_max").unwrap().parse::<f64>().expect("Invalid y_max"),
    };

    let mut rng = rand::thread_rng();
    let points = gen_point_list(&mut rng, &range_config, num_points);
    let locations = locate_n_point(&circle, &points);

    let svg_doc = create_svg(&circle, &points, &locations);

    let file_name = "output.svg";
    let mut file = File::create(file_name)?;
    svg::write(&mut file, &svg_doc)?;

    println!("SVG image saved as '{}'", file_name);

    Ok(())
}
