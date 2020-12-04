pub mod vector;
pub mod color;
pub mod ray;
pub mod shape;
pub mod camera;
extern crate bmp;
use bmp::Image;
extern crate rand;
use rand::Rng;

fn ray_color(r: &ray::Ray, world: &Vec<Box<dyn shape::Shape>>, depth: i32) -> color::RGB {
    if depth < 0 {
        return color::RGB::black();
    }
    let hit = shape::hit_list(world, r, 0.0001, f64::INFINITY);      
    if hit.h == true {
        let ori = hit.p;
        let target = hit.p + hit.n + vector::Vec3d::rand_in_unit_sphere();
        let rr = ray::Ray::new(ori, target-hit.p);
        return (0.5*ray_color(&rr, &world, depth - 1).to_vec()).to_rgb();
    }
    let unit_dir = r.dir.norm();
    let t = 0.5*(unit_dir.y + 1.);
    ((1.-t)*vector::Vec3d::new(1., 1., 1.)+t*vector::Vec3d::new(0.5, 0.7, 1.)).to_rgb()
}

fn main() {
    let aspect_ratio = 16./9.;
    let img_width = 400 as u32;
    let img_height = (400./aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let world: Vec<Box<dyn shape::Shape>> = vec![
        Box::new(shape::Sphere{center: vector::Vec3d::new(0., 0., -1.), radius: 0.5}),
        Box::new(shape::Sphere{center: vector::Vec3d::new(0., -100.5, -1.), radius: 100.})];

    let cam = camera::Camera::new();

    let mut img = Image::new(img_width, img_height);
    for i in 0..img_width {
        for j in 0..img_height {
            let mut pixel_color = vector::Vec3d::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0, 1.0))/(img_width as f64 - 1.);
                let v = (j as f64 + rand::thread_rng().gen_range(0.0, 1.0))/(img_height as f64 - 1.);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + (ray_color(&r, &world, max_depth).to_vec() / samples_per_pixel as f64);
            }
            img.set_pixel(i, img_height-j-1, pixel_color.to_rgb().pixel());
        }
    }
    let _ = img.save("test.bmp");
}

/*
fn main() {
    let mut img = Image::new(256, 256);
    for i in 0..256 {
        for j in 0..256 {
            let r = (((i as f64) / 256.1)*256.) as u8;
            let g = (((j as f64) / 256.1)*256.) as u8;
            let b = 1;
            img.set_pixel(i, j, bmp::Pixel::new(r, g, b));
        }
    }
    let _ = img.save("test.bmp");
}
*/