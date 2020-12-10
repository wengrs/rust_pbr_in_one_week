pub mod vector;
use vector::Vec3d;
pub mod color;
pub mod ray;
pub mod shape;
use crate::shape::Shape;
pub mod camera;
pub mod material;
pub mod aabb;
extern crate bmp;
use bmp::Image;
extern crate rand;
use rand::Rng;
use std::sync::Arc;

fn ray_color(r: &ray::Ray, world: &shape::Objects, depth: i32) -> color::RGB {
    if depth < 0 {
        return color::RGB::black();
    }
    let hit = world.hit(r, 0.0001, f64::INFINITY);      
    if hit.h == true {
        let scatter = hit.mat.scatter(&r, &hit);
        if scatter.s == true {
            return (scatter.a.to_vec() * ray_color(&scatter.r, &world, depth - 1).to_vec()).to_rgb()
        }
        else
        {
            return color::RGB::black();
        }
    }
    let unit_dir = r.dir.norm();
    let t = 0.5*(unit_dir.y + 1.);
    ((1.-t)*vector::Vec3d::new(1., 1., 1.)+t*vector::Vec3d::new(0.5, 0.7, 1.)).to_rgb()
}

fn main() {
    let aspect_ratio = 16./9.;
    let img_width = 400 as u32;
    let img_height = (img_width as f64/aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 20;
    
    let look_from = vector::Vec3d::new(13., 2., 3.);
    let look_at = vector::Vec3d::new(0.,0.,0.);
    let up = vector::Vec3d::new(0., 1., 0.);
    let focus_length = 10.;
    let aperture = 0.1;
    let cam = camera::Camera::new(look_from, look_at, up, 20., aspect_ratio, aperture, focus_length, 0., 1.);

    let world = shape::Objects::new(random_scene());

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
        println!("{0}/{1}", i+1, img_width);
    }
    let _ = img.save("test.bmp");
}

fn random_scene() -> Vec<Box<dyn shape::Shape>> {
    let mut world: Vec<Box<dyn shape::Shape>> = Vec::new();
    let ground_mat = Arc::new(material::Lambertian{albedo: color::RGB::new(0.5, 0.5, 0.5)});
    world.push(Box::new(shape::Sphere{center:Vec3d::new(0.,-1000.,0.), radius:1000., mat:ground_mat.clone()}));
    for a in -5..6 {
        for b in -5..6 {
            let radius = 0.2;
            let choose_mat = rand::thread_rng().gen_range(0.,1.);
            let center = Vec3d::new(a as f64 + 0.9*rand::thread_rng().gen_range(0.,1.),
                                    0.2,
                                    b as f64 + 0.9*rand::thread_rng().gen_range(0.,1.));
            if choose_mat < 0.8 {
                let albedo = Vec3d::rand_vec(0., 1.).to_rgb();
                let mat = Arc::new(material::Lambertian{albedo});
                let center2 = center + Vec3d::new(0., rand::thread_rng().gen_range(0., 0.5), 0.);
                world.push(Box::new(shape::MovingSphere{c0:center, c1:center2, t0:0., t1:1., radius, mat}));
            }
            else if choose_mat < 0.95 {
                let albedo = Vec3d::rand_vec(0.5, 1.).to_rgb();
                let fuzz = rand::thread_rng().gen_range(0.,0.5);
                let mat = Arc::new(material::Metal{albedo, fuzz});
                world.push(Box::new(shape::Sphere{center, radius, mat}));
            }
            else {
                let mat = Arc::new(material::Dielectric{ir:1.5});
                world.push(Box::new(shape::Sphere{center, radius, mat}));
            }
        }
    }
    let radius = 1.;
    let mat = Arc::new(material::Dielectric{ir:1.5});
    world.push(Box::new(shape::Sphere{center:Vec3d::new(0.,1.,0.), radius, mat}));
    let mat = Arc::new(material::Lambertian{albedo:color::RGB::new(0.4,0.2,0.1)});
    world.push(Box::new(shape::Sphere{center:Vec3d::new(-4.,1.,0.), radius, mat}));
    let mat = Arc::new(material::Metal{albedo:color::RGB::new(0.7,0.6,0.5),fuzz:0.});
    world.push(Box::new(shape::Sphere{center:Vec3d::new(4.,1.,0.), radius, mat}));
    world
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