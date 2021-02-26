use crate::structs::hitable::HitList;
use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

use rand::Rng;

use rayon::prelude::*;

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

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
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
            self.origin_ + offset,
            self.lower_left_corner_ + s * self.horizontal_ + t * self.vertical_
                - self.origin_
                - offset,
        )
    }

    fn ray_col<R: Rng>(r: &Ray, scene: &HitList, rng: &mut R, depth: u32) -> Vec3 {
        if depth != 0 {
            match scene.hit(r, 0.001, f64::MAX) {
                Some(hit_rec) => {
                    if let Some(scatter_vec) = hit_rec.material.scatter(&r, &hit_rec, rng) {
                        hit_rec.material.attenuation()
                            * Viewport::ray_col(&scatter_vec, &scene, rng, depth - 1)
                    } else {
                        Vec3::new(0., 0., 0.)
                    }
                }
                None => {
                    let t = 0.5 * (Vec3::unit_vector(r.direction()).y_ + 1.);
                    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
                }
            }
        } else {
            Vec3::new(0., 0., 0.)
        }
    }

    pub fn render(
        &self,
        img_width: u32,
        img_height: u32,
        samples: u32,
        ray_depth: u32,
        scene: HitList,
    ) -> Vec<u8> {
        (0..img_height)
            .into_par_iter()
            .rev()
            .map_init(rand::thread_rng, |rng, j| {
                let mut result = Vec::with_capacity(3 * img_width as usize);
                for i in 0..img_width {
                    let mut col = Vec3::new(0., 0., 0.);
                    for _ns in 0..samples {
                        let u = (i as f64 + rng.gen::<f64>()) / img_width as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / img_height as f64;
                        let r = self.send_ray(u, v, rng);
                        col = col + Viewport::ray_col(&r, &scene, rng, ray_depth);
                    }

                    col = col / samples as f64;

                    let ir = (256. * num::clamp(col.x_.sqrt(), 0., 0.999)) as u8;
                    let ig = (256. * num::clamp(col.y_.sqrt(), 0., 0.999)) as u8;
                    let ib = (256. * num::clamp(col.z_.sqrt(), 0., 0.999)) as u8;

                    result.push(ir);
                    result.push(ig);
                    result.push(ib);
                }
                result
            })
            .flatten()
            .collect()
    }
}
