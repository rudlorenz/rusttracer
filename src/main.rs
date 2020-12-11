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
    if depth != 0 {
        match world.hit(r, 0.001, f64::MAX) {
            Some(hit_rec) => {
                if let Some(scatter_vec) = hit_rec.material_.scatter(&r, &hit_rec, rng) {
                    hit_rec.material_.attenuation() * ray_col(&scatter_vec, &world, rng, depth - 1)
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
    else {
        Vec3::new(0., 0., 0.)
    }
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let img_width = 1200u32;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples = 100;
    let ray_depth = 50;

    let lookfrom = Vec3::new(1., 1., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    let vertical_fov = 60.;

    let viewport = Viewport::new(
        lookfrom,
        lookat,
        vup,
        vertical_fov,
        aspect_ratio,
        aperture,
        dist_to_focus
    );

    let hit_listy = HitList {
        elements_: vec![
            Box::new(Sphere::new(
                0.49,
                &Point3::new(0., 0., -1.),
                Material::new_lambertian(&Vec3::new(0.8, 0.3, 0.3)),
            )),
            Box::new(Sphere::new(
                0.49,
                &Point3::new(1., 0., -1.),
                Material::new_metal(&Vec3::new(0.5, 0.8, 0.2), 0.5),
            )),
            Box::new(Sphere::new(
                0.49,
                &Point3::new(-1., 0., -1.),
                Material::new_dielectric(1.5),
            )),
            Box::new(Sphere::new(
                100.,
                &Vec3::new(0., -100.5, -1.),
                Material::new_metal(&Vec3::new(0.4, 0.4, 0.4), 0.7),
            )),
        ],
    };

    let raw_pixels: Vec<u8> = (0..img_height)
        .into_par_iter()
        .rev()
        .map_init(rand::thread_rng, |rng, j| {
            let mut rslt = Vec::with_capacity(3 * img_width as usize);
            for i in 0..img_width {
                let mut col = Vec3::new(0., 0., 0.);
                for _ns in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / img_width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / img_height as f64;
                    let r = viewport.send_ray(u, v, rng);
                    col = col + ray_col(&r, &hit_listy, rng, ray_depth);
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
        image::ImageBuffer::from_vec(img_width, img_height, raw_pixels).unwrap();

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
