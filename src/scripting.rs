use crate::lattice::Lattice;
use std::{error::Error, io::prelude::*};

use rlua::{Lua, Error as LuaError};

type RawUvMap = Vec<Vec<Vec<f32>>>;

pub fn execute_script(name: &str, mut reader: impl Read) -> Result<Lattice, Box<dyn Error>> {
    let lua = Lua::new();

    let mut script = String::with_capacity(8192);
    reader.read_to_string(&mut script)?;

    let raw_map: Result<_, LuaError> = lua.context(|ctx| {
        ctx.load(&script).set_name(name)?.exec()?;

        let globals = ctx.globals();
        let raw_map: RawUvMap = globals.get("Lattice")?;

        Ok(raw_map)
    });

    let lattice = Lattice::new(raw_map?)?;
    Ok(lattice)
}
