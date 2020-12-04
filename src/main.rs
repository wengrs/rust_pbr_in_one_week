pub mod vector;
pub mod color;
pub mod ray;
pub mod shape;
extern crate bmp;
use bmp::Image;

fn hit_sphere(cen: vector::Vec3d, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.ori - cen;
    let a = vector::Vec3d::dot(r.dir, r.dir);
    let b = 2. * vector::Vec3d::dot(oc, r.dir);
    let c = vector::Vec3d::dot(oc, oc) - radius * radius;
    let dis = b*b - 4.*a*c;
    if dis < 0. {
        return -1.;
    }
    else {
        return (-b-dis.sqrt()) / (2.*a)
    }
}

fn ray_color(r: &ray::Ray) -> color::RGB {
    let t = hit_sphere(vector::Vec3d::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.at(t) - vector::Vec3d::new(0.,0.,-1.)).norm();
        return color::RGB::from_vec(0.5*(n+vector::Vec3d::one()));
    }
    let unit_dir = r.dir.norm();
    let t = 0.5*(unit_dir.y + 1.);
    color::RGB::from_vec((1.-t)*vector::Vec3d::new(1., 1., 1.)+t*vector::Vec3d::new(0.5, 0.7, 1.))
}

fn main() {
    let aspect_ratio = 16./9.;
    let img_width = 400 as u32;
    let img_height = (400./aspect_ratio) as u32;

    let view_height = 2.;
    let view_width = view_height*aspect_ratio;
    let focal_length = 1.;

    let origin = vector::Vec3d::zero();
    let horizontal = vector::Vec3d::new(view_width, 0., 0.);
    let vertical = vector::Vec3d::new(0., view_height, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - vector::Vec3d::new(0., 0., focal_length);

    let mut img = Image::new(img_width, img_height);
    for i in 0..img_width {
        for j in 0..img_height {
            let u = i as f64 / (img_width - 1) as f64;
            let v = j as f64 / (img_height - 1) as f64;
            let r = ray::Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&r);
            img.set_pixel(i, j, pixel_color.pixel())
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