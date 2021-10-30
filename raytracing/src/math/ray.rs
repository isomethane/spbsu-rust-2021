use crate::math::vec3d::Vec3D;
use num_traits::real::Real;

#[derive(Copy, Clone)]
pub struct Ray<T: Real> {
    pub origin: Vec3D<T>,
    pub direction: Vec3D<T>,
}

impl<T: Real> Ray<T> {
    pub fn point_at(&self, distance: T) -> Vec3D<T> {
        self.origin + self.direction * distance
    }
}
