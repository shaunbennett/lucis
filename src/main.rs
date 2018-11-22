extern crate image;
extern crate lucis;

use lucis::scene::run_lua_script;

fn main() {
    run_lua_script("test.lua");
}
