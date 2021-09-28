use num_traits::real::Real;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3D<T: Real> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Real> Default for Vec3D<T> {
    fn default() -> Self {
        Vec3D {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T: Real> Vec3D<T> {
    pub fn norm(self) -> T {
        (self * self).sqrt()
    }

    pub fn normalize(&self) -> Vec3D<T> {
        let norm = self.norm();
        Vec3D {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

impl<T: Real> ops::Add<Self> for Vec3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Real> ops::Sub<Self> for Vec3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Real> ops::Mul<Self> for Vec3D<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Real> ops::Mul<T> for Vec3D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Real> ops::Neg for Vec3D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
