use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rusttracer::structs::vec3::Vec3;
use rusttracer::structs::viewport::Viewport;

fn setup_and_run(samples: u32) -> Vec<u8> {
    let aspect_ratio = 16. / 9.;
    let img_width = 1200u32;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let ray_depth = 50;

    let lookfrom = Vec3::new(8., 2., 2.);
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

    viewport.render(
        img_width,
        img_height,
        samples,
        ray_depth,
        rusttracer::benchmarking_scene(),
    )
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("100scene", |b| b.iter(|| setup_and_run(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
