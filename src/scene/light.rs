use std::default::Default;
use std::num;
use rand::{thread_rng, Rng};
use nalgebra::Point3;
use scene::Color;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Light {
    // The color of the light
    pub color: Color,
    // Position of the light in 3d space
    pub position: Point3<f32>,
    pub falloff: [f32; 3],
    // Radius of the light, used for soft shadows
    pub radius: f32,
    // How many times this light should be sampled
    pub num_samples: u32,
    pub light_samples: Vec<Point3<f32>>,
}

impl Light {
    pub fn new(color: Color, position: Point3<f32>, falloff: [f32; 3], radius: f32, num_samples: u32) -> Light {
        Light {
            color,
            position,
            falloff,
            radius,
            num_samples,
            light_samples: Light::generate_light_samples(&position, radius, num_samples),
        }
    }

    pub fn set_soft(&mut self, radius: f32, num_samples: u32) {
        self.radius = radius;
        self.num_samples = num_samples;
        self.light_samples = Light::generate_light_samples(&self.position, radius, num_samples);
    }

    /// Get a random point in the radius of the light
    /// Used to create soft shadows
    pub fn get_random_point(&self) -> Point3<f32> {
        let mut rng = thread_rng();
        let x_delta = rng.gen_range(-1.0f32, 1.0f32);
        let y_delta = rng.gen_range(-1.0f32, 1.0f32);
        let z_delta = rng.gen_range(-1.0f32, 1.0f32);
        Point3::new(x_delta, y_delta, z_delta) + self.position.coords
    }

    fn generate_light_samples(position: &Point3<f32>, radius: f32, num_samples: u32) -> Vec<Point3<f32>> {
        if radius == 0.0 || num_samples == 1 {
            return vec![*position];
        }

        // Generate an array of random light samples uniformly distributed across the light sphere
        let mut samples = vec![];
        let samples_per_dimension = (num_samples as f32).sqrt() as u32;
        println!("Samples per dimension: {}", samples_per_dimension);
        assert!(samples_per_dimension > 1);
        let inc = (radius * 2.0) / (samples_per_dimension - 1) as f32;

        for x in 0..samples_per_dimension {
            for y in 0..samples_per_dimension {
//                for z in 0..samples_per_dimension {
                    let x_delta = -radius + (x as f32 * inc);
                    let y_delta = -radius + (y as f32 * inc);
//                    let z_delta = -radius + (z as f32 * inc);
                    let sample = Point3::new(x_delta, y_delta, 0.0) + position.coords;
                    println!("Added sample: {}", sample);
                    samples.push(sample);
//                }
            }
        }
        samples
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            color: Color::new(1.0, 1.0, 1.0),
            position: Point3::new(0.0, 0.0, 0.0),
            // No falloff
            falloff: [1.0, 0.0, 0.0],
            // Hard lighting
            radius: 0.0,
            num_samples: 1,
            light_samples: vec![Point3::new(0.0, 0.0, 0.0)]
        }
    }
}
