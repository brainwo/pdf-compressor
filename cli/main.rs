use std::{fs, io, path::PathBuf, str::FromStr};

use clap::Parser;
use pdf_compressor::*;

#[derive(Debug, Parser)]
#[command(name = "pdf-compressor", version)]
#[command(about = "A CLI tool to compress PDF")]
struct Cli {
    #[arg(required = true)]
    path: PathBuf,
    #[arg(
        short = 'q',
        long,
        help = "Compression quality for JPEG images, the higher the better"
    )]
    image_quality: Option<u8>,
    #[arg(
        short = 'o',
        long,
        help = "Output default to \"compressed-<FILE_NAME>.pdf\""
    )]
    output: Option<PathBuf>,
    #[arg(
        short = 's',
        long,
        default_value_t = false,
        help = "Terminal will not output anything"
    )]
    silent: bool,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    let binary = fs::read(&cli.path)?;
    let quality = cli.image_quality.unwrap_or(30);
    let output_path = cli.output.unwrap_or_else(|| {
        PathBuf::from_str(&format!(
            "./compressed-{}",
            cli.path.file_name().unwrap().to_str().unwrap()
        ))
        .unwrap()
    });

    let mut pdf = compress_pdf(&binary, quality, !cli.silent);
    pdf.save(&output_path)?;

    if !cli.silent {
        init_progress_bar(10);
        print_progress_bar_info(
            "Success",
            &format!("PDF file saved to {:?}", output_path),
            Color::Green,
            Style::Bold,
        );
    }

    Ok(())
}
