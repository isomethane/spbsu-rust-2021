use num_traits::real::Real;

pub mod color;
pub mod ray;
pub mod shape;
pub mod vec3d;

pub fn threshold<T: Real>() -> T {
    T::from(0.0001).unwrap()
}
