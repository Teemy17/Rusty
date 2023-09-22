use std::fs::File;
use clap::{App, Arg};
use lab9::save_points;
use lab9::gen_layer_list;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Create layers and colors")
        .version("1.0")
        .author("bruh")
        .about("create colors and layers")
        .arg(
            Arg::with_name("Layers")
                .short("l")
                .long("Layers")
                .value_name("n")
                .help("Define how many layers the output should have")
                .required(true)
                .takes_value(true),
            )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output CSV file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
        let n_arg = matches.value_of("Layers").unwrap();
        let n: i64 = n_arg.parse()?;
        let output_name = matches.value_of("output").unwrap();

        let mut rng = rand::thread_rng();
        let layer_list = gen_layer_list(&mut rng, n);
        save_points(File::create(output_name)?, layer_list)?;

    Ok(())   
}