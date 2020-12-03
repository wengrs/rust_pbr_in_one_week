pub mod vector;
pub mod color;
extern crate image;

fn main() {
    println!("P3\n255");
    println!("256 256");
    for i in 0..256 {
        for j in 0..256 {
            let r = (((i as f64) / 256.1)*256.) as u32;
            let g = (((j as f64) / 256.1)*256.) as u32;
            let b = 1;
            println!("{0} {1} {2}", r, g, b);
        }
    }
}
