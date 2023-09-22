use rand::Rng;
use csv::Writer;
use std::fs::File;
use clap::{App, Arg};
use std::error::Error;

pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub struct Layer {
    pub name: String,
    pub color: String,
    pub points: Vec<Point>,
}

pub fn gen_layer<R: Rng>(name: String, color: String, rng: &mut R) -> Layer {
    let mut result = Vec::new();
    let n = rng.gen_range(20..=50);
    
    for _ in 0..n {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);
        let point = Point {x , y};
        result.push(point);
    }
   
    Layer { name, color, points: result,}
}


pub fn random_color<R: Rng>(rng: &mut R) -> String {
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    let a = rng.gen_range(0..=255);

    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

pub fn gen_points<R: Rng>(rng: &mut R) -> Vec<Point> {
    let mut result = Vec::new();
    let num_points = rng.gen_range(20..=50);

    for _ in 0..num_points {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);
        result.push(Point { x, y });
    }

    result
}

pub fn gen_layer_list<R: Rng>(rng: &mut R, n: i64) -> Vec<Layer> {
    let mut result = Vec::new();
    
    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let color = random_color(rng);
        let points = gen_points(rng);
        result.push(Layer {
            name,
            color,
            points,
        });
    }
    result
}

pub fn save_points<W: std::io::Write>(writer: W, layers: Vec<Layer>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(writer);

    for layer in layers {
        let name = layer.name;
        let color = layer.color;
        let points: Vec<String> = layer.points.iter()
            .map(|point| format!("{},{},", point.x, point.y))
            .collect();

        wtr.write_record(&[name, color, points.join(",")])?;
    }

    wtr.flush()?;
    Ok(())
}



#[test]
fn test_gen_layer() {
    let mut rng = rand::thread_rng();
    let name = String::from("Test Layer");
    let color = String::from("Red");
    let layer = gen_layer(name.clone(), color.clone(), &mut rng);

    assert_eq!(layer.name, name);
    assert_eq!(layer.color, color);

    let num_points = layer.points.len();
    assert!(num_points >= 20 && num_points <= 50);

    for point in layer.points {
        assert!(point.x >= -100 && point.x <= 100);
        assert!(point.y >= -100 && point.y <= 100);
    }
}

#[test]
fn test_gen_layer_list(){
    let mut rng = rand::thread_rng();
    let gen_layer_list = gen_layer_list(&mut rng, 2);
    assert_eq!(gen_layer_list.len(), 2);
}



