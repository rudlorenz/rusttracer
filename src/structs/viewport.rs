use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub struct Viewport {
    origin_: Vec3,
    lower_left_corner_: Vec3,
    horizontal_: Vec3,
    vertical_: Vec3,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            origin_: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner_: Vec3::new(-2.0, -1.0, -1.0),
            horizontal_: Vec3::new(4.0, 0.0, 0.0),
            vertical_: Vec3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn send_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin_,
            &(self.lower_left_corner_ + u * self.horizontal_ + v * self.vertical_ - self.origin_),
        )
    }
}
