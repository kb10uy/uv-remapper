mod lattice;
mod scripting;

use std::{error::Error, fs::File, io::BufReader, path::Path};

use clap::Clap;
use colored::Colorize;
use image::ImageBuffer;
use ultraviolet::vec::Vec2;

#[derive(Clap)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
struct Arguments {
    /// The script filename to execute.
    script: String,

    /// Output texture size.
    #[clap(short = "s", long = "size")]
    size: Option<usize>,
}

fn main() {
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

    let script_path = Path::new(&arguments.script);
    let file = BufReader::new(File::open(script_path)?);

    let filename = script_path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let lattice = scripting::execute_script(&filename, file)?;

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
