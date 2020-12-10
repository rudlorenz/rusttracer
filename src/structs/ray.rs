use crate::structs::vec3::{Point3, Vec3};

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig_: Point3,
    direction_: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, direction: &Vec3) -> Ray {
        Ray {
            orig_: *orig,
            direction_: *direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig_
    }

    pub fn direction(&self) -> Vec3 {
        self.direction_
    }

    pub fn point_at(&self, t: f64) -> Point3 {
        self.orig_ + t * self.direction_
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.orig_, self.direction_)
    }
}
