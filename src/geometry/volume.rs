use crate::geometry::{aabb_collision, Ray};
use crate::scene::{Color, Intersection};
use nalgebra::{distance, Affine3, Matrix4, Point3, Vector3};
use roots::{find_roots_quadratic, Roots};

pub struct VolumetricSolid {
    volume: Volume,
    effect: VolumeEffect,
}

impl VolumetricSolid {
    pub fn new(volume: Volume, effect: VolumeEffect) -> VolumetricSolid {
        VolumetricSolid { volume, effect }
    }

    pub fn apply(&self, ray: &Ray, ri: &Option<Intersection>, curr_color: Color) -> Color {
        match self.volume.passes_through(ray) {
            Some(intersection) => self.effect.apply(ray, ri, &intersection, curr_color),
            None => curr_color,
        }
    }
}

// Represents a volume that can be passed through by a ray
#[derive(Debug, Clone, PartialEq)]
pub enum Volume {
    Box(BoxParams),
    Cone(ConeParams),
}

impl Volume {
    fn passes_through(&self, ray: &Ray) -> Option<VolumeIntersection> {
        match self {
            Volume::Box(b) => box_passes_through(b, ray),
            Volume::Cone(c) => cone_passes_through(c, ray),
        }
    }
}

// Represents an effect on the resulting pixel a volume has while being passed through
pub enum VolumeEffect {
    // Fog color
    Fog(Color),
    // Color/Intensity
    Light(Color),
    Solid(Color),
    None,
}

impl VolumeEffect {
    fn apply(
        &self,
        ray: &Ray,
        ri: &Option<Intersection>,
        vi: &VolumeIntersection,
        curr_color: Color,
    ) -> Color {
        match self {
            VolumeEffect::Fog(color) => fog_apply(*color, ray, ri, vi, curr_color),
            VolumeEffect::Light(color) => light_apply(*color, ray, ri, vi, curr_color),
            VolumeEffect::Solid(color) => *color,
            VolumeEffect::None => curr_color,
        }
    }
}

fn light_apply(
    light_color: Color,
    ray: &Ray,
    ri: &Option<Intersection>,
    vi: &VolumeIntersection,
    curr_color: Color,
) -> Color {
    // Do calculation to apply fog effect
    let intensity = match ri {
        Some(ray_i) => {
            // Caluclate time in fog
            let enter = (vi.i_1 - ray.src).x / ray.dir.x;
            let leave = (vi.i_2 - ray.src).x / ray.dir.x;
            let t_intersect = (ray_i.point - ray.src).x / ray.dir.x;

            if enter >= t_intersect {
                return curr_color;
            }

            if leave < t_intersect {
                let i1 = vi.i_1;
                let i2 = vi.i_2;
                let distance = distance(&i1, &i2);
                distance * 0.2
            } else {
                let i1 = vi.i_1;
                let i2 = ray_i.point;
                let distance = distance(&i1, &i2);
                distance * 0.2
            }
        }
        None => {
            let i1 = vi.i_1;
            let i2 = vi.i_2;
            let distance = distance(&i1, &i2);
            distance * 0.2
        }
    }
    .max(0.0)
    .min(0.7);

    // if intensity > 0.0 {
    //     return Color::new(1.0, 0.0, 0.0);
    // }

    let r = (curr_color.r + (intensity * light_color.r)).min(1.0);
    let g = (curr_color.g + (intensity * light_color.g)).min(1.0);
    let b = (curr_color.b + (intensity * light_color.b)).min(1.0);

    Color::new(r, g, b)
}

