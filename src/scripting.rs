use crate::remapper::{Lattice, Range, Normalized, Remapper};
use std::{collections::{VecDeque, HashMap}, error::Error, io::prelude::*};

use image::{DynamicImage, ImageBuffer, RgbaImage};
use rlua::prelude::*;




/// uv-remapper 内で Lua と関係するデータのラッパー
pub struct Environment {
    base_image: RgbaImage,
    source_images: HashMap<String, DynamicImage>,
}

impl Environment {

    fn patch(
        &mut self,
        source_key: String,
        range: Range<Normalized>,
        mask_key: Option<String>,
        lattice: Lattice,
    ) {
        let scaled = range.to_scaled(
            self.base_image.width() as usize,
            self.base_image.height() as usize,
        );

        // TODO: ここに image を挿入
        let image = ImageBuffer::from_fn(scaled.width as u32, scaled.height as u32, |x, y| {
            let inner_u = x as f32 / scaled.width;
            let inner_v = y as f32 / scaled.height;

            image::Rgba([(inner_u * 255.0) as u8, (inner_v * 255.0) as u8, 0u8, 255u8])
        });

        let patching_image =
            ImageBuffer::from_fn(scaled.width as u32, scaled.height as u32, |x, y| {
                let inner_u = x as f32 / scaled.width;
                let inner_v = y as f32 / scaled.height;
                let warped_uv = lattice.warp_bilinear((inner_u, inner_v).into());
                let source_pixel = image.get_pixel(
                    (warped_uv.x * scaled.width) as u32,
                    (warped_uv.y * scaled.height) as u32,
                );

                // TODO: ここに mask を挿入
                source_pixel.clone()
            });

        image::imageops::replace(&mut self.base_image, &patching_image, 0, 0);
    }
}

pub fn execute_script(
    remapper: Remapper,
    name: &str,
    mut reader: impl Read,
) -> Result<Remapper, Box<dyn Error>> {
    let lua = Lua::new();

    let mut script = String::with_capacity(8192);
    reader.read_to_string(&mut script)?;

    lua.context(|ctx| {
        ctx.load(&script).set_name(name)?.exec()?;

        let globals = ctx.globals();
        globals.set("UVR", remapper)?;
        let patch_func: LuaFunction = globals.get("Patch")?;
        patch_func.call(())?;

        globals.get("UVR")
    })
}
