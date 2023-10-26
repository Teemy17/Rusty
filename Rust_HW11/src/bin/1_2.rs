trait Shape2 {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_shape(&self) -> Box<dyn Shape2>;
}


struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

struct Triangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
}

impl Clone for Box<dyn Shape2> {
    fn clone(&self) -> Box<dyn Shape2> {
        self.clone_shape()
    }
}

impl Shape2 for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r as f64) * (self.r as f64)
    }

    fn clone_shape(&self) -> Box<dyn Shape2> {
        Box::new(Circle { x: self.x, y: self.y, r: self.r })
    }
}

impl Shape2 for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
    }

    fn area(&self) -> f64 {
        (self.w as f64) * (self.h as f64)
    }

    fn clone_shape(&self) -> Box<dyn Shape2> {
        Box::new(Rectangle { x: self.x, y: self.y, w: self.w, h: self.h })
    }
}

impl Shape2 for Triangle {
    fn rep_string(&self) -> String {
        format!("<Triangle: ({}, {}), ({}, {}), ({}, {})>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
    }

    fn area(&self) -> f64 {
        let a = 0.5 * ((self.x1 - self.x3)*(self.y2 - self.y1) - (self.x1 - self.x2)*(self.y3 - self.y1)) as f64;
        a
    }

    fn clone_shape(&self) -> Box<dyn Shape2> {
        Box::new(Triangle { x1: self.x1, y1: self.y1, x2: self.x2, y2: self.y2, x3: self.x3, y3: self.y3 })
    }
}

impl Circle {
    fn new(x: i32, y: i32, r: i32) -> Box<dyn Shape2> {
        Box::new(Circle { x, y, r })
    }
}

impl Rectangle {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Box<dyn Shape2> {
        Box::new(Rectangle { x, y, w, h })
    }
}

impl Triangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Box<dyn Shape2> {
        Box::new(Triangle { x1, y1, x2, y2, x3, y3 })
    }
}

fn input_shape_list() -> Vec<Box<dyn Shape2>> {
    vec![
        Circle::new(0, 0, 1),
        Circle::new(50, 50, 15),
        Rectangle::new(40, 40, 20, 20),
        Rectangle::new(10, 40, 15, 10),
        Triangle::new(10, 20, 30, 40, 32, 47),
        Triangle::new(30, 32, 48, 32, 34, 22),
    ]
}
const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Rectangle: 10, 40, 15, 10>",
    "<Triangle: (10, 20), (30, 40), (32, 47)>",
    "<Triangle: (30, 32), (48, 32), (34, 22)>"
];
const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: (10, 20), (30, 40), (32, 47)>, area: 50.00",
    "<Triangle: (30, 32), (48, 32), (34, 22)>, area: -90.00",
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}