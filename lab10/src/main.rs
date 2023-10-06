use lab10::{RangeConfig, Circle, Point, gen_point_list, locate_n_point};


fn main() {
    let mut rng = rand::thread_rng();
    let cfg = RangeConfig {
        x_min: -1.5,
        x_max: 1.5,
        y_min: -1.5,
        y_max: 1.5,
    };
    let c = Circle { center: Point { x: -0.1, y: -0.1 }, r: 0.8 };
    let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, 50);
    let loc_list: Vec<lab10::Locate> = locate_n_point(&c, &pt_list);
    for loc in loc_list {
        println!("{:?}", loc);
    }
    // let (w, h) = (600, 600); // SVG image size
    // let scale = (h as f64) / (cfg.y_max - cfg.y_min);
    // let pt_map = |pt: &Point| {
    // let x = (pt.x - cfg.x_min) * scale;
    // let y = (-pt.y - cfg.y_min) * scale;
    // (x as i64, y as i64) // map (x, y) to SVG output
    // };
}
