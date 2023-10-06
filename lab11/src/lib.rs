use std::collections::HashMap;
use std::io::{ Read, Result };
use std::str::FromStr;

fn encode_hex(b: u8) -> u8 {
    // Define the mapping table as an array
    let mapping_table: [u8; 16] = [
        0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4, 0xc, 0xd, 0xf, 0xe, 0xa, 0xb, 0x9, 0x8,
    ];

    // Extract the high and low nibbles
    // & is bitwise AND
    // >> is right shift
    // << is left shift
    let high_nibble = (b >> 4) & 0xf;
    let low_nibble = b & 0xf;

    // Look up the values in the mapping table
    let encoded_high_nibble = mapping_table[high_nibble as usize];
    let encoded_low_nibble = mapping_table[low_nibble as usize];

    // Combine and return the result
    (encoded_high_nibble << 4) | encoded_low_nibble
}

pub fn encode_hex_data(v: &[u8]) -> Vec<u8> {
    // Initialize a Vec<u8> to store the encoded result
    let mut encoded_result = Vec::new();

    // Iterate through each byte in the array and encode it
    for &byte in v {
        let encoded_byte = encode_hex(byte);
        encoded_result.push(encoded_byte);
    }

    encoded_result
}

fn decode_hex(b: u8) -> u8 {
    // Define the mapping table as an array
    let mapping_table: [u8; 16] = [
        0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xf, 0xe, 0xc, 0xd, 0x8, 0x9, 0xb, 0xa,
    ];

    // Extract the high and low nibbles
    let high_nibble = (b >> 4) & 0xf;
    let low_nibble = b & 0xf;

    // Look up the values in the mapping table
    let decoded_high_nibble = mapping_table[high_nibble as usize];
    let decoded_low_nibble = mapping_table[low_nibble as usize];

    // Combine and return the result
    (decoded_high_nibble << 4) | decoded_low_nibble
}

fn decode_hex_data(v: &[u8]) -> Vec<u8> {
    // Initialize a Vec<u8> to store the decoded result
    let mut decoded_result = Vec::new();

    // Iterate through each byte in the array and decode it
    for &byte in v {
        let decoded_byte = decode_hex(byte);
        decoded_result.push(decoded_byte);
    }

    decoded_result
}

const FOX: &str = "The quick brown fox jumps over the lazy dog.";
const ENCODED_DATA: &[u8] = &[
    0x76, 0x5c, 0x57, 0x30, 0x41, 0x47, 0x5d, 0x52, 0x5e, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59, 0x30,
    0x55, 0x58, 0x4c, 0x30, 0x5f, 0x47, 0x5b, 0x40, 0x42, 0x30, 0x58, 0x45, 0x57, 0x43, 0x30, 0x46,
    0x5c, 0x57, 0x30, 0x5a, 0x51, 0x4f, 0x4d, 0x30, 0x56, 0x58, 0x54, 0x39,
];

#[test]
fn test_encode_hex() {
    assert_eq!(
        (0..16).map(encode_hex).collect::<Vec<_>>(),
        [0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4, 0xc, 0xd, 0xf, 0xe, 0xa, 0xb, 0x9, 0x8]
    );
    assert_eq!(encode_hex(0x54), 0x76);
    assert_eq!(encode_hex(0x68), 0x5c);
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(encode_hex_data(original_data), encoded_data);
}

#[test]
fn test_decode_hex() {
    assert_eq!(
        (0..16).map(decode_hex).collect::<Vec<_>>(),
        [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xf, 0xe, 0xc, 0xd, 0x8, 0x9, 0xb, 0xa]
    );
    assert_eq!(decode_hex(0x76), 0x54);
    assert_eq!(decode_hex(0x5c), 0x68);
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(decode_hex_data(encoded_data), original_data);
}

#[derive(Debug, PartialEq)]
struct XpmImage {
    width: usize,
    height: usize,
    colors: Vec<(String, String)>,
    pixels: Vec<Vec<String>>,
}

impl XpmImage {
    fn new(width: usize, height: usize) -> Self {
        XpmImage {
            width,
            height,
            colors: Vec::new(),
            pixels: Vec::new(),
        }
    }
}

fn parse_xpm2(reader: &mut dyn Read) -> Result<XpmImage> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let header = lines
        .next()
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid XPM2 header"))?;

    if header != "! XPM2" {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid XPM2 header"));
    }

    let dimensions: Vec<usize> = lines
        .next()
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid XPM2 dimensions"))?
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    if dimensions.len() != 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid XPM2 dimensions"));
    }

    let width = dimensions[0];
    let height = dimensions[1];
    let color_count = dimensions[2];
    let char_per_pixel = dimensions[3];

    let mut colors = Vec::new();

    for _ in 0..color_count {
        let line = lines
            .next()
            .ok_or(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid XPM2 color definition"
                )
            )?;

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid XPM2 color definition"
                )
            );
        }

        colors.push((parts[0].into(), parts[1].into()));
    }

    let mut pixels = Vec::new();

    for line in lines {
        let pixel_row: Vec<String> = line
            .chars()
            .map(|c| c.to_string())
            .collect();
        if pixel_row.len() != width * char_per_pixel {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid XPM2 pixel row")
            );
        }
        pixels.push(pixel_row);
    }

    Ok(XpmImage {
        width,
        height,
        colors,
        pixels,
    })
}

fn read_xpm2(reader: &mut dyn Read) -> Result<XpmImage> {
    parse_xpm2(reader)
}

#[test]
fn test_read_xpm2() {
    let checker = "\
            ! XPM2\n\
            4 4 2 1\n\
            # c #000000\n\
            - c #FFFFFF\n\
            ##--\n\
            ##--\n\
            --##\n\
            --##\n\
            ";
    let data = checker.as_bytes().to_vec();
    let mut reader = data.as_slice();
    let img = read_xpm2(&mut reader).unwrap();
    assert_eq!(img.colors, [
        ("#".into(), "c #000000".into()),
        ("-".into(), "c #FFFFFF".into()),
    ]);
    assert_eq!(img.pixels[0].len(), 4);
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.colors.len(), 2);
    assert_eq!(
        img.pixels
            .iter()
            .map(|r| r.len())
            .max(),
        Some(4)
    );
}
