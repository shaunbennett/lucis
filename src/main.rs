extern crate image;
extern crate lucis;

use lucis::geometry::Primitive;
use lucis::geometry::Ray;
use lucis::lua;
use lucis::model::{Collidable, SceneNode};
use lucis::{Point, Raytracer, TracingOptions, Transform, Vector};

fn main() {
    // let mut child = SceneNode::new(1, "child node".to_string());
    // let mut root = SceneNode::new(0, "root node".to_string());

    // child.primitive = Primitive::Sphere;
    // root.add_child(child);

    lua::test();

    // let options: TracingOptions = Default::default();
    // let tracer: Raytracer = Raytracer {
    //     root_node: root,
    //     eye: Point::new(0.0, 0.0, 5.0),
    //     view: Point::new(0.0, 0.0, 0.0),
    //     up: Vector::new(0.0, 1.0, 0.0),
    //     fov_y: 30.0,
    //     ambient: Vector::new(0.1, 0.1, 0.1),
    // };
    // tracer.render(256, 256, options);
    //    tracer.render(1024, 1024, options);

    // println!("{:#?}", root);
}

// fn create_image() {
//     let width: u32 = 1024;
//     let height: u32 = 1024;
//     let mut img_buffer = RgbImage::new(width, height);

//     for x in 0..width {
//         for y in 0..height {
//             img_buffer.put_pixel(x, y, Rgb([255u8, 0, 0]));
//         }
//     }

//     img_buffer.save("test.png").unwrap();
// }
