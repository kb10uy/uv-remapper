use std::marker::PhantomData;

use rlua::prelude::*;

/// スケーリング用のタグのトレイト
pub trait Scaling {}

/// 正規化された `Range` のタグ
#[derive(Debug, Clone)]
pub enum Normalized {}
impl Scaling for Normalized {}

/// スケーリングされた `Range` のタグ
#[derive(Debug, Clone)]
pub enum Scaled {}
impl Scaling for Scaled {}

/// 画像内の範囲
#[derive(Debug, Clone, PartialEq)]
pub struct Range<T: Scaling> {
    /// 左上の正規化 X 座標
    pub x: f32,

    /// 左上の正規化 Y 座標
    pub y: f32,

    /// 正規化幅
    pub width: f32,

    /// 正規化高さ
    pub height: f32,

    /// スケーリングの状態の幽霊型
    tag: PhantomData<fn() -> T>,
}

impl<T: Scaling> Range<T> {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Range<T> {
        Range {
            x,
            y,
            width,
            height,
            tag: PhantomData,
        }
    }
}

impl Range<Normalized> {
    /// 正規化状態に画像サイズを適用する。
    pub fn to_scaled(&self, width: usize, height: usize) -> Range<Scaled> {
        Range {
            x: width as f32 * self.x,
            y: height as f32 * self.y,
            width: width as f32 * self.width,
            height: height as f32 * self.height,
            tag: PhantomData,
        }
    }
}

impl<'lua> ToLua<'lua> for &Range<Normalized> {
    fn to_lua(self, ctx: LuaContext) -> Result<LuaValue, LuaError> {
        let table = ctx.create_table()?;
        table.set("x", self.x)?;
        table.set("y", self.y)?;
        table.set("width", self.width)?;
        table.set("height", self.height)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Range<Normalized> {
    fn from_lua(value: LuaValue, _: LuaContext) -> Result<Range<Normalized>, LuaError> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "Non-table",
                    to: "Range",
                    message: Some("非テーブル値は Range に変換できません".into()),
                })
            }
        };

        Ok(Range::<Normalized> {
            x: table.get("x")?,
            y: table.get("y")?,
            width: table.get("width")?,
            height: table.get("height")?,
            tag: PhantomData,
        })
    }
}
