use rand::Rng;
use std::{fs::File, io::Write};
use clap::{App, Arg};
use std::error::Error;

//Arway and Spoon codes
pub struct Point {
    x: i64,
    y: i64,
}

pub struct Layer {
    name: String,
    color: String, // Use a string to store "#RRGGBBAA"
    points: Vec<Point>,
}

impl Layer {
    fn new(name: &str, color: &str) -> Self {
        Layer {
            name: name.to_string(),
            color: color.to_string(),
            points: Vec::new(),
        }
    }

    fn add_point(&mut self, x: i64, y: i64) {
        let point = Point { x, y };
        self.points.push(point);
    }
}

pub fn gen_layer<R: Rng>(rng: &mut R, name: &str, color: &str) -> Layer {
    let mut layer = Layer::new(name, color);
    let random = rng.gen_range(1..=1000);

    for _ in 0..random {
        let x = rng.gen_range(-250..=250);
        let y = rng.gen_range(-250..=250);
        layer.add_point(x, y);
    }

    layer // Return the constructed layer
}

pub fn gen_layer_list<R: Rng>(rng: &mut R, n: usize) -> Vec<Layer> {
    let mut layer_list = Vec::new();
    for i in 0..n {
        let name = format!("Layer {}", i);
        let color = format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rng.gen::<u8>(),
            rng.gen::<u8>(),
            rng.gen::<u8>(),
            rng.gen::<u8>()
        );
        let layers = gen_layer(rng, &name, &color);
        layer_list.push(layers);
    }
    layer_list
}

fn generate_svg(layers: &[Layer]) -> String {
    let mut svg = String::from("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");

    for layer in layers {
        // Note: This line will draw a black circle with a radius of 250 at the center of the SVG.
        // You may want to modify it depending on your desired output.
        svg.push_str("<circle cx=\"250\" cy=\"250\" r=\"250\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n");

        for point in &layer.points {
            svg.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"10\" fill=\"{}\" />\n", point.x + 250, point.y + 250, layer.color));
        }
    }

    svg.push_str("</svg>");
    svg
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Create Layers and Colors")
        .version("1.0")
        .author("Save")
        .about("Creating layers and colors")
        .arg(
            Arg::with_name("layers")
                .short("n")
                .long("layers")
                .value_name("NUM_LAYERS")
                .help("Number of layers to generate")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output SVG file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let n = matches.value_of("layers").unwrap().parse::<usize>()?;
    let output_name = matches.value_of("output").unwrap();

    let mut rng = rand::thread_rng();

    let layer_list = gen_layer_list(&mut rng, n); // Generate the specified number of random layers

    let svg_content = generate_svg(&layer_list);
    let mut output_file = File::create(output_name)?;
    output_file.write_all(svg_content.as_bytes())?;

    Ok(())
}