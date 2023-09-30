use rand::Rng;
use std::fs::File;
use csv::{WriterBuilder, ReaderBuilder, Trim};
use std::io::Write;
use std::error::Error;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

pub struct Layer {
    pub name: String,
    pub color: String,
    pub objects: Vec<Circle>,
}

pub fn random_color<R: Rng>(rng: &mut R) -> String {
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    let a = rng.gen_range(0..=255);

    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

pub fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: i64) -> Vec<Layer> {
    let mut result = Vec::new();

    for i in 1..=n {
        let color = random_color(rng);

        let mut cir = Vec::new();
        let num_circle = rng.gen_range(20..=50);

        for _ in 0..num_circle {
            let x: f64 = rng.gen_range(-100.0..100.0);
            let y: f64 = rng.gen_range(-100.0..100.0);
            let r: f64 = rng.gen_range(-10.0..20.0);

            let circle = Circle {x, y , r};
            cir.push(circle)
        }
    
        let layer = Layer {
            name:format!("Layer {}", i),
            color,
            objects: cir,
        };

        result.push(layer);

    }
        
    result

}

#[test]
fn test_obj_layer_list() {
    let mut rng = rand::thread_rng();
    assert_eq!(5, gen_obj_layer_list(&mut rng, 5).len());
    for i in gen_obj_layer_list(&mut rng, 5) {
        assert!(i.objects.len() >= 20 && i.objects.len() <= 50);
        for j in &i.objects {
            let x = j.x;
            let y = j.y;
            let r = j.r;
            assert!(x >= -100. && x <= 100. && y >= -100. && y <= 100.);
            assert!(r >= -10. && r<= 20.);
        }
    }
}

pub fn cal_average_area(layer: &Vec<Layer>) -> Vec<(String, f64)> {
    let mut result = Vec::new();
    for i in layer {
        let mut sum = 0.;
        for j in &i.objects {
            sum += j.r * j.r * std::f64::consts::PI;
        }
        let average = sum / i.objects.len() as f64;
        result.push((i.name.clone(), average));
    }
    result
}

#[test]
fn test_cal_average_area() {
    let mut rng = rand::thread_rng();
    let layer = gen_obj_layer_list(&mut rng, 5);
    let result = cal_average_area(&layer);
    assert_eq!(5, result.len());
    for i in result {
        assert!(i.1 >= 0.);
    }
}


pub fn savefile<W:Write>(write:W, list: Vec<Layer>) -> Result<(), Box<dyn Error>>{
    let mut wtr = WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(write);
    for i in list {
        let mut res = Vec::new();
        for j in i.objects{
            res.push(j.x.to_string());
            res.push(j.y.to_string());
            res.push(j.r.to_string());
        }
        wtr.write_record([i.name, i.color, res.join(",")]).unwrap();
    }
    wtr.flush()?;
    Ok(())
}

pub fn loadfile(filename: &str) -> Result<Vec<Layer>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut read = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(file);
    let mut readresult = Vec::new();
    for i in read.records() {
        if let Ok(r) = i {
            let name = r[0].to_string();
            let color = r[1].to_string();
            let mut epoint = Vec::new();

            let cir: Vec<f64> = r[2]
                .split(',')
                .map(|x| x.parse::<f64>().unwrap_or(0.0))
                .collect();
            for point in cir.chunks(3) {
                let x = point[0];
                let y = point[1];
                let r = point[2];
                let hi = Circle { x, y, r };
                epoint.push(hi);
            }

            readresult.push(Layer {
                name: name,
                color: color,
                objects: epoint,
            });
        }
    }
    Ok(readresult)
}

pub fn save_average_areas<W: Write>(write: W, average_areas: Vec<(String, f64)>,) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(write);

    for (name, average) in average_areas {
        wtr.write_record(&[name, average.to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn cal_average_area_min_max(layer: &Vec<Layer>) -> Vec<(String, f64, f64, f64)> {
    let mut result = Vec::new();
    for i in layer{
        let mut avg = 0.0;
        let mut check = Vec::new();
        for j in &i.objects{
            let sum = (j.r as f64).powf(2.0) * std::f64::consts::PI;
            check.push(sum);
            avg += sum
        }
        avg /= i.objects.len() as f64;
        let max = check.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min = check.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        result.push((i.name.clone(), avg, *max, *min));
    }
    result
}

