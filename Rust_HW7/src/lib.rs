pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct PolarPoint {
    pub r: f64,
    pub t: f64,
}


pub fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint> {
    let mut v = Vec::new();

    for point in pt_list {
        let r = ((point.x * point.x) + (point.y * point.y)).sqrt();
        let t = point.y.atan2(point.x);

        let polar_point = PolarPoint {
            r,
            t,
        };
        v.push(polar_point)
    }
    return v
}

pub fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut v = Vec::new();

    for point in pt_list {
        let x = point.r * point.t.cos();
        let y = point.r * point.t.sin();
    
        let cartesian_point = Point {
            x,
            y,
        };
        v.push(cartesian_point)
    }
    return v
}

