mod structs;

use crate::structs::hitable::*;
use crate::structs::material::*;
use crate::structs::ray::Ray;
use crate::structs::sphere::Sphere;
use crate::structs::vec3::{Point3, Vec3};
use crate::structs::viewport::Viewport;

use rayon::prelude::*;

extern crate image;
use image::{ImageBuffer, Rgb};

use rand::prelude::*;

fn ray_col<R: Rng>(r: &Ray, world: &HitList, rng: &mut R, depth: u32) -> Vec3 {
    match world.hit(r, 0.001, f64::MAX) {
        Some(hit_rec) => {
            if depth < 50 {
                if let Some(scatter_vec) = hit_rec.material_.scatter(&r, &hit_rec, rng) {
                    hit_rec.material_.attenuation() * ray_col(&scatter_vec, &world, rng, depth + 1)
                } else {
                    Vec3::new(0., 0., 0.)
                }
            } else {
                Vec3::new(0., 0., 0.)
            }
        }
        None => {
            let t = 0.5 * (Vec3::unit_vector(&r.direction()).y_ + 1.);
            (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 1280u32;
    let ny = 720u32;
    let samples = 50;
    let viewport = Viewport::new();

    let hit_listy = HitList {
        elements_: vec![
            Box::new(Sphere::new(
                0.5,
                &Point3::new(0., 0., -1.),
                Material::new_lambertian(&Vec3::new(0.8, 0.3, 0.3)),
            )),
            Box::new(Sphere::new(
                100.,
                &Vec3::new(0., -100.5, -1.),
                Material::new_metal(&Vec3::new(0.4, 0.4, 0.4), 0.7),
            )),
            Box::new(Sphere::new(
                0.5,
                &Point3::new(1., 0., -1.),
                Material::new_metal(&Vec3::new(0.5, 0.8, 0.2), 0.5),
            )),
            Box::new(Sphere::new(
                0.5,
                &Point3::new(-1., 0., -1.),
                //Material::new_metal(&Vec3::new(0.2, 0.2, 0.6), 0.),
                Material::new_dielectric(1.5),
            )),
        ],
    };

    let raw_pixels: Vec<u8> = (0..ny)
        .into_par_iter()
        .rev()
        .map_init(rand::thread_rng, |rng, j| {
            let mut rslt = Vec::with_capacity(3 * nx as usize);
            for i in 0..nx {
                let mut col = Vec3::new(0., 0., 0.);
                for _ns in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                    let r = viewport.send_ray(u, v);
                    col = col + ray_col(&r, &hit_listy, rng, 0);
                }

                col = col / samples as f64;

                let ir = (256. * num::clamp(col.x_.sqrt(), 0., 0.999)) as u8;
                let ig = (256. * num::clamp(col.y_.sqrt(), 0., 0.999)) as u8;
                let ib = (256. * num::clamp(col.z_.sqrt(), 0., 0.999)) as u8;

                rslt.push(ir);
                rslt.push(ig);
                rslt.push(ib);
            }
            rslt
        })
        .flatten()
        .collect();

    let img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_vec(nx, ny, raw_pixels).unwrap();

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
