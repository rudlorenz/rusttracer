use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

pub struct Viewport {
    origin_: Point3,
    horizontal_: Vec3,
    vertical_: Vec3,
    lower_left_corner_: Vec3,
}

impl Viewport {
    pub fn new() -> Viewport {
        let aspect_ratio = 16. / 9.;
        let v_height = 2.5;
        let v_width = v_height * aspect_ratio;
        let focal_len = 1.;
        Viewport {
            origin_: Point3::new(0.0, 0.0, 0.0),
            horizontal_: Vec3::new(v_width, 0.0, 0.0),
            vertical_: Vec3::new(0.0, v_height, 0.0),
            lower_left_corner_: Vec3::new(0. - v_width / 2., 0. - v_height / 2., 0. - focal_len),
        }
    }

    pub fn send_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin_,
            &(self.lower_left_corner_ + u * self.horizontal_ + v * self.vertical_ - self.origin_),
        )
    }
}
