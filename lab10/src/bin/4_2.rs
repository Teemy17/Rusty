use rand::Rng;
use std::{fs::File, io::Write};
use clap::{App, Arg};
use std::error::Error;
use lab10::{RangeConfig, CirclePara, CirclePara2, Point, Locate2, Bound, gen_point_list, locate_n_point2};
use svg::Document;
use svg::node::element::Circle;
use std::io::Result;

fn create_svg(circles: &Bound, points: &[Point], locations: &[Locate2]) -> Document {
    let mut doc = Document::new().set("viewBox", (0, 0, 500, 500));

    // Draw the first circle
    let circle_element1 = Circle::new()
        .set("cx", circles.circle1.center_x)
        .set("cy", circles.circle1.center_y)
        .set("r", circles.circle1.r)
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 2);
    doc = doc.add(circle_element1);

    // Draw the second circle
    let circle_element2 = Circle::new()
        .set("cx", circles.circle2.center_x2)
        .set("cy", circles.circle2.center_y2)
        .set("r", circles.circle2.r2)
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 2);
    doc = doc.add(circle_element2);

    // Draw points and color them based on their location
    for (point, location) in points.iter().zip(locations.iter()) {
        let color = match location {
            Locate2::InsideBoth => "green",
            Locate2::OnFirstCircle => "blue",
            Locate2::OnSecondCircle => "red",
            Locate2::OutsideBoth => "black",
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
        .arg(Arg::with_name("x1")
             .short("x1")
             .takes_value(true)
             .required(true)
             .help("X coordinate of the first circle center"))
        .arg(Arg::with_name("y1")
             .short("y1")
             .takes_value(true)
             .required(true)
             .help("Y coordinate of the first circle center"))
        .arg(Arg::with_name("r1")
             .short("r1")
             .takes_value(true)
             .required(true)
             .help("Radius of the first circle"))
        .arg(Arg::with_name("x2")
             .short("-ax2")
             .takes_value(true)
             .required(true)
             .help("X coordinate of the second circle center"))
        .arg(Arg::with_name("y2")
             .short("-by2")
             .takes_value(true)
             .required(true)
             .help("Y coordinate of the second circle center"))
        .arg(Arg::with_name("r2")
             .short("-cr2")
             .takes_value(true)
             .required(true)
             .help("Radius of the second circle"))
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

    let center_x1 = matches.value_of("x1").unwrap().parse::<f64>().expect("Invalid x1 coordinate");
    let center_y1 = matches.value_of("y1").unwrap().parse::<f64>().expect("Invalid y1 coordinate");
    let r1 = matches.value_of("r1").unwrap().parse::<f64>().expect("Invalid radius1");
    
    let center_x2 = matches.value_of("x2").unwrap().parse::<f64>().expect("Invalid x2 coordinate");
    let center_y2 = matches.value_of("y2").unwrap().parse::<f64>().expect("Invalid y2 coordinate");
    let r2 = matches.value_of("r2").unwrap().parse::<f64>().expect("Invalid radius2");
    
    let num_points = matches.value_of("n").unwrap().parse::<i64>().expect("Invalid number of points");

    let circle1 = CirclePara {
        center_x: center_x1,
        center_y: center_y1,
        r: r1,
    };
    
    let circle2 = CirclePara2 {
        center_x2: center_x2,
        center_y2: center_y2,
        r2: r2,
    };

    let boundary = Bound {
        circle1,
        circle2,
    };

    let range_config = RangeConfig {
        x_min: matches.value_of("x_min").unwrap().parse::<f64>().expect("Invalid x_min"),
        x_max: matches.value_of("x_max").unwrap().parse::<f64>().expect("Invalid x_max"),
        y_min: matches.value_of("y_min").unwrap().parse::<f64>().expect("Invalid y_min"),
        y_max: matches.value_of("y_max").unwrap().parse::<f64>().expect("Invalid y_max"),
    };

    let mut rng = rand::thread_rng();
    let points = gen_point_list(&mut rng, &range_config, num_points);
    let locations = locate_n_point2(&boundary, &points);

    let svg_doc = create_svg(&boundary, &points, &locations);

    let file_name = "output2.svg";
    let mut file = File::create(file_name)?;
    svg::write(&mut file, &svg_doc)?;

    println!("SVG image saved as '{}'", file_name);

    Ok(())
}