fn fog_apply(
    fog_color: Color,
    ray: &Ray,
    ri: &Option<Intersection>,
    vi: &VolumeIntersection,
    curr_color: Color,
) -> Color {
    // Do calculation to apply fog effect
    let fog_amount = match ri {
        Some(ray_i) => {
            // Caluclate time in fog
            let enter = vi.t_enter.max(0.0);
            let leave = vi.t_leave;
            let t_intersect = (ray_i.point - ray.src).x / ray.dir.x;

            if enter >= t_intersect {
                return curr_color;
            }

            if leave < t_intersect {
                let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
                let i2 = ray.src + (vi.t_leave * ray.dir);
                let distance = distance(&i1, &i2);
                distance * 0.03
            } else {
                let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
                let i2 = ray_i.point;
                let distance = distance(&i1, &i2);
                distance * 0.03
            }
        }
        None => {
            let i1 = ray.src + (vi.t_enter.max(0.0) * ray.dir);
            let i2 = ray.src + (vi.t_leave * ray.dir);
            let distance = distance(&i1, &i2);
            distance * 0.03
        }
    }
    .max(0.0)
    .min(1.0);

    (fog_amount * fog_color) + ((1.0 - fog_amount) * curr_color)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct VolumeIntersection {
    // When the ray entered the volume
    pub t_enter: f32,
    // When the ray left the volume
    pub t_leave: f32,
    pub i_1: Point3<f32>,
    pub i_2: Point3<f32>,
}

impl VolumeIntersection {
    fn new(t_enter: f32, t_leave: f32, i_1: Point3<f32>, i_2: Point3<f32>) -> VolumeIntersection {
        VolumeIntersection {
            t_enter,
            t_leave,
            i_1,
            i_2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BoxParams {
    pub pos: Vector3<f32>,
    pub size: Vector3<f32>,
}

fn box_passes_through(b: &BoxParams, ray: &Ray) -> Option<VolumeIntersection> {
    let roots = aabb_collision(ray, &b.pos, &b.size);

    match roots {
        Roots::Two([t1, t2]) => Some(VolumeIntersection::new(
            t1,
            t2,
            ray.src + (t1 * ray.dir),
            ray.src + (t2 * ray.dir),
        )),
        Roots::One([t1]) => Some(VolumeIntersection::new(
            0.0f32,
            t1,
            ray.src,
            ray.src + (t1 * ray.dir),
        )),
        _ => None,
    }
}

fn cone_passes_through(cone: &ConeParams, ray: &Ray) -> Option<VolumeIntersection> {
    let transformed_ray = cone.inv_transform * *ray;

    let src = &transformed_ray.src;
    let dir = &transformed_ray.dir;

    let a = (dir.x * dir.x) + (dir.z * dir.z) - (dir.y * dir.y);
    let b = 2.0f32 * ((src.x * dir.x) + (src.z * dir.z) - (src.y * dir.y));
    let c = (src.x * src.x) + (src.z * src.z) - (src.y * src.y);

    match find_roots_quadratic(a, b, c) {
        Roots::One(_) => return None,
        Roots::Two([r1, r2]) => {
            let i_1 = transformed_ray.src + (r1 * transformed_ray.dir);
            if i_1.y >= 0.0 && i_1.y <= 3.0 {
                let i_2 = transformed_ray.src + (r2 * transformed_ray.dir);
                Some(VolumeIntersection::new(
                    r1,
                    r2,
                    cone.transform * i_1,
                    cone.transform * i_2,
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConeParams {
    pub pos: Vector3<f32>,
    pub scale_y: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub transform: Affine3<f32>,
    pub inv_transform: Affine3<f32>,
}

impl ConeParams {
    pub fn new(pos: Vector3<f32>, scale_y: f32, rot_x: f32, rot_y: f32, rot_z: f32) -> ConeParams {
        let mut cone_params = ConeParams {
            pos,
            scale_y,
            rot_x,
            rot_y,
            rot_z,
            transform: Affine3::identity(),
            inv_transform: Affine3::identity(),
        };

        cone_params.scale(1.0, 15.0, 1.0);
        cone_params.rotate("x", -90.0);
        // cone_params.rotate("z", 45.0);
        cone_params.rotate("y", 25.0);
        cone_params.translate(1.5, 0.77, -12.2);
        // cone_params.translate(1.5, 0.77, -20.2);

        cone_params
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.apply_transform(Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z)));
    }
    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.apply_transform(Matrix4::new_translation(&Vector3::new(x, y, z)));
    }
    fn rotate(&mut self, axis: &str, angle: f32) {
        let axis = match axis {
            "x" | "X" => Vector3::x_axis(),
            "y" | "Y" => Vector3::y_axis(),
            "z" | "Z" => Vector3::z_axis(),
            _ => panic!(
                "Got unexpected axis: \'{}\' while trying to apply rotation to cone volume",
                axis
            ),
        };
        self.apply_transform(Matrix4::from_axis_angle(&axis, angle.to_radians()));
    }
    fn apply_transform(&mut self, t: Matrix4<f32>) {
        let ta: Affine3<f32> = Affine3::from_matrix_unchecked(t);
        self.transform = ta * self.transform;
        self.inv_transform = self.transform.inverse();
    }
}
