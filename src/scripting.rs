use crate::remapper::{Environment, RemapQueue, SourceLoader};
use std::{error::Error, io::prelude::*};

use rlua::prelude::*;

/// Lua スクリプトを読み込んで exec() まで実行する。
pub fn prepare(lua: &Lua, name: &str, mut reader: impl Read) -> Result<(), Box<dyn Error>> {
    let mut script = String::with_capacity(8192);
    reader.read_to_string(&mut script)?;

    // 登録
    let environment = Environment::new();
    lua.context(|ctx| environment.register_with_lua(ctx))?;

    // スクリプト読み込み
    lua.context(|ctx| {
        ctx.load(&script)
            .set_name(name)
            .and_then(|chunk| chunk.exec())
    })?;

    Ok(())
}

pub fn call_initialize(lua: &Lua) -> Result<SourceLoader, LuaError> {
    let result = lua.context(|ctx| {
        let globals = ctx.globals();
        let func: LuaFunction = globals.get("Initialize")?;

        let loader = SourceLoader::new();
        func.call(loader)
    });

    Ok(result?)
}

pub fn call_run(lua: &Lua) -> Result<RemapQueue, LuaError> {
    let result = lua.context(|ctx| {
        let globals = ctx.globals();
        let func: LuaFunction = globals.get("Run")?;

        let queue = RemapQueue::new();
        func.call(queue)
    });

    Ok(result?)
}
