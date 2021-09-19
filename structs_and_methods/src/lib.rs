pub mod figures {
    use num_traits::Num;
    use std::fmt::{Debug, Formatter};

    #[derive(Eq, PartialEq, Hash)]
    pub struct Point<T: Num> {
        pub x: T,
        pub y: T,
    }
    impl<T: Num + Debug> Debug for Point<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({:?}; {:?})", self.x, self.y)
        }
    }
    impl<T: Num> Default for Point<T> {
        fn default() -> Self {
            Point { x: T::zero(), y: T::zero() }
        }
    }

    #[derive(Eq, PartialEq, Hash)]
    pub struct Rect<T: Num> {
        pub lower_left: Point<T>,
        pub width: T,
        pub height: T,
    }
    impl<T: Num + Debug> Debug for Rect<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[lower_left: {:?}, width: {:?}, height: {:?}]",
                   self.lower_left, self.width, self.height)
        }
    }
    impl<T: Num> Default for Rect<T> {
        fn default() -> Self {
            Rect { lower_left: Point::default(), width: T::one(), height: T::one() }
        }
    }

    #[derive(Eq, PartialEq, Hash)]
    pub struct Circle<T: Num> {
        pub center: Point<T>,
        pub radius: T,
    }
    impl<T: Num + Debug> Debug for Circle<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[center: {:?}, radius: {:?}]",
                   self.center, self.radius)
        }
    }
    impl<T: Num> Default for Circle<T> {
        fn default() -> Self {
            Circle { center: Point::default(), radius: T::one() }
        }
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Figure<T: Num> {
        Rect(Rect<T>),
        Circle(Circle<T>),
    }
}

pub mod calc {
    use crate::figures::{Point, Rect, Circle, Figure};
    use num_traits::Num;
    use std::f64::consts::PI;

    impl<T: Num + Copy + PartialOrd> Rect<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            p.x >= self.lower_left.x && p.x <= self.lower_left.x + self.width &&
                p.y >= self.lower_left.y && p.y <= self.lower_left.y + self.height
        }
    }
    impl<T: Num + Copy + Into<f64>> Rect<T> {
        pub fn area(&self) -> f64 {
            (self.width * self.height).into()
        }
    }

    impl<T: Num + Copy + PartialOrd> Circle<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            (p.x - self.center.x) * (p.x - self.center.x) +
                (p.y - self.center.y) * (p.y - self.center.y) <=
                self.radius * self.radius
        }
    }
    impl<T: Num + Copy + Into<f64>> Circle<T> {
        pub fn area(&self) -> f64 {
            PI * self.radius.into().powi(2)
        }
    }

    impl<T: Num + Copy + PartialOrd> Figure<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            match self {
                Figure::Rect(rect) => { rect.contains(p) }
                Figure::Circle(circle) => { circle.contains(p) }
            }
        }
    }
    impl<T: Num + Copy + Into<f64>> Figure<T> {
        pub fn area(&self) -> f64 {
            match self {
                Figure::Rect(rect) => { rect.area() }
                Figure::Circle(circle) => { circle.area() }
            }
        }
    }
}
