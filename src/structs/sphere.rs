use crate::structs::hitable::{HitRecord, Hitable};
use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

pub struct Sphere {
    radius_: f64,
    center_: Point3,
    material_: Box<dyn Material>,
}

impl Sphere {
    pub fn new(radius: f64, center: &Point3, material: Box<dyn Material>) -> Sphere {
        Sphere {
            radius_: radius,
            center_: *center,
            material_: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center_;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius_ * self.radius_;
        let discr = b * b - a * c;
        if discr > 0. {
            let root = (-b - discr.sqrt()) / a;
            if root > t_min && root < t_max {
                return Some(HitRecord::new(
                    root,
                    &r.point_at(root),
                    &((r.point_at(root) - self.center_) / self.radius_),
                    self.material_.clone(),
                ));
            }
            let root = (-b + discr.sqrt()) / a;
            if root > t_min && root < t_max {
                return Some(HitRecord::new(
                    root,
                    &r.point_at(root),
                    &((r.point_at(root) - self.center_) / self.radius_),
                    self.material_.clone(),
                ));
            }
        }
        None
    }
}
