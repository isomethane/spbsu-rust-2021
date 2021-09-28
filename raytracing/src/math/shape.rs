use crate::math::ray::Ray;
use crate::math::threshold;
use crate::math::vec3d::Vec3D;
use num_traits::real::Real;

pub trait Intersect<T: Real> {
    fn intersect(&self, ray: Ray<T>) -> Option<T>;
    fn normal(&self, point: Vec3D<T>) -> Vec3D<T>;
}

pub enum Shape<T: Real> {
    Sphere(Sphere<T>),
    Plane(Plane<T>),
}

impl<T: Real> Intersect<T> for Shape<T> {
    fn intersect(&self, ray: Ray<T>) -> Option<T> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),
        }
    }

    fn normal(&self, point: Vec3D<T>) -> Vec3D<T> {
        match self {
            Shape::Sphere(sphere) => sphere.normal(point),
            Shape::Plane(plane) => plane.normal(point),
        }
    }
}

pub struct Sphere<T: Real> {
    pub center: Vec3D<T>,
    pub radius: T,
}

impl<T: Real> Intersect<T> for Sphere<T> {
    fn intersect(&self, ray: Ray<T>) -> Option<T> {
        let center_direction = self.center - ray.origin;
        let center_projection = center_direction * ray.direction;
        let distance_square =
            center_direction * center_direction - center_projection * center_projection;
        if distance_square > self.radius * self.radius {
            return None;
        }
        let intersection_distance = (self.radius * self.radius - distance_square).sqrt();
        let t0 = center_projection - intersection_distance;
        let t1 = center_projection + intersection_distance;
        if t1 < threshold() {
            None
        } else {
            Some(if t0 < threshold() { t1 } else { t0 })
        }
    }

    fn normal(&self, point: Vec3D<T>) -> Vec3D<T> {
        (point - self.center).normalize()
    }
}

pub struct Plane<T: Real> {
    pub origin: Vec3D<T>,
    pub normal: Vec3D<T>,
}

impl<T: Real> Intersect<T> for Plane<T> {
    fn intersect(&self, ray: Ray<T>) -> Option<T> {
        if (ray.direction * self.normal).abs() < threshold() {
            None
        } else {
            let t = (self.origin - ray.origin) * self.normal / (ray.direction * self.normal);
            if t < threshold() {
                None
            } else {
                Some(t)
            }
        }
    }

    fn normal(&self, _point: Vec3D<T>) -> Vec3D<T> {
        self.normal
    }
}
