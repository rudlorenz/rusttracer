mod structs;

use crate::structs::hitable::*;
use crate::structs::ray::Ray;
use crate::structs::sphere::Sphere;
use crate::structs::vec3::Vec3;
use crate::structs::viewport::Viewport;

extern crate image;
use image::Rgb;

use rand::Rng;

fn ray_col(r: &Ray, world: &HitList) -> Vec3 {
    let tryhit = world.hit(r, 0., f64::MAX);

    match tryhit {
        Some(hit_rec) => 0.5 * (hit_rec.normal_ + 1.),
        None => {
            let t = 0.5 * (Vec3::unit_vector(&r.direction()).y_ + 1.);
            (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let samples = 50;
    let viewport = Viewport::new();

    let hit_listy = HitList {
        elements_: vec![
            Box::new(Sphere::new(0.5, &Vec3::new(0., 0., -1.))),
            Box::new(Sphere::new(100., &Vec3::new(0., -100.5, -1.))),
        ],
    };

    let mut img_buf = image::ImageBuffer::new(nx, ny);

    let mut rng = rand::thread_rng();

    for j in 0..ny {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ns in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = ((ny - j) as f64 + rng.gen::<f64>()) / ny as f64;
                let r = viewport.send_ray(u, v);
                col = col + ray_col(&r, &hit_listy);
            }

            col = col / samples as f64;

            let ir = (255.99 * col.x_) as u8;
            let ig = (255.99 * col.y_) as u8;
            let ib = (255.99 * col.z_) as u8;

            img_buf.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
