extern crate rand;
extern crate clap;
extern crate svg;

use rand::Rng;
use clap::{App, Arg};
use svg::Document;
use svg::node::element::Circle;
use std::fs::File;
use std::io::Result;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct CircleParams {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
}

pub struct RangeConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

// Enum to represent the location of a point relative to a circle
pub enum PointLocation {
    Inside(f64),  // Inside the circle with distance from the center
    Outside(f64), // Outside the circle with distance from the center
}

// Function to calculate the Euclidean distance between two points
fn euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

// Function to generate a list of random points
fn gen_point_list<R: Rng>(rng: &mut R, cfg: &RangeConfig, n: i64) -> Vec<Point> {
    let mut result = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_min..=cfg.x_max);
        let y = rng.gen_range(cfg.y_min..=cfg.y_max);
        result.push(Point { x, y });
    }
    result
}

// Function to locate points relative to a circle and calculate distances
fn locate_n_point(circle: &CircleParams, pt_list: &[Point]) -> Vec<PointLocation> {
    pt_list
        .iter()
        .map(|point| {
            let distance = euclidean_distance(&Point { x: circle.center_x, y: circle.center_y }, point);

            if distance <= circle.radius {
                PointLocation::Inside(distance)
            } else {
                PointLocation::Outside(distance)
            }
        })
        .collect()
}

// Function to create an SVG document with points and circles
fn create_svg(circle: &CircleParams, points: &[Point], locations: &[PointLocation]) -> Document {
    let mut doc = Document::new().set("viewBox", (0, 0, 100, 100));

    // Draw the circle
    let circle_element = Circle::new()
        .set("cx", circle.center_x)
        .set("cy", circle.center_y)
        .set("r", circle.radius)
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 2);
    doc = doc.add(circle_element);

    // Draw points and color them based on their location
    for (point, location) in points.iter().zip(locations.iter()) {
        let color = match location {
            PointLocation::Inside(_) => "green",
            PointLocation::Outside(_) => "red",
        };
        let point_element = Circle::new()
            .set("cx", point.x)
            .set("cy", point.y)
            .set("r", 1.0) // Adjust the radius as needed
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
        .get_matches();

    let center_x = matches.value_of("x").unwrap().parse::<f64>().expect("Invalid x coordinate");
    let center_y = matches.value_of("y").unwrap().parse::<f64>().expect("Invalid y coordinate");
    let radius = matches.value_of("r").unwrap().parse::<f64>().expect("Invalid radius");
    let num_points = matches.value_of("n").unwrap().parse::<i64>().expect("Invalid number of points");

    let circle = CircleParams {
        center_x,
        center_y,
        radius,
    };

    let range_config = RangeConfig {
        x_min: 0.0,
        x_max: 100.0, // Adjust these values as needed
        y_min: 0.0,
        y_max: 100.0, // Adjust these values as needed
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
