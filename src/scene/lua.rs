use geometry::{Primitive,Mesh};
use nalgebra::{Point3, Vector3};
use rlua::{Function, Lua, Result, Table, UserData, UserDataMethods};
use scene::{Color, Light, Material, SceneNode};
use std::fs::File;
use std::io::prelude::*;
use Raytracer;

fn print_node(_: &Lua, node: SceneNode) -> Result<()> {
    println!("{:#?}", node);
    Ok(())
}

fn create_node(lua: &Lua, name: String) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1).unwrap();
    println!("Creating new node \'{}\'", name);
    Ok(SceneNode::new(node_count, name))
}

fn create_sphere(lua: &Lua, name: String) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1).unwrap();
    println!("Creating new sphere \'{}\'", name);
    let mut node = SceneNode::new(node_count, name);
    node.primitive = Primitive::Sphere;
    Ok(node)
}

fn create_cylinder(lua: &Lua, name: String) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1).unwrap();
    println!("Creating new cylinder \'{}\'", name);
    let mut node = SceneNode::new(node_count, name);
    node.primitive = Primitive::Cylinder;
    Ok(node)
}

fn create_cube(lua: &Lua, name: String) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1).unwrap();
    println!("Creating new cube \'{}\'", name);
    let mut node = SceneNode::new(node_count, name);
    node.primitive = Primitive::Cube;
    Ok(node)
}

fn create_mesh(lua: &Lua, (name, file_name): (String, String)) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1).unwrap();
    println!("Creating new mesh({}) \'{}\'", file_name, name);
    let mut node = SceneNode::new(node_count, name);
    // TODO: Better error handling
    node.primitive = Primitive::Mesh(Mesh::from_file(file_name.as_ref()).unwrap());
    Ok(node)
}

fn create_material(_: &Lua, (d, s, p): (Table, Table, f32)) -> Result<Material> {
    let dr: f32 = d.raw_get(1).unwrap();
    let dg: f32 = d.raw_get(2).unwrap();
    let db: f32 = d.raw_get(3).unwrap();
    let sr: f32 = s.raw_get(1).unwrap();
    let sg: f32 = s.raw_get(2).unwrap();
    let sb: f32 = s.raw_get(3).unwrap();

    Ok(Material::phong(
        Color::new(dr, dg, db),
        Color::new(sr, sg, sb),
        p,
    ))
}

fn create_light(_: &Lua, (p, c, a): (Table, Table, Table)) -> Result<Light> {
    let px: f32 = p.raw_get(1).unwrap();
    let py: f32 = p.raw_get(2).unwrap();
    let pz: f32 = p.raw_get(3).unwrap();
    let cr: f32 = c.raw_get(1).unwrap();
    let cg: f32 = c.raw_get(2).unwrap();
    let cb: f32 = c.raw_get(3).unwrap();
    let a1: f32 = a.raw_get(1).unwrap();
    let a2: f32 = a.raw_get(2).unwrap();
    let a3: f32 = a.raw_get(3).unwrap();
    Ok(Light::new(
        Color::new(cr, cg, cb),
        Point3::new(px, py, pz),
        [a1, a2, a3],
        1.0,
        64
    ))
}

fn render(
    _: &Lua,
    (node, file_name, width, height, eye, view, up, fov, lights): (
        SceneNode,
        String,
        u32,
        u32,
        Table,
        Table,
        Table,
        f32,
        Table,
    ),
) -> Result<()> {
    let mut lights_vec: Vec<Light> = Vec::new();
    for i in 1..=lights.raw_len() {
        lights_vec.push(lights.raw_get(i).unwrap());
    }
    let raytracer = Raytracer {
        root_node: node,
        eye: Point3::new(
            eye.raw_get(1).unwrap(),
            eye.raw_get(2).unwrap(),
            eye.raw_get(3).unwrap(),
        ),
        view: Point3::new(
            view.raw_get(1).unwrap(),
            view.raw_get(2).unwrap(),
            view.raw_get(3).unwrap(),
        ),
        up: Vector3::new(
            up.raw_get(1).unwrap(),
            up.raw_get(2).unwrap(),
            up.raw_get(3).unwrap(),
        ),
        fov_y: fov,
        ambient: Color::new(0.2, 0.2, 0.2),
        lights: lights_vec,
    };
    raytracer.render(file_name.as_ref(), width, height, Default::default());
    println!("Rendering complete!");
    Ok(())
}

impl UserData for Material {}

impl UserData for Light {}

impl UserData for SceneNode {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_child", |_, node, child| {
            node.add_child(child);
            Ok(())
        });
        methods.add_method_mut("set_material", |_, node, material| {
            node.material = material;
            Ok(())
        });
        methods.add_method_mut("scale", |_, node, (x, y, z): (f32, f32, f32)| {
            node.scale(x, y, z);
            Ok(())
        });
        methods.add_method_mut("translate", |_, node, (x, y, z): (f32, f32, f32)| {
            node.translate(x, y, z);
            Ok(())
        });
        methods.add_method_mut("rotate", |_, node, (axis, angle): (String, f32)| {
            node.rotate(axis.as_ref(), angle);
            Ok(())
        });
    }
}

pub fn run_lua_script(file_name: &str) {
    let lua = Lua::new();

    let core_functions: Vec<(&str, Function)> = vec![
        // Create a node
        ("node", lua.create_function(create_node).unwrap()),
        // Create a sphere node
        ("sphere", lua.create_function(create_sphere).unwrap()),
        // Create a cylinder node
        ("cylinder", lua.create_function(create_cylinder).unwrap()),
        // Create a cube node
        ("cube", lua.create_function(create_cube).unwrap()),
        // Create a mesh node
        ("mesh", lua.create_function(create_mesh).unwrap()),
        // Create a new material
        ("material", lua.create_function(create_material).unwrap()),
        // Create a new light
        ("light", lua.create_function(create_light).unwrap()),
        // Print the details of a node
        ("print", lua.create_function(print_node).unwrap()),
        // Render a scene
        ("render", lua.create_function(render).unwrap()),
    ];

    let f_table = lua.create_table_from(core_functions).unwrap();

    let globals = lua.globals();
    // Track node count as we create new nodes
    globals.set("node_count", 0u32).unwrap();
    globals.set("rt", f_table).unwrap();

    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    lua.eval::<_, ()>(&contents, Some(file_name)).unwrap();
}
