use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vec3d::Vec3D;
use crate::render::scene::Scene;
use image::{ImageBuffer, ImageResult, Rgb};
use num_traits::real::Real;
use std::path::Path;

pub struct Frame<T: Real> {
    pub width: u32,
    pub height: u32,
    pub field_of_view: f64,
    pub(in crate::render) frame_buffer: Vec<Vec<Color<T>>>,
}

impl<T: Real> Frame<T> {
    pub fn new(width: u32, height: u32, field_of_view_degrees: f64) -> Frame<T> {
        Frame {
            width,
            height,
            field_of_view: field_of_view_degrees.to_radians(),
            frame_buffer: vec![vec!(Color::default(); width as usize); height as usize],
        }
    }

    pub fn render(&mut self, scene: &Scene<T>)
    where
        T: From<f64>,
    {
        for y_index in 0..self.height {
            for x_index in 0..self.width {
                let dir_x = (x_index as f64 + 0.5) - self.width as f64 / 2.0;
                let dir_y = -(y_index as f64 + 0.5) + self.height as f64 / 2.0;
                let dir_z = -(self.height as f64) / (2.0 * (self.field_of_view / 2.0).tan());

                let direction = Vec3D {
                    x: dir_x.into(),
                    y: dir_y.into(),
                    z: dir_z.into(),
                }
                .normalize();
                let ray = Ray {
                    origin: Vec3D::default(),
                    direction,
                };
                self.frame_buffer[y_index as usize][x_index as usize] = scene.cast_ray(ray, 0);
            }
        }
    }
}

impl<T: Real> Frame<T> {
    pub fn save<Q: AsRef<Path>>(&self, path: Q) -> ImageResult<()> {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            *pixel = self.frame_buffer[y as usize][x as usize].into();
        }
        image_buffer.save(path)
    }

    pub fn save_compressed<Q: AsRef<Path>>(&self, path: Q) -> ImageResult<()> {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(self.width / 2, self.height / 2);
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let x_index = x as usize * 2;
            let y_index = y as usize * 2;
            let color: Color<T> = vec![
                self.frame_buffer[y_index][x_index],
                self.frame_buffer[y_index][x_index + 1],
                self.frame_buffer[y_index + 1][x_index],
                self.frame_buffer[y_index + 1][x_index + 1],
            ]
            .into_iter()
            .fold(Color::zero(), |sum, x| sum + x)
                / 4;
            *pixel = color.into();
        }
        image_buffer.save(path)
    }
}
