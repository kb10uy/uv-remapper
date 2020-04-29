use crate::remapper::{Environment, RemapQueue};
use std::{error::Error, io::prelude::*};

use rlua::prelude::*;

pub fn execute_script(name: &str, mut reader: impl Read) -> Result<RemapQueue, Box<dyn Error>> {
    let lua = Lua::new();
    let environment = Environment::new();

    let mut script = String::with_capacity(8192);
    reader.read_to_string(&mut script)?;

    // 登録
    lua.context(|ctx| environment.register_with_lua(ctx))?;

    // スクリプト読み込み
    lua.context(|ctx| {
        ctx.load(&script)
            .set_name(name)
            .and_then(|chunk| chunk.exec())
    })?;

    // Initialize 呼び出し
    // TODO: まず Loader を書け

    // Run 呼び出し
    let result = lua.context(|ctx| {
        let globals = ctx.globals();
        let func: LuaFunction = globals.get("Run")?;

        let queue = RemapQueue::new();
        func.call(queue)
    });

    Ok(result?)
}
