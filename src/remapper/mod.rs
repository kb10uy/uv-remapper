//! リマップ関連の操作

pub mod bridge;
pub mod lattice;
pub mod range;

pub use bridge::{Environment, RemapCommand, RemapQueue};
pub use lattice::{Lattice, LatticeError};
pub use range::{Normalized, Range, Scaled};

use image::{DynamicImage, ImageBuffer, RgbaImage};
use std::collections::HashMap;

/// リマップを実行する主体となるオブジェクト
pub struct Remapper {
    base_image: RgbaImage,
    source_images: HashMap<String, DynamicImage>,
}

impl Remapper {
    pub fn new(width: usize, height: usize) -> Remapper {
        Remapper {
            base_image: ImageBuffer::new(width as u32, height as u32),
            source_images: HashMap::new(),
        }
    }

    pub fn base_image(&self) -> &RgbaImage {
        &self.base_image
    }

    /// リマップを行う。
    pub fn patch(&mut self, command: RemapCommand) {
        let scaled = command.range.to_scaled(
            self.base_image.width() as usize,
            self.base_image.height() as usize,
        );

        // TODO: ここに image を挿入
        let image = ImageBuffer::from_fn(scaled.width as u32, scaled.height as u32, |x, y| {
            let inner_u = x as f32 / scaled.width;
            let inner_v = y as f32 / scaled.height;

            image::Rgba([(inner_u * 255.0) as u8, (inner_v * 255.0) as u8, 0u8, 255u8])
        });

        if let Some(lattice) = command.lattice {
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
            image::imageops::replace(
                &mut self.base_image,
                &patching_image,
                scaled.x as u32,
                scaled.y as u32,
            );
        } else {
            image::imageops::replace(
                &mut self.base_image,
                &image,
                scaled.x as u32,
                scaled.y as u32,
            );
        };
    }
}
