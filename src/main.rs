#![feature(test)]

extern crate test;
extern crate image;
extern crate nalgebra;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;
use image::{RgbImage, Rgb};

fn main() {
    println!("Hello, world!");

    // create_image();
    create_image_png();
}

fn create_image() {
    let width: u32 = 1024;
    let height: u32 = 1024;
    let mut img_buffer = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            img_buffer.put_pixel(x, y, Rgb([255u8, 0, 0]));
        }
    }

    img_buffer.save("test.png").unwrap();
}

fn create_image_png() {
    let width: u32 = 1024;
    let height: u32 = 1024;

    let path = Path::new(r"test2.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut img_buffer: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
    unsafe { img_buffer.set_len((width * height * 3) as usize); }
    for x in 0..width {
        for y in 0..height {
            let index = (((y * width) + x) * 3) as usize;
            img_buffer[index] = 255;
            img_buffer[index + 1] = 0;
            img_buffer[index + 2] = 0;
        }
    }

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&img_buffer).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_image(b: &mut Bencher) {
        b.iter(|| create_image());
    }

    #[bench]
    fn bench_png(b: &mut Bencher) {
        b.iter(|| create_image_png());
    }
}
