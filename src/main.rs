mod scripting;
mod remapper;

use std::{error::Error, fs::File, io::BufReader, path::PathBuf, collections::HashMap};

use clap::Clap;
use colored::Colorize;
use image::ImageBuffer;
use ultraviolet::vec::Vec2;

#[derive(Clap)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
struct Arguments {
    /// 実行する Lua スクリプトのパス
    script: PathBuf,

    /// 出力先画像ファイルのパス
    output: PathBuf,

    /// 出力画像のサイズ (デフォルト: 1024)
    #[clap(short = "s", long = "size")]
    size: Option<usize>,

    /// ベース画像
    #[clap(short = "b", long = "base")]
    base_image: Option<PathBuf>,
}

fn main() {
    // Termination trait 安定化までのワークアラウンド
    let termination = run();
    match termination {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);

            let mut source = e.source();
            while source.is_some() {
                eprintln!("    -> {}",e);
                source = e.source();
            }
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();
    let remappers: HashMap<PathBuf, Remapper> = HashMap::new();

    let file = BufReader::new(File::open(&arguments.script)?);
    let texture_size = arguments.size.unwrap_or(1024);

    let remapper_key = arguments.script.canonicalize()?;
    let remapper = Remapper::new(texture_size, texture_size);
    remappers.insert(remapper_key, remapper);

    let filename = arguments.script
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let texture_size = arguments.size.unwrap_or(1024);
    let (width, height) = (texture_size, texture_size);

    let image = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let dst_uv = Vec2::new(x as f32 / width as f32, y as f32 / height as f32);
        let src_uv = lattice.warp_bilinear(dst_uv);

        image::Rgba([
            (src_uv.x * 255.0) as u8,
            (src_uv.y * 255.0) as u8,
            0u8,
            255u8,
        ])
    });
    image.save("result.png")?;

    Ok(())
}
