//! リマップ関連の操作

pub mod bridge;
pub mod lattice;
pub mod range;

pub use bridge::{Environment, RemapCommand, RemapQueue, SourceLoader};
pub use lattice::{Lattice, LatticeError};
pub use range::{Normalized, Range, Scaled};

use image::{
    imageops,
    imageops::{colorops, FilterType},
    DynamicImage, GenericImageView, ImageBuffer, RgbaImage,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum RemapperError {
    /// 指定されたキーの画像が格納されていない
    ImageNotFound(String),

    /// std::io::Error からのエラー
    IoError(IoError),
}

impl Display for RemapperError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            RemapperError::ImageNotFound(key) => write!(f, "キー {} の画像がありません", key),
            RemapperError::IoError(e) => write!(f, "IO エラー: {}", e),
        }
    }
}

impl Error for RemapperError {}

/// リマップを実行する主体となるオブジェクト
pub struct Remapper {
    base_image: RgbaImage,
    source_images: HashMap<String, DynamicImage>,
}

impl Remapper {
    pub fn new(width: usize, height: usize) -> Remapper {
        let mut remapper = Remapper {
            base_image: ImageBuffer::new(width as u32, height as u32),
            source_images: HashMap::new(),
        };

        remapper.prepare_default_images();
        remapper
    }

    pub fn from_image(base_image: DynamicImage) -> Remapper {
        let mut remapper = Remapper {
            base_image: base_image.into_rgba(),
            source_images: HashMap::new(),
        };

        remapper.prepare_default_images();
        remapper
    }

    pub fn base_image(&self) -> &RgbaImage {
        &self.base_image
    }

    pub fn insert_source(&mut self, key: &str, image: DynamicImage) {
        self.source_images.insert(key.into(), image);
    }

    /// リマップを行う。
    pub fn patch(&mut self, command: RemapCommand) -> Result<(), RemapperError> {
        let scaled = command.range.to_scaled(
            self.base_image.width() as usize,
            self.base_image.height() as usize,
        );

        let image = self
            .source_images
            .get(&command.source_key)
            .ok_or_else(|| RemapperError::ImageNotFound(command.source_key.to_owned()))?;
        let resized_image = image.resize(
            scaled.width as u32,
            scaled.height as u32,
            FilterType::Triangle,
        );

        let mask = match &command.mask_key {
            Some(key) => {
                let mask_image = self
                    .source_images
                    .get(key)
                    .ok_or_else(|| RemapperError::ImageNotFound(command.source_key.to_owned()))?;
                colorops::grayscale(mask_image)
            }
            None => ImageBuffer::from_pixel(
                scaled.width as u32,
                scaled.height as u32,
                image::Luma([255u8]),
            ),
        };

        if let Some(lattice) = command.lattice {
            let patching_image =
                ImageBuffer::from_fn(scaled.width as u32, scaled.height as u32, |x, y| {
                    let inner_u = x as f32 / scaled.width;
                    let inner_v = y as f32 / scaled.height;
                    let warped_uv = lattice.warp_bilinear((inner_u, inner_v).into());
                    let source_x = (warped_uv.x * scaled.width) as u32;
                    let source_y = (warped_uv.y * scaled.height) as u32;

                    let pixel = resized_image.get_pixel(source_x, source_y);
                    let alpha = mask.get_pixel(source_x, source_y)[0] as f32 / 255.0;
                    image::Rgba([
                        (pixel[0] as f32 * alpha) as u8,
                        (pixel[1] as f32 * alpha) as u8,
                        (pixel[2] as f32 * alpha) as u8,
                        (pixel[3] as f32 * alpha) as u8,
                    ])
                });
            imageops::overlay(
                &mut self.base_image,
                &patching_image,
                scaled.x as u32,
                scaled.y as u32,
            );
        } else {
            imageops::overlay(
                &mut self.base_image,
                &resized_image,
                scaled.x as u32,
                scaled.y as u32,
            );
        };

        Ok(())
    }

    fn prepare_default_images(&mut self) {
        let (w, h) = (self.base_image.width(), self.base_image.height());
        self.insert_source(
            "default_uv",
            DynamicImage::ImageRgba8(ImageBuffer::from_fn(w, h, |x, y| {
                let inner_u = x as f32 / w as f32;
                let inner_v = y as f32 / h as f32;

                image::Rgba([(inner_u * 255.0) as u8, (inner_v * 255.0) as u8, 0u8, 255u8])
            })),
        );
    }
}
