use std::fs::File;
use Rust_HW8::gen_obj_layer_list;
use Rust_HW8::savefile;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let num_layers = 5; 

    let layers = gen_obj_layer_list(&mut rng, num_layers);

    let file = File::create("layers2_1.csv")?;
    savefile(file, layers)?;

    println!("Layers with circle data have been saved to layers2_1.csv");

    Ok(())
}

