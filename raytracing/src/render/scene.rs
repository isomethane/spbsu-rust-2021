use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vec3d::Vec3D;
use crate::render::hittable::Hittable;
use crate::render::lightning::{phong_lightning, reflected_lightning, refracted_lightning, Light};
use crate::render::material::Material;
use num_traits::real::Real;

pub struct Scene<T: Real> {
    pub background_color: Color<T>,
    pub objects: Vec<Hittable<T>>,
    pub lights: Vec<Light<T>>,
    pub recursion_depth: i32,
}

#[derive(Copy, Clone)]
pub struct IntersectResult<T: Real> {
    pub point: Vec3D<T>,
    pub normal: Vec3D<T>,
    pub material: Material<T>,
}

impl<T: Real> Scene<T> {
    pub fn intersect(&self, ray: Ray<T>) -> Option<IntersectResult<T>> {
        let mut min_distance = T::max_value();
        let mut result: Option<IntersectResult<T>> = None;
        for object in &self.objects {
            if let Some(distance) = object.intersect(ray) {
                if min_distance > distance {
                    min_distance = distance;
                    result = Some(object.intersection_result(ray.point_at(distance)));
                }
            }
        }
        result
    }

    pub fn cast_ray(&self, ray: Ray<T>, depth: i32) -> Color<T> {
        if depth > self.recursion_depth {
            return self.background_color;
        }

        if let Some(result) = self.intersect(ray) {
            let phong = phong_lightning(self, ray, result);
            let reflected = reflected_lightning(self, ray, result, depth);
            let refracted = refracted_lightning(self, ray, result, depth);
            phong + reflected + refracted
        } else {
            self.background_color
        }
    }
}
