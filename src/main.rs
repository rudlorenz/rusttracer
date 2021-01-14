use rusttracer::structs::bvh::BVH;
use rusttracer::structs::vec3::Vec3;
use rusttracer::structs::viewport::Viewport;

use image::{ImageBuffer, Rgb};

fn main() {
    let aspect_ratio = 16. / 9.;
    let img_width = 1200u32;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples = 50;
    let ray_depth = 50;

    let lookfrom = Vec3::new(8., 3., 2.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let vertical_fov = 60.;

    let viewport = Viewport::new(
        lookfrom,
        lookat,
        vup,
        vertical_fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let raw_pixels = viewport.render(
        img_width,
        img_height,
        samples,
        ray_depth,
        BVH::new(rusttracer::benchmarking_scene()),
    );

    let img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_vec(img_width, img_height, raw_pixels).unwrap();

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
