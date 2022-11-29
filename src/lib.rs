use mlua::{Lua, lua_module, Result};
use mlua_derive::{FromLua, ToLua};
use std::collections::HashMap;

#[derive(Debug, FromLua, ToLua)]
struct Cookie {}

#[derive(Debug, FromLua, ToLua)]
struct Megastruct {
    is_cool: bool,
    description: String,
    boxed_str: Box<str>,
    byte: u8,
    int: i32,
    uint: u32,
    size: isize,
    float: f32,
    double: f64,
    int_bytes: [u8; 4],
    data: Vec<u8>,
    maybe: Option<Cookie>,
    authors: HashMap<String, String>,
}

#[derive(FromLua)]
struct Config {
    data: Vec<u8>,
}

/// Returns a megastruct
fn get_data(_lua: &Lua, args: (Config,)) -> Result<Megastruct> {
    Ok(Megastruct {
        is_cool: true,
        description: "This is a MEGASTRUCT because IT'S HUGE".to_string(),
        boxed_str: "why not?".into(),
        byte: 0xFF,
        int: i32::MIN,
        uint: u32::MAX,
        size: isize::MAX,
        float: std::f32::consts::PI,
        double: 3.1415926535897932,
        int_bytes: u32::to_le_bytes(0xDEADBEEF),
        data: args.0.data,
        authors: HashMap::from([
            ("All Quiet on the Western Front".into(), "Erich Maria Remarque".into()),
            ("The Rust Programming Language".into(), "Steve Klabnik, Carol Nichols".into()),
            ("Republic".into(), "Plato".into()),
        ]),
        maybe: Some(Cookie {}),
    })
}

fn print_data(_lua: &Lua, args: (Megastruct,)) -> Result<()> {
    println!("{:#?}", args.0);
    Ok(())
}

#[lua_module]
fn moonbind(lua: &Lua) -> mlua::Result<mlua::Table> {
    let exports = lua.create_table()?;
    exports.set("get_data", lua.create_function(get_data)?)?;
    exports.set("print_data", lua.create_function(print_data)?)?;
    Ok(exports)
}