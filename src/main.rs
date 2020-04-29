mod remapper;
mod scripting;

use crate::remapper::Remapper;
use std::{error::Error, fs::File, io::BufReader};

use clap::Clap;
use log::{error, info};
use rlua::prelude::*;

#[derive(Clap)]
#[clap(name = "UV Remapper", version, author, about)]
struct Arguments {
    /// 実行する Lua スクリプトのパス
    script: String,

    /// 出力先画像ファイルのパス
    output: String,

    /// 出力画像のサイズ (デフォルト: 1024)
    #[clap(short = "s", long = "size")]
    size: Option<usize>,

    /// ベース画像のパス。 --size は無視される
    #[clap(short = "b", long = "base")]
    base_image: Option<String>,
}

fn main() {
    pretty_env_logger::init();

    // Termination trait 安定化までのワークアラウンド
    let termination = run();
    match termination {
        Ok(()) => (),
        Err(e) => {
            error!("{}", e);

            let mut source = e.source();
            while let Some(e) = source {
                error!("-> {}", e);
                source = e.source();
            }
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();
    let mut remapper = match arguments.base_image {
        Some(path) => {
            let image = image::open(&path)?;
            info!("Loaded '{}' as base image", path);
            Remapper::from_image(image)
        }
        None => {
            let texture_size = arguments.size.unwrap_or(1024);
            info!(
                "Set empty image ({}x{}) as base image",
                texture_size, texture_size
            );
            Remapper::new(texture_size, texture_size)
        }
    };

    let lua = Lua::new();
    let script_reader = BufReader::new(File::open(&arguments.script)?);
    scripting::prepare(&lua, &arguments.script, script_reader)?;
    info!("Loaded '{}' as manipulation script", arguments.script);

    info!("Loading source images");
    let loader = scripting::call_initialize(&lua)?;
    for (key, filename) in loader.entries() {
        let source = image::open(&filename)?;
        remapper.insert_source(&key, source);
        info!("Loaded '{}' as '{}'", filename, key);
    }

    info!("Executing remapper script");
    let queue = scripting::call_run(&lua)?;

    info!("Remapping image");
    for command in queue.commands() {
        remapper.patch(command)?;
    }

    info!("Saving as '{}'", arguments.output);
    remapper.base_image().save(arguments.output)?;
    Ok(())
}
