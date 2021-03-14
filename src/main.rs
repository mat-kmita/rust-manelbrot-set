use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;

use std::ops::{Add, Sub, Div, Mul};

#[derive(Copy, Clone)]
struct Complex64 {
    re: f64,
    im: f64
}

impl Complex64 {
    fn abs(self) -> f64{
        (self.im * self.im + self.re * self.re)
    }
}

impl Add for Complex64 {
    type Output = Complex64;

    fn add(self, rhs: Complex64) -> Self::Output {
        Complex64 {
            re: rhs.re + self.re,
            im: rhs.im + self.im
        }
    }
}

impl Sub for Complex64 {
    type Output = Complex64;

    fn sub(self, rhs: Complex64) -> Self::Output {
        Complex64 {
            re: self.re - rhs.re,
            im: -self.im - rhs.im
        }
    }
}

impl Div for Complex64 {
    type Output = Complex64;

    fn div(self, rhs: Complex64) -> Self::Output {
        let den: f64 = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex64 {
            re: (self.re * rhs.re + self.im * rhs.im) / den,
            im: - (self.re * rhs.im + self.im * rhs.re) / den
        }
    }
}

impl Mul for Complex64 {
    type Output = Complex64;

    fn mul(self, rhs: Complex64) -> Self::Output {
        Complex64 {
            re: self.re * rhs.re - self.im * rhs.im,
            im: rhs.re * self.im + self.re * rhs.im 
        }
    }
}


struct Image24Bits {
    pixels: Vec<(u8,u8,u8)>,
    width: usize,
    height: usize
}

impl Image24Bits {
    fn new(width: usize, height: usize, default: (u8, u8, u8)) -> Image24Bits {
        Image24Bits {
            pixels: vec![default; width * height],
            width,
            height
        }
    }

    fn save_to_file(self, file: &mut File) {
        let header: &str = &format!(
    "P3\n# Created in Rust\n{} {}\n255\n", self.width, self.height);

        match file.write_all(header.as_bytes()) {
            Err(e) => panic!("couldn't write header: {}", e),
            Ok(_) => {}
        }

        let max = 8192;
        let mut buffered_writer = BufWriter::with_capacity(max, file);
        for (r, g, b) in &self.pixels {
            match writeln!(buffered_writer, "{} {} {}\n", r, g, b) {
                Err(e) => panic!("couldn't write image tofile: {}", e),
                Ok(_) => {} 
            }
        }

        match buffered_writer.flush() {
            Err(e) => panic!("couldn't flush buffer to image file: {}", e),
            Ok(_) => {}
        }
    }

    fn set_pixel(&mut self, column: usize, row: usize, color: (u8, u8, u8)) {
        std::mem::replace(&mut self.pixels[self.width * row + column], color);
    }
}

fn mandelbrot(point: Complex64) -> (u8, u8, u8) {
    let mut z = Complex64{re: 0.0, im: 0.0};

    for i in 1..255 {
        if Complex64::abs(z) > 4.0 {
            // return (i % 4 * 64, i % 8 * 32, i % 16 * 16);
            return (i,i,i)
        }
        z = (z * z) + point;
    }

    (0,0,0)
}

fn main() {
    let size = 4096;
    let mut image = Image24Bits::new(size, size, (0,0,0));
    
    let re_start = -2.0;
    let re_end = 1.0;
    let im_start = -1.0;
    let im_end = 1.0;

    for i in 0..size {
        for j in 0..size {
            let c = Complex64 {
                re: re_start + (i as f64 /size as f64) * (re_end - re_start),
                im: im_start + (j as f64 /size as f64) * (im_end - im_start)
            };
            let pixel = mandelbrot(c);
            image.set_pixel(i, j, pixel);
        }
    }

    let path = Path::new("image.ppm");
    let mut file = match File::create(&path) {
        Err(e) => panic!("Cannot create file {}: {}", path.display(), e),
        Ok(file) => file
    };

    image.save_to_file(& mut file);

}
