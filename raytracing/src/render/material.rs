use crate::math::color::Color;
use num_traits::real::Real;

#[derive(Copy, Clone)]
pub struct Material<T: Real> {
    pub ambient: Color<T>,
    pub diffuse: Color<T>,
    pub specular: Color<T>,
    pub shininess: T,
    pub reflectiveness: T,
    pub transparency: T,
    pub refractive_index: T,
}

impl<T: Real> Default for Material<T> {
    fn default() -> Self {
        Material {
            ambient: Color::default(),
            diffuse: Color::default(),
            specular: Color::default(),
            shininess: T::zero(),
            reflectiveness: T::zero(),
            transparency: T::zero(),
            refractive_index: T::one(),
        }
    }
}
