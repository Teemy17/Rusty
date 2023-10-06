use rand::Rng;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct CirclePara {
    pub center_x: f64,
    pub center_y: f64,
    pub r: f64,
}

pub struct CirclePara2 {
    pub center_x2: f64,
    pub center_y2: f64,
    pub r2: f64,
}

pub struct Bound {
    pub circle1: CirclePara,
    pub circle2: CirclePara2,
}

pub struct RangeConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug)]
pub enum Locate {
    Inside(f64), 
    Outside(f64), 
}

pub enum Locate2 {
    InsideBoth,
    OnFirstCircle,
    OnSecondCircle,
    OutsideBoth,
}

pub fn gen_point_list<R: Rng>(rng: &mut R, cfg: &RangeConfig, n: i64) -> Vec<Point> {
    let mut result = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_min..=cfg.x_max);
        let y = rng.gen_range(cfg.y_min..=cfg.y_max);
        result.push(Point { x, y });
    }
    result
}

pub fn locate_n_point(c: &CirclePara, pt_list: &[Point]) -> Vec<Locate> {
    let mut result = Vec::new();
    for pt in pt_list {
        let d = ((pt.x - c.center_x).powi(2) + (pt.y - c.center_y).powi(2)).sqrt();
        if d < c.r {
            result.push(Locate::Inside(d));
        } else {
            result.push(Locate::Outside(d));
        }
    }
    result
}

pub fn locate_n_point2(b: &Bound, pt_list: &[Point]) -> Vec<Locate2> {
    let mut result = Vec::new();
    for pt in pt_list {
        let d1 = ((pt.x - b.circle1.center_x).powi(2) + (pt.y - b.circle1.center_y).powi(2)).sqrt();
        let d2 = ((pt.x - b.circle2.center_x2).powi(2) + (pt.y - b.circle2.center_y2).powi(2)).sqrt();
        if d1 < b.circle1.r && d2 < b.circle2.r2 {
            result.push(Locate2::InsideBoth);
        } else if d1 < b.circle1.r {
            result.push(Locate2::OnFirstCircle);
        } else if d2 < b.circle2.r2 {
            result.push(Locate2::OnSecondCircle);
        } else {
            result.push(Locate2::OutsideBoth);
        }
    }    
    result
}









