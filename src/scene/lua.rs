use geometry::Primitive;
use nalgebra::{Point3, Vector3};
use rlua::{Function, Lua, Result, Table, UserData, UserDataMethods};
use scene::{Color, Material, SceneNode};
use std::fs::File;
use std::io::prelude::*;
use Raytracer;

fn print_node(lua: &Lua, node: SceneNode) -> Result<()> {
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

fn render(
    _: &Lua,
    (node, file_name, width, height, eye, view, up, fov): (
        SceneNode,
        String,
        u32,
        u32,
        Table,
        Table,
        Table,
        f32,
    ),
) -> Result<()> {
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
        ambient: Vector3::new(0.8, 0.8, 0.8),
    };
    raytracer.render(file_name.as_ref(), width, height, Default::default());
    println!("Rendering complete!");
    Ok(())
}

impl UserData for Material {}

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
        // Create a new material
        ("material", lua.create_function(create_material).unwrap()),
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
