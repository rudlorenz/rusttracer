extern crate overload;
use overload::overload;
use std::fmt;
use std::ops;

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

    pub fn unit_vector(direction: &Vec3) -> Vec3 {
        direction / direction.length()
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x_ * rhs.x_ + lhs.y_ * rhs.y_ + lhs.z_ * rhs.z_
    }

    pub fn length(&self) -> f64 {
        Vec3::dot(self, self).sqrt()
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
