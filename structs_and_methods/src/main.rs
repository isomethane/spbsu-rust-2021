use std::fmt;
use std::fmt::Formatter;
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}; {})", self.x, self.y)
    }
}

struct Rect {
    lower_left: Point,
    width: f64,
    height: f64,
}
impl Rect {
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.lower_left.x && p.x <= self.lower_left.x + self.width &&
            p.y >= self.lower_left.y && p.y <= self.lower_left.y + self.height
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    center: Point,
    radius: f64,
}
impl Circle {
    fn contains(&self, p: &Point) -> bool {
        (p.x - self.center.x).powi(2) + (p.y - self.center.y).powi(2) <=
            self.radius.powi(2)
    }

    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
}

enum Figure {
    Rect(Rect),
    Circle(Circle),
}
impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Rect(rect) => { rect.contains(p) }
            Figure::Circle(circle) => { circle.contains(p) }
        }
    }

    fn area(&self) -> f64 {
        match self {
            Figure::Rect(rect) => { rect.area() }
            Figure::Circle(circle) => { circle.area() }
        }
    }
}

fn main() {
    let rect = Rect { lower_left: Point { x: 3.0, y: 5.0 }, width: 6.0, height: 4.0 };
    println!(
        "rect: lower_left {}, width {}, height {}",
        rect.lower_left, rect.width, rect.height
    );

    let circle = Circle { center: Point { x: 7.0, y: 4.0 }, radius: 3.0 };
    println!("circle: center {}, radius {}", circle.center, circle.radius);

    let a = Point { x: 6.0, y: 6.0 };
    let b = Point { x: 4.0, y: 3.0 };
    let c = Point { x: 4.0, y: 5.0 };
    println!("points: a {}, b {}, c {}", a, b, c);

    let figures: [(&str, &Figure); 2] = [
        ("rect", &Figure::Rect(rect)), ("circle", &Figure::Circle(circle))
    ];
    let points: [(&str, &Point); 3] = [("a", &a), ("b", &b), ("c", &c)];

    for (figure_name, figure) in figures {
        for (point_name, point) in points {
            println!("Does {} contain {}? {}", figure_name, point_name, figure.contains(point));
        }
    }
    for (figure_name, figure) in figures {
        println!("{} area: {}", figure_name, figure.area());
    }
}
