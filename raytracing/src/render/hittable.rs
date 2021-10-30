use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::shape::{Intersect, Plane, Shape};
use crate::math::vec3d::Vec3D;
use crate::render::material::Material;
use crate::render::scene::IntersectResult;
use num_traits::real::Real;

pub enum Hittable<T: Real> {
    Primitive {
        shape: Shape<T>,
        material: Material<T>,
    },
    CheckerBoard {
        plane: Plane<T>,
        width: T,
        basic_material: Material<T>,
        checker_color: Color<T>,
        checker_size: T,
    },
}

impl<T: Real> Hittable<T> {
    pub fn intersect(&self, ray: Ray<T>) -> Option<T> {
        match self {
            Hittable::Primitive { shape, .. } => shape.intersect(ray),
            Hittable::CheckerBoard { plane, width, .. } => {
                if let Some(t) = plane.intersect(ray) {
                    let point = ray.point_at(t);
                    if point.x < plane.origin.x
                        || point.z < plane.origin.z
                        || point.x - plane.origin.x > *width
                        || point.z - plane.origin.z > *width
                    {
                        None
                    } else {
                        Some(t)
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn intersection_result(&self, point: Vec3D<T>) -> IntersectResult<T> {
        match self {
            Hittable::Primitive { shape, material } => IntersectResult {
                point,
                normal: shape.normal(point),
                material: *material,
            },
            Hittable::CheckerBoard {
                plane,
                basic_material,
                checker_color,
                checker_size,
                ..
            } => {
                let mut material = *basic_material;
                if ((point.x / *checker_size).floor().to_i32().unwrap()
                    + (point.z / *checker_size).floor().to_i32().unwrap())
                    % 2
                    == 0
                {
                    material.diffuse = *checker_color;
                }
                IntersectResult {
                    point,
                    normal: plane.normal,
                    material,
                }
            }
        }
    }
}
