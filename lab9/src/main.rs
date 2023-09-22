use lab9::gen_layer_list;

// testing some functions
fn main() {
    // Initialize a random number generator
    let mut rng = rand::thread_rng();
    
    // Generate a list of layers (change the number as needed)
    let num_layers = 5; // You can customize the number of layers
    let layers = gen_layer_list(&mut rng, num_layers);

    // Print the generated layers
    for (i, layer) in layers.iter().enumerate() {
        println!("Layer {} - Name: {}, Color: {}, Points: {}", i + 1, layer.name, layer.color, layer.points.len());
    }
}
