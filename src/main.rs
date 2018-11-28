extern crate image;
extern crate lucis;

use lucis::scene::run_lua_script;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() > 1 {
        &args[1]
    } else {
        "scene/test2.lua"
    };
    run_lua_script(file_name);
}
