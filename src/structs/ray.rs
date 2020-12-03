use crate::structs::vec3::Vec3;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig_: Vec3,
    direction_: Vec3,
}

impl Ray {
    pub fn new(orig: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            orig_: *orig,
            direction_: *direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig_
    }

    pub fn direction(&self) -> Vec3 {
        self.direction_
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.orig_ + t * self.direction_
    }

    pub fn color(r: &Ray) -> Vec3 {
        let unit_dir: Vec3 = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_dir.y_ + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.7, 0.7, 1.0)
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.orig_, self.direction_)
    }
}
