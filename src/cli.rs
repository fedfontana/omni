use std::path::PathBuf;

use clap::Parser;

use crate::format::Format;

#[derive(Parser, Debug)]
pub struct Args {
    pub in_path: Option<PathBuf>,

    #[clap(long, short = 'o')]
    pub out_path: Option<PathBuf>,

    /// If specified, the input will be treated as this format (has precedence over the inference
    /// from the file extension)
    #[clap(long)]
    pub in_format: Option<Format>,

    /// If specified, the output will be treated as this format (has precedence over the inference
    /// from the file extension)
    #[clap(long)]
    pub out_format: Option<Format>,
}
