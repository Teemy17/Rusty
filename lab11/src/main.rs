use lab11::encode_hex_data;

fn main() {
    // Example data to encode
    let data_to_encode: [u8; 4] = [0x12, 0x21, 0x54, 0x68];

    // Encode the data
    let encoded_data = encode_hex_data(&data_to_encode);

    // Print the encoded data
    for byte in &encoded_data {
        print!("{:02x} ", byte);
    }
}

