extern crate num_traits;
extern crate noisy_float;

use structs_and_methods::figures::{Point, Rect, Circle, Figure};

use noisy_float::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn main() {
    let default_point: Point<i8> = Point::default();
    println!("Default i8 point: {:?}", default_point);
    let default_rect: Rect<R64> = Rect::default();
    println!("Default R64 rect: {:?}", default_rect);
    let default_circle: Circle<usize> = Circle::default();
    println!("Default usize circle: {:?}", default_circle);

    println!("Are default points equal?: {}", default_point == Point::default());
    println!("Are default rectangles equal?: {}", default_rect == Rect::default());
    println!("Are default circles equal?: {}", default_circle == Circle::default());

    {
        let mut hasher = DefaultHasher::new();
        default_point.hash(&mut hasher);
        println!("Default point hash: {:?}", hasher.finish());
    }
    {
        let mut hasher = DefaultHasher::new();
        default_rect.hash(&mut hasher);
        println!("Default rect hash: {:?}", hasher.finish());
    }
    {
        let mut hasher = DefaultHasher::new();
        default_circle.hash(&mut hasher);
        println!("Default circle hash: {:?}", hasher.finish());
    }

    let rect = Rect { lower_left: Point { x: 3.0, y: 5.0 }, width: 6.0, height: 4.0 };
    println!("rect: {:?}", rect);

    let circle = Circle { center: Point { x: 7.0, y: 4.0 }, radius: 3.0 };
    println!("circle: {:?}", circle);

    let a = Point { x: 6.0, y: 6.0 };
    let b = Point { x: 4.0, y: 3.0 };
    let c = Point { x: 4.0, y: 5.0 };
    println!("points: a {:?}, b {:?}, c {:?}", a, b, c);

    let figures: [(&str, Figure<f64>); 2] = [
        ("rect", Figure::Rect(rect)), ("circle", Figure::Circle(circle))
    ];
    let points: [(&str, Point<f64>); 3] = [("a", a), ("b", b), ("c", c)];

    for (figure_name, figure) in &figures {
        for (point_name, point) in &points {
            println!("Does {} contain {}? {}", figure_name, point_name, figure.contains(point));
        }
    }
    for (figure_name, figure) in &figures {
        println!("{} area: {}", figure_name, figure.area());
    }
}
