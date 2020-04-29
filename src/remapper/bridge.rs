//! Rust <=> Lua 相互運用のブリッジ

use super::{
    lattice::Lattice,
    range::{Normalized, Range},
};
use std::collections::VecDeque;

use rlua::prelude::*;

/// Lua スクリプトに `UVR` として公開されるオブジェクト
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {}

impl Environment {
    /// ムーブして `LuaContext` に登録する。
    pub fn register_with_lua(self, lua: LuaContext) -> Result<(), LuaError> {
        lua.globals().set("UVR", self)?;
        Ok(())
    }

    fn create_lattice(width: usize, height: usize) -> Result<Lattice, LuaError> {
        Ok(Lattice::new(width, height))
    }

    fn create_range(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<Range<Normalized>, LuaError> {
        Ok(Range::new(x, y, width, height))
    }
}

impl LuaUserData for Environment {
    fn add_methods<'lua, T: LuaUserDataMethods<'lua, Environment>>(methods: &mut T) {
        methods.add_function("create_lattice", |_, (w, h)| {
            Environment::create_lattice(w, h)
        });
        methods.add_function("create_range", |_, (x, y, width, height)| {
            Environment::create_range(x, y, width, height)
        });
    }
}

/// Lua スクリプトから予約されたパッチの情報
pub struct RemapCommand {
    pub source_key: String,
    pub range: Range<Normalized>,
    pub mask_key: Option<String>,
    pub lattice: Option<Lattice>,
}

/// Lua スクリプトの Run 関数に渡されるコマンドキュー
pub struct RemapQueue {
    queue: VecDeque<RemapCommand>,
}

impl RemapQueue {
    pub fn new() -> RemapQueue {
        RemapQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn commands(self) -> impl Iterator<Item = RemapCommand> {
        self.queue.into_iter()
    }
}

impl LuaUserData for RemapQueue {
    fn add_methods<'lua, T: LuaUserDataMethods<'lua, Self>>(methods: &mut T) {
        // queue.push(source, range, mask, lattice)
        // キューに登録する
        methods.add_method_mut("push", |ctx, this, (source, range, mask, lattice)| {
            this.queue.push_back(RemapCommand {
                source_key: source,
                range,
                mask_key: mask,
                lattice,
            });
            Ok(())
        });
    }
}
