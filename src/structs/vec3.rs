extern crate overload;
use overload::overload;
use std::fmt;
use std::ops;
use std::ops::{Index, IndexMut};

use rand::distributions::Uniform;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x_: f64,
    pub y_: f64,
    pub z_: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x_: x,
            y_: y,
            z_: z,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x_: 0.0,
            y_: 0.0,
            z_: 0.0,
        }
    }

    pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
        let rng_range = Uniform::new_inclusive(-1f64, 1f64);
        loop {
            let p = Vec3::new(
                rng_range.sample(rng),
                rng_range.sample(rng),
                rng_range.sample(rng),
            );

            if Vec3::dot(&p, &p) < 1. {
                break p;
            }
        }
    }

    pub fn random_unit<R: Rng>(rng: &mut R) -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere(rng))
    }

    pub fn random_in_hemisphere<R: Rng>(normal: &Vec3, rng: &mut R) -> Vec3 {
        let inside = Vec3::random_in_unit_sphere(rng);
        if Vec3::dot(&inside, &normal) > 0. {
            inside
        } else {
            -inside
        }
    }

    pub fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
        let rng_range = Uniform::new_inclusive(-1f64, 1f64);
        loop {
            let p = Vec3::new(rng_range.sample(rng), rng_range.sample(rng), 0.);
            if Vec3::dot(&p, &p) < 1. {
                break p;
            }
        }
    }

    pub fn unit_vector(direction: Vec3) -> Vec3 {
        direction / direction.length()
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x_ * rhs.x_ + lhs.y_ * rhs.y_ + lhs.z_ * rhs.z_
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x_: a.y_ * b.z_ - a.z_ * b.y_,
            y_: a.z_ * b.x_ - a.x_ * b.z_,
            z_: a.x_ * b.y_ - a.y_ * b.x_,
        }
    }

    pub fn length(&self) -> f64 {
        Vec3::dot(self, self).sqrt()
    }

    pub fn as_array(self) -> [f64; 3] {
        [self.x_, self.y_, self.z_]
    }
}

impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x_,
            1 => &self.y_,
            2 => &self.z_,
            _ => panic!("out of bounds!"),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x_,
            1 => &mut self.y_,
            2 => &mut self.z_,
            _ => panic!("out of bounds!"),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x_, self.y_, self.z_)
    }
}

overload!((lhs: ?Vec3) + (rhs: ?Vec3) -> Vec3 {
    Vec3 {
        x_: lhs.x_ + rhs.x_,
        y_: lhs.y_ + rhs.y_,
        z_: lhs.z_ + rhs.z_,
    }
});

overload!((lhs: ?Vec3) - (rhs: ?Vec3) -> Vec3 {
    Vec3 {
        x_: lhs.x_ - rhs.x_,
        y_: lhs.y_ - rhs.y_,
        z_: lhs.z_ - rhs.z_,
    }
});

overload!((lhs: ?Vec3) * (rhs: ?Vec3) -> Vec3 {
    Vec3 {
        x_: lhs.x_ * rhs.x_,
        y_: lhs.y_ * rhs.y_,
        z_: lhs.z_ * rhs.z_,
    }
});

overload!((lhs: f64) * (rhs: ? Vec3) -> Vec3 {
    Vec3 {
        x_: rhs.x_ * lhs,
        y_: rhs.y_ * lhs,
        z_: rhs.z_ * lhs,
    }
});

overload!((lhs: ?Vec3) / (rhs: f64) -> Vec3 {
    Vec3 {
        x_: lhs.x_ / rhs,
        y_: lhs.y_ / rhs,
        z_: lhs.z_ / rhs,
    }
});

overload!((lhs: ?Vec3) + (rhs: f64) -> Vec3 {
    Vec3 {
        x_: lhs.x_ + rhs,
        y_: lhs.y_ + rhs,
        z_: lhs.z_ + rhs,
    }
});

overload!((lhs: ?Vec3) - (rhs: f64) -> Vec3 {
    Vec3 {
        x_: lhs.x_ - rhs,
        y_: lhs.y_ - rhs,
        z_: lhs.z_ - rhs,
    }
});

overload!(-(op: ?Vec3) -> Vec3 {
        Vec3 {
        x_: -op.x_,
        y_: -op.y_,
        z_: -op.z_,
    }
});
