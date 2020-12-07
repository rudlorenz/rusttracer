mod structs;

use crate::structs::hitable::*;
use crate::structs::ray::Ray;
use crate::structs::sphere::Sphere;
use crate::structs::vec3::Vec3;
use crate::structs::viewport::Viewport;

use rayon::prelude::*;

extern crate image;
use image::{ImageBuffer, Rgb};

use rand::Rng;

fn random_unit_in_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = 2. * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1., 1., 1.);

        if Vec3::dot(&p, &p) < 1. {
            break p;
        }
    }
}

fn ray_col<R: Rng>(r: &Ray, world: &HitList, rng: &mut R) -> Vec3 {
    let tryhit = world.hit(r, 0., f64::MAX);

    match tryhit {
        Some(hit_rec) => {
            let target = hit_rec.normal_ + random_unit_in_sphere(rng);
            0.5 * ray_col(&Ray::new(&hit_rec.p_, &target), &world, rng)
        }
        None => {
            let t = 0.5 * (Vec3::unit_vector(&r.direction()).y_ + 1.);
            (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 800u32;
    let ny = 400u32;
    let samples = 50;
    let viewport = Viewport::new();

    let hit_listy = HitList {
        elements_: vec![
            Box::new(Sphere::new(0.5, &Vec3::new(0., 0., -1.))),
            Box::new(Sphere::new(100., &Vec3::new(0., -100.5, -1.))),
        ],
    };

    //let mut img_buf = image::ImageBuffer::new(nx, ny);

    let raw_pixels: Vec<u8> = (0..ny)
        .into_par_iter()
        .rev()
        .map_init(
            || rand::thread_rng(),
            |mut rng, j| {
                let mut rslt = Vec::with_capacity(3 * nx as usize);
                for i in 0..nx {
                    let mut col = Vec3::new(0., 0., 0.);
                    for _ns in 0..samples {
                        let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                        let r = viewport.send_ray(u, v);
                        col = col + ray_col(&r, &hit_listy, &mut rng);
                    }

                    col = col / samples as f64;

                    let ir = (255.99 * col.x_) as u8;
                    let ig = (255.99 * col.y_) as u8;
                    let ib = (255.99 * col.z_) as u8;

                    rslt.push(ir);
                    rslt.push(ig);
                    rslt.push(ib);
                }
                rslt
            },
        )
        .flatten()
        .collect();

    let img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_vec(nx, ny, raw_pixels).unwrap();

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
