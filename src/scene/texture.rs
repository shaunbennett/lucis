use scene::Color;
use image::{RgbImage,open};

#[derive(Debug, Clone)]
pub struct Texture {
    image: RgbImage,
    u_max: f32,
    v_max: f32,
}

impl Texture {
    pub fn load_texture(file_name: &str, u_max: f32, v_max: f32) -> Texture {
        // TODO: Error handling
        let image = open(file_name).unwrap().to_rgb();
        Texture {
            image,
            u_max,
            v_max,
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let u_mapped = (u / self.u_max).fract() * self.u_max;
        let v_mapped = (v / self.v_max).fract() * self.v_max;
        assert!(u_mapped <= self.u_max);
        assert!(v_mapped <= self.v_max);
        // println!("umapped: {}, vmapped: {}", u_mapped, v_mapped);

        let (width, height) = self.image.dimensions();
        let pixel_x = (u_mapped * (width-1) as f32).floor() as u32;
        let pixel_y = (v_mapped * (height-1) as f32).floor() as u32;

        Color::from_rgb(self.image.get_pixel(pixel_x, pixel_y))
    }
}