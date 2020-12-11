use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};
use rand::Rng;

pub struct Viewport {
    lens_radius_: f64,
    origin_: Point3,
    horizontal_: Vec3,
    vertical_: Vec3,
    lower_left_corner_: Vec3,
    u_: Vec3,
    v_: Vec3,
}

impl Viewport {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Viewport {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let view_height = h;
        let view_width = aspect_ratio * view_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Viewport {
            lens_radius_: aperture / 2.,
            origin_: lookfrom,
            horizontal_: 2. * focus_dist * view_width * u,
            vertical_: 2. * focus_dist * view_height * v,
            lower_left_corner_: (lookfrom
                - view_width * focus_dist * u
                - view_height * focus_dist * v
                - focus_dist * w),
            u_: u,
            v_: v,
        }
    }

    pub fn send_ray<R: Rng>(&self, s: f64, t: f64, rng: &mut R) -> Ray {
        let rd = self.lens_radius_ * Vec3::random_in_unit_disk(rng);
        let offset = rd.x_ * self.u_ + rd.y_ * self.v_;

        Ray::new(
            &(self.origin_ + offset),
            &(self.lower_left_corner_ + s * self.horizontal_ + t * self.vertical_
                - self.origin_
                - offset),
        )
    }
}
