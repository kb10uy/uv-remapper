mod remapper;
mod scripting;

use crate::remapper::Remapper;
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use clap::Clap;
use colored::Colorize;

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
            while let Some(e) = source {
                eprintln!("    -> {}", e);
                source = e.source();
            }
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();
    let texture_size = arguments.size.unwrap_or(1024);
    let file = BufReader::new(File::open(&arguments.script)?);

    let filename = arguments
        .script
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let mut remapper = Remapper::new(texture_size, texture_size);
    let queue = scripting::execute_script(filename, file)?;

    for command in queue.commands() {
        remapper.patch(command);
    }

    remapper.base_image().save(arguments.output)?;

    Ok(())
}
