use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use ultraviolet::{Mat2, Vec2};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LatticeError {
    NonRectangular,
}

impl Display for LatticeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LatticeError::NonRectangular => write!(f, "The source was not recutangular form."),
        }
    }
}

impl Error for LatticeError {}

pub struct Lattice {
    x_blocks: usize,
    y_blocks: usize,
    uvs: Box<[Vec2]>,
}

impl Lattice {
    pub fn new<V: AsRef<[f32]>, C: AsRef<[V]>, R: AsRef<[C]>>(
        source: R,
    ) -> Result<Lattice, LatticeError> {
        let rows = source.as_ref();
        if rows.len() <= 1 {
            return Err(LatticeError::NonRectangular);
        }

        let first_row = rows[0].as_ref();
        if first_row.len() <= 1 {
            return Err(LatticeError::NonRectangular);
        }

        let (width, height) = (first_row.len(), rows.len());
        let mut uvs = Vec::with_capacity(width * height);
        for row in rows {
            let column = row.as_ref();
            if column.len() != width {
                return Err(LatticeError::NonRectangular);
            }
            for value in column {
                let uv = value.as_ref();
                if uv.len() != 2 {
                    return Err(LatticeError::NonRectangular);
                }
                uvs.push((uv[0], uv[1]).into());
            }
        }

        Ok(Lattice {
            uvs: uvs.into_boxed_slice(),
            x_blocks: width - 1,
            y_blocks: height - 1,
        })
    }

    fn lattice_at(&self, x: usize, y: usize) -> Vec2 {
        self.uvs[y * (self.x_blocks + 1) + x]
    }

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

        let llt = self.lattice_at(left_index, top_index);
        let lrt = self.lattice_at(left_index + 1, top_index);
        let llb = self.lattice_at(left_index, top_index + 1);
        let lrb = self.lattice_at(left_index + 1, top_index + 1);

        let u_matrix: Mat2 = [llt.x, lrt.x, llb.x, lrb.x].into();
        let v_matrix: Mat2 = [llt.y, lrt.y, llb.y, lrb.y].into();
        let x_ratio: Vec2 = (1.0 - normalized.x, normalized.x).into();
        let y_ratio: Vec2 = (1.0 - normalized.y, normalized.y).into();

        let warped_u = x_ratio.dot(u_matrix * y_ratio);
        let warped_v = x_ratio.dot(v_matrix * y_ratio);

        (warped_u, warped_v).into()
    }
}
