extern crate image;
extern crate lucis;

use std::env;
use lucis::scene::run_lua_script;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name = "scene/test2.lua";
    if args.len() > 1 {
        file_name = &args[1];
    }
    run_lua_script(file_name);
}
