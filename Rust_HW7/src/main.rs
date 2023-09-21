use Rust_HW7::to_polar;
use Rust_HW7::Point;

// just testing the functions
fn main() {
    let points = vec![
        Point { x: 3.0, y: 4.0 },
        Point { x: -1.0, y: 1.0 },
        Point { x: 0.0, y: 0.0 },
    ];

    let polar_points = to_polar(points);

    for polar_point in &polar_points {
        println!("Polar coordinates: (radius = {:.2}, angle = {:.2} radians)", polar_point.r, polar_point.t);
    }
}
