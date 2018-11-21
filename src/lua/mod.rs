use super::model::SceneNode;
use rlua::{
    FromLua, Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Value, Variadic,
};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn test_function(state: &Lua, text: String) -> Result<()> {
    println!("Test function was called with string: {}!", text);
    Ok(())
}

fn print_node(lua: &Lua, node: SceneNode) -> Result<()> {
    println!("{:#?}", node);
    Ok(())
}

fn create_node(lua: &Lua, name: String) -> Result<SceneNode> {
    let node_count: u32 = lua.globals().get("node_count")?;
    lua.globals().set("node_count", node_count + 1);
    Ok(SceneNode::new(node_count, name))
}

impl UserData for SceneNode {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_child", |_, mut node, child| {
            node.add_child(child);
            Ok(())
        });
        //        methods.add_method_mut("set_material", |_, mut node, material| {
        //
        //        })
    }
}

//impl<'lua> FromLua<'lua> for SceneNode {
//    fn from_lua(lua_value: Value, lua: &'lua Lua) -> Result<Self> {
//        let node_count: u32 = lua.globals().get("node_count")?;
//        lua.globals().set("node_count", node_count + 1);
//        Ok(Self::new(node_count, "node".to_string()))
//    }
//}

pub fn test() {
    let lua = Lua::new();

    let globals = lua.globals();
    globals.set("node_count", 0u32);

    let ltf = lua.create_function(test_function).unwrap();
    globals.set("test_function", ltf);

    let cnf = lua.create_function(create_node).unwrap();
    globals.set("create_node", cnf);

    let pnf = lua.create_function(print_node).unwrap();
    globals.set("print_node", pnf);

    let file_name = "test.lua";
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    lua.eval::<_, ()>(&contents, Some(file_name));
}
