use crate::structs::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub enum Texture {
    SolidColor(Vec3),
    CheckerTexture(CheckerTexture),
}

impl Texture {
    pub fn new_solid_color(color: Vec3) -> Self {
        Texture::SolidColor(color)
    }

    pub fn new_checker_color(odd_col: Vec3, even_col: Vec3) -> Self {
        Texture::CheckerTexture(CheckerTexture {
            odd: odd_col,
            even: even_col,
        })
    }

    pub fn value(&self, _u: f64, _v: f64, p: &Point3) -> Vec3 {
        match self {
            Texture::SolidColor(color) => *color,
            Texture::CheckerTexture(checker) => checker.value(p),
        }
    }
}

#[derive(Clone, Copy)]
pub struct CheckerTexture {
    odd: Vec3,
    even: Vec3,
}

impl CheckerTexture {
    pub fn value(&self, p: &Point3) -> Vec3 {
        let sine = f64::sin(10f64 * p.x_) * f64::sin(10f64 * p.y_) * f64::sin(10f64 * p.z_);
        if sine < 0f64 {
            self.odd
        } else {
            self.even
        }
    }
}
