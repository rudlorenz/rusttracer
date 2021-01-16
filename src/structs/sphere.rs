use crate::structs::aabb::AABB;
use crate::structs::hitable::{HitRecord, Hitable};
use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

pub struct Sphere {
    radius: f64,
    center: Point3,
    material: Material,
    bbox: AABB,
}

impl Sphere {
    pub fn new(radius: f64, center: Point3, material: Material) -> Sphere {
        let bbox = AABB::new(
            center - Vec3::new(radius, radius, radius),
            center + Vec3::new(radius, radius, radius),
        );
        Sphere {
            radius,
            center,
            material,
            bbox,
        }
    }

    fn get_texture_coordinates(p: &Vec3) -> (f64, f64) {
        let theta = (-p.y_).acos();
        let phi = (-p.z_).atan2(p.x_ + std::f64::consts::PI);

        let u = phi / (2f64 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr = b * b - a * c;
        if discr > 0. {
            let rt = (-b - discr.sqrt()) / a;
            let root;
            if t_min < rt && rt < t_max {
                root = rt;
            } else {
                root = (-b + discr.sqrt()) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let normal = (ray.point_at(root) - self.center) / self.radius;
            let front_face = Vec3::dot(&ray.direction(), &normal) < 0.;
            let normal = if front_face { normal } else { -normal };
            let (u_texture, v_texture) = Sphere::get_texture_coordinates(&normal);

            return Some(HitRecord::new(
                front_face,
                root,
                u_texture,
                v_texture,
                ray.point_at(root),
                normal,
                self.material,
            ));
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
