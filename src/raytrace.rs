use super::geometry::Ray;
use image::RgbImage;
use nalgebra::{Affine3, Isometry, Point3, Vector3};
use rand::Rng;
use scene::{Color, Intersect, SceneNode};

type Isometry3<N> = Isometry<N, nalgebra::U3, nalgebra::Rotation3<f32>>;

pub struct TracingOptions {
    super_sampling: bool,
    shadow_rays: bool,
    texture_mapping: bool,
    phong_lighting: bool,
    num_threads: u16,
}

impl Default for TracingOptions {
    fn default() -> TracingOptions {
        TracingOptions {
            super_sampling: true,
            shadow_rays: true,
            texture_mapping: true,
            phong_lighting: true,
            num_threads: 1,
        }
    }
}

pub struct Raytracer {
    pub root_node: SceneNode,

    // Viewing
    pub eye: Point3<f32>,
    pub view: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov_y: f32,

    // Lighting
    pub ambient: Vector3<f32>,
}

impl Default for Raytracer {
    fn default() -> Raytracer {
        let root = SceneNode::new(0, "root node".to_string());
        Raytracer {
            root_node: root,
            eye: Point3::new(0.0, 0.0, 0.0),
            view: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 0.0, 0.0),
            fov_y: 30.,
            ambient: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

const Z_NEAR: f32 = -1.0;

impl Raytracer {
    // Ray trace and save a specific image
    pub fn render(&self, file_name: &str, width: u32, height: u32, options: TracingOptions) {
        println!("Rendering!",);
        println!("Eye: {}", self.eye);
        println!("View: {}", self.view);
        println!("Up: {}", self.up);

        let mut img_buffer = RgbImage::new(width, height);

        let view_matrix: Affine3<f32> =
            nalgebra::convert(Isometry3::look_at_rh(&self.eye, &self.view, &self.up));

        let side = -2.0f32 * (self.fov_y.to_radians() / 2.0f32).tan();
        let fw = width as f32;
        let fh = height as f32;

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 + 0.5;
                let fy = y as f32 + 0.5;

                let pixel_vec = view_matrix * Vector3::new(
                    Z_NEAR * ((fx / fw) - 0.5) * side * fw / fh,
                    Z_NEAR * -((fy / fh) - 0.5) * side,
                    Z_NEAR,
                );
                let ray = Ray::new(self.eye, pixel_vec);
                let pixel_color = self.trace_ray(width, height, &ray, x, y);
                img_buffer.put_pixel(x, y, pixel_color.as_rgb());
            }
        }

        img_buffer.save(file_name).unwrap();
    }

    fn trace_ray(&self, width: u32, height: u32, ray: &Ray, x: u32, y: u32) -> Color {
        let collision = self.root_node.intersects(ray);
        match collision {
            Some(c) => c.node.material.get_color(&c),
            None => get_background_color(x, y, width, height),
        }
    }
}

fn get_background_color(x: u32, y: u32, width: u32, height: u32) -> Color {
    let fw = width as f32;
    let fh = height as f32;
    let r_rate = 67.0f32 / 255.;
    let g_rate = 133.0f32 / 255.;
    let b_rate = 255.0f32 / 255.;
    let height_rate = f32::max(0.0f32, (y as f32 / fh) - 0.2f32);

    if height_rate <= 0.35 {
        let mut rand_chance = 0.005f32;
        if height_rate >= 0.05 {
            let reverse_height = 0.4f32 - height_rate;
            let percent = reverse_height / 0.35f32;
            rand_chance = percent * 0.003f32;
        }

        let mut rng = rand::thread_rng();
        let render_star: f32 = rng.gen();
        if render_star <= rand_chance {
            // Render a star instead
            let gray_rand: f32 = rng.gen();
            let gray_range = 200.0f32;
            let gray = 55 + (gray_rand * gray_range) as i32;
            let value = gray as f32 / 255.0f32;
            return Color::new(value, value, value);
        }
    }

    return Color::new(
        r_rate * height_rate,
        g_rate * height_rate,
        b_rate * height_rate,
    );
}
