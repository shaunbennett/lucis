use super::geometry::Ray;
use image::{save_buffer, Rgb, RGB};
use nalgebra::{convert, Affine3, Isometry, Point3, Rotation3, Vector3, U3};
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use scene::{Color, Intersect, Light, SceneNode};
use std::slice;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use geometry::volume::{VolumetricSolid,Volume,VolumeEffect,BoxParams};

type Isometry3<N> = Isometry<N, U3, Rotation3<f32>>;

// TODO: Tracing Options
// pub struct TracingOptions {
//     super_sampling: bool,
//     shadow_rays: bool,
//     texture_mapping: bool,
//     phong_lighting: bool,
//     num_threads: u16,
// }

// impl Default for TracingOptions {
//     fn default() -> TracingOptions {
//         TracingOptions {
//             super_sampling: true,
//             shadow_rays: true,
//             texture_mapping: true,
//             phong_lighting: true,
//             num_threads: 1,
//         }
//     }
// }

pub struct Raytracer {
    pub root_node: SceneNode,

    // Viewing
    pub eye: Point3<f32>,
    pub view: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov_y: f32,

    // Lighting
    pub ambient: Color,
    pub lights: Vec<Light>,
    pub volumes: Vec<VolumetricSolid>,
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
            ambient: Color::new(0.0, 0.0, 0.0),
            lights: Vec::new(),
            volumes: Vec::new(),
        }
    }
}

const Z_NEAR: f32 = -1.0;

impl Raytracer {
    // Ray trace and save a specific image
    pub fn render(
        &self,
        file_name: &str,
        width: u32,
        height: u32, /*, options: TracingOptions*/
    ) {
        let view_matrix: Affine3<f32> =
            convert(Isometry3::look_at_rh(&self.eye, &self.view, &self.up));

        let side = -2.0f32 * (self.fov_y.to_radians() / 2.0f32).tan();
        let fw = width as f32;
        let fh = height as f32;

        let pixel_count = width * height;

        let pixels_rendered = Arc::new(AtomicUsize::new(0));

        let t_pr = pixels_rendered.clone();
        let pc_usize = pixel_count as usize;
        let completion_string = format!("Completed {}!", file_name);
        let progress_thread = thread::spawn(move || {
            let mut pb = ProgressBar::new(pc_usize as u64);
            pb.show_counter = false;
            pb.show_speed = false;
            pb.tick_format("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
            pb.format("[▱▱ ]");
            let mut value = t_pr.load(Ordering::Relaxed);
            while value < pc_usize {
                thread::sleep(Duration::from_millis(100));
                pb.set(value as u64);
                value = t_pr.load(Ordering::Relaxed);
            }
            pb.finish_print(&completion_string);
        });

        let image_pixels: Vec<Rgb<u8>> = (0..pixel_count)
            .into_par_iter()
            .map(|i| {
                let x = i % height;
                let y = i / width;
                let fx = x as f32 + 0.5;
                let fy = y as f32 + 0.5;
                let pixel_vec = view_matrix
                    * Vector3::new(
                        Z_NEAR * ((fx / fw) - 0.5) * side * fw / fh,
                        Z_NEAR * -((fy / fh) - 0.5) * side,
                        Z_NEAR,
                    );
                let ray = Ray::new(self.eye, pixel_vec);
                let color = self.trace_ray(width, height, &ray, x, y);
                pixels_rendered.fetch_add(1, Ordering::Relaxed);
                color.as_rgb()
            })
            .collect();

        // ENTERING SCARY ZONE, DON'T ASK QUESTIONS
        let transmuted_pixels = unsafe {
            slice::from_raw_parts(image_pixels.as_ptr() as *mut u8, (pixel_count * 3) as usize)
        };
        save_buffer(file_name, transmuted_pixels, width, height, RGB(8)).unwrap();
        progress_thread.join().unwrap();
    }

    fn trace_ray(&self, width: u32, height: u32, ray: &Ray, x: u32, y: u32) -> Color {
        let collision = self.root_node.intersects(ray);
        match collision {
            Some(c) => {
                let mut color = c.node.material.get_color(ray, self, &c);
                for volume in self.volumes.iter() {
                    // TODO: don't do this
                    color = volume.apply(ray, &collision, color)
                }
                color
            },
            None => {
                let mut color = get_background_color(x, y, width, height);
                for volume in self.volumes.iter() {
                    // TODO: don't do this
                    color = volume.apply(ray, &collision, color)
                }
                color
            }
        }
    }
}

fn get_background_color(_x: u32, y: u32, _width: u32, height: u32) -> Color {
    // let fw = width as f32;
    let fh = height as f32;
    let r_rate = 67.0f32 / 255.;
    let g_rate = 133.0f32 / 255.;
    let b_rate = 1.0f32;
    let height_rate = f32::max(0.0f32, (y as f32 / fh) - 0.2f32);

    if height_rate <= 0.35 {
        let rand_chance = if height_rate >= 0.05 {
            let reverse_height = 0.4f32 - height_rate;
            let percent = reverse_height / 0.35f32;
            percent * 0.003f32
        } else {
            0.005f32
        };

        let mut rng = thread_rng();
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

    Color::new(
        r_rate * height_rate,
        g_rate * height_rate,
        b_rate * height_rate,
    )
}
