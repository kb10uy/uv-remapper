use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    sync::Arc,
};

use ultraviolet::{Vec2, Mat2};
use rlua::prelude::*;

/// `Lattice` 関連のエラー
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LatticeError {
    /// 生成元 Lua テーブルに凹凸が存在する
    NonRectangular,
}

impl Display for LatticeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LatticeError::NonRectangular => write!(f, "The source was not recutangular form."),
        }
    }
}

impl Error for LatticeError {}

/// 画像を UV 変形するラティス
pub struct Lattice {
    /// X 方向の分割ブロック数(ラティス要素数 - 1)
    x_blocks: usize,

    /// Y 方向の分割ブロック数(ラティス要素数 - 1)
    y_blocks: usize,

    /// UV 値
    uvs: Box<[Vec2]>,
}

impl Lattice {
    /// f32[2][][] からラティスを生成する。
    pub fn new(width: usize, height: usize) -> Lattice {
        let mut uvs = Vec::with_capacity((width + 1) * (height + 1));

        for y in 0..=height {
            for x in 0..=width {
                uvs.push((x as f32 / width as f32, y as f32 / height as f32).into());
            }
        }

        Lattice {
            x_blocks: width,
            y_blocks: height,
            uvs: uvs.into_boxed_slice(),
        }
    }

    /// 指定された UV 座標の変形後の UV 座標を計算する。
    /// バイリニア補完。
    pub fn warp_bilinear(&self, uv: Vec2) -> Vec2 {
        // [0, x/y_blocks] に持ってくる
        let ranged_u = if uv.x == 1.0 {
            uv.x
        } else {
            uv.x - uv.x.floor()
        } * self.x_blocks as f32;
        let ranged_v = if uv.y == 1.0 {
            uv.y
        } else {
            uv.y - uv.y.floor()
        } * self.y_blocks as f32;

        let left_index = ranged_u as usize;
        let top_index = ranged_v as usize;
        let normalized: Vec2 = (ranged_u.fract(), ranged_v.fract()).into();

        // 周囲のラティス
        let llt = self.lattice_at(left_index, top_index);
        let lrt = self.lattice_at(left_index + 1, top_index);
        let llb = self.lattice_at(left_index, top_index + 1);
        let lrb = self.lattice_at(left_index + 1, top_index + 1);

        // cf. https://en.wikipedia.org/wiki/Bilinear_interpolation
        let u_matrix: Mat2 = [llt.x, lrt.x, llb.x, lrb.x].into();
        let v_matrix: Mat2 = [llt.y, lrt.y, llb.y, lrb.y].into();
        let x_ratio: Vec2 = (1.0 - normalized.x, normalized.x).into();
        let y_ratio: Vec2 = (1.0 - normalized.y, normalized.y).into();
        let warped_u = x_ratio.dot(u_matrix * y_ratio);
        let warped_v = x_ratio.dot(v_matrix * y_ratio);

        (warped_u, warped_v).into()
    }

    /// 指定インデックスのラティスの値を取得する。
    fn lattice_at(&self, x: usize, y: usize) -> Vec2 {
        self.uvs[y * (self.x_blocks + 1) + x]
    }
}

impl<'lua> ToLua<'lua> for &Lattice {
    fn to_lua(self, ctx: LuaContext) -> Result<LuaValue, LuaError> {
        let lattice_table = ctx.create_table()?;
        for (y, row) in self.uvs.chunks(self.x_blocks + 1).enumerate() {
            let row_table = ctx.create_table()?;
            for (x, uv) in row.iter().enumerate() {
                row_table.set(x + 1, vec![uv.x, uv.y])?;
            }
            lattice_table.set(y + 1, row_table)?;
        }

        Ok(LuaValue::Table(lattice_table))
    }
}

impl<'lua> FromLua<'lua> for Lattice {
    fn from_lua(value: LuaValue, ctx: LuaContext) -> Result<Lattice, LuaError> {
        let lattice_table = match value {
            LuaValue::Table(t) => t,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "Non-table",
                    to: "Lattice",
                    message: Some("Non-table value cannot be converted into Lattice".into()),
                })
            }
        };

        let height = lattice_table.len()?;
        if height <= 1 {
            return Err(LuaError::ExternalError(Arc::new(
                LatticeError::NonRectangular,
            )));
        }

        let width = lattice_table.get::<_, LuaTable>(1)?.len()?;
        if width <= 1 {
            return Err(LuaError::ExternalError(Arc::new(
                LatticeError::NonRectangular,
            )));
        }

        let mut uvs = Vec::with_capacity((width * height) as usize);
        for y in 1..=height {
            let row_table: LuaTable = lattice_table.get(y)?;
            if row_table.len()? != width {
                return Err(LuaError::ExternalError(Arc::new(
                    LatticeError::NonRectangular,
                )));
            }
            for x in 1..=width {
                let uv: LuaTable = row_table.get(x)?;
                if uv.len()? != 2 {
                    return Err(LuaError::ExternalError(Arc::new(
                        LatticeError::NonRectangular,
                    )));
                }
                uvs.push((uv.get(1)?, uv.get(2)?).into());
            }
        }

        Ok(Lattice {
            uvs: uvs.into_boxed_slice(),
            x_blocks: (width - 1) as usize,
            y_blocks: (height - 1) as usize,
        })
    }
}

impl LuaUserData for Lattice {}
