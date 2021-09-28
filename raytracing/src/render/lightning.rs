use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vec3d::Vec3D;
use crate::render::scene::{IntersectResult, Scene};
use num_traits::real::Real;

pub struct Light<T: Real> {
    pub position: Vec3D<T>,
    pub intensity: T,
}

pub fn phong_lightning<T: Real>(
    scene: &Scene<T>,
    ray: Ray<T>,
    intersect_result: IntersectResult<T>,
) -> Color<T> {
    let IntersectResult {
        point,
        normal,
        material,
    } = intersect_result;

    let mut diffuse_light = T::zero();
    let mut specular_light = T::zero();
    for light in &scene.lights {
        let light_direction = (light.position - point).normalize();
        let light_distance = (light.position - point).norm();

        if let Some(IntersectResult {
            point: shadow_point,
            ..
        }) = scene.intersect(Ray {
            origin: point,
            direction: light_direction,
        }) {
            if (shadow_point - point).norm() < light_distance {
                continue;
            }
        }

        diffuse_light = diffuse_light + light.intensity * (light_direction * normal).max(T::zero());

        specular_light = specular_light
            + (reflect(light_direction, normal) * ray.direction)
                .max(T::zero())
                .powf(material.shininess)
                * light.intensity;
    }
    material.ambient + material.diffuse * diffuse_light + material.specular * specular_light
}

pub fn reflected_lightning<T: Real>(
    scene: &Scene<T>,
    ray: Ray<T>,
    intersect_result: IntersectResult<T>,
    depth: i32,
) -> Color<T> {
    let IntersectResult {
        point,
        normal,
        material,
    } = intersect_result;
    if material.reflectiveness == T::zero() {
        return Color::zero();
    }

    let reflect_direction = reflect(ray.direction, normal).normalize();
    let reflected_color = scene.cast_ray(
        Ray {
            origin: point,
            direction: reflect_direction,
        },
        depth + 1,
    );
    reflected_color * material.reflectiveness
}

pub fn refracted_lightning<T: Real>(
    scene: &Scene<T>,
    ray: Ray<T>,
    intersect_result: IntersectResult<T>,
    depth: i32,
) -> Color<T> {
    let IntersectResult {
        point,
        normal,
        material,
    } = intersect_result;
    if material.transparency == T::zero() {
        return Color::zero();
    }

    if let Some(refract_direction) =
        refract(ray.direction, normal, material.refractive_index, T::one())
    {
        let refracted_color = scene.cast_ray(
            Ray {
                origin: point,
                direction: refract_direction.normalize(),
            },
            depth + 1,
        );
        refracted_color * material.transparency
    } else {
        Color::zero()
    }
}

fn reflect<T: Real>(direction: Vec3D<T>, normal: Vec3D<T>) -> Vec3D<T> {
    direction - (normal + normal) * (direction * normal)
}

fn refract<T: Real>(
    direction: Vec3D<T>,
    normal: Vec3D<T>,
    inner_refractive_index: T,
    outer_refractive_index: T,
) -> Option<Vec3D<T>> {
    let direction_projection = -direction * normal;
    if direction_projection < T::zero() {
        return refract(
            direction,
            -normal,
            outer_refractive_index,
            inner_refractive_index,
        );
    }

    let eta = outer_refractive_index / inner_refractive_index;
    let k = T::one() - eta.powi(2) * (T::one() - direction_projection.powi(2));
    if k < T::zero() {
        None
    } else {
        Some(direction * eta + normal * (eta * direction_projection - k.sqrt()))
    }
}
