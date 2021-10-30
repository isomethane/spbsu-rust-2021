use image::Rgb;
use num_traits::real::Real;
use num_traits::{NumCast, ToPrimitive};
use std::ops;

#[derive(Copy, Clone)]
pub struct Color<T: Real> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Real> Color<T> {
    pub fn new(r: u8, g: u8, b: u8) -> Color<T> {
        Color {
            r: T::from(r as f64 / 255.0).unwrap(),
            g: T::from(g as f64 / 255.0).unwrap(),
            b: T::from(b as f64 / 255.0).unwrap(),
        }
    }

    pub fn zero() -> Color<T> {
        Color {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
        }
    }

    pub fn unit() -> Color<T> {
        Color {
            r: T::one(),
            g: T::one(),
            b: T::one(),
        }
    }
}

impl<T: Real> Default for Color<T> {
    fn default() -> Self {
        Color::zero()
    }
}

impl<T: Real> From<Color<T>> for Rgb<u8> {
    fn from(color: Color<T>) -> Self {
        let max_value = T::from(255).unwrap();
        Rgb([
            (color.r.min(T::one()) * max_value).to_u8().unwrap(),
            (color.g.min(T::one()) * max_value).to_u8().unwrap(),
            (color.b.min(T::one()) * max_value).to_u8().unwrap(),
        ])
    }
}

impl<T: Real> ops::Add<Self> for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl<T: Real> ops::Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<T: Real, R: ToPrimitive> ops::Div<R> for Color<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        let divisor = NumCast::from(rhs).unwrap();
        Color {
            r: self.r / divisor,
            g: self.g / divisor,
            b: self.b / divisor,
        }
    }
}
