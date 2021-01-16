use crate::structs::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub enum Texture {
    SolidColor(Vec3),
}

impl Texture {
    pub fn new_solid_color(color: Vec3) -> Self {
        Texture::SolidColor(color)
    }

    pub fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Vec3 {
        match self {
            Texture::SolidColor(color) => *color,
        }
    }
}
