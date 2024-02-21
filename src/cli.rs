use std::path::PathBuf;

use clap::Parser;

use crate::format::Format;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, short = 'i')]
    pub in_path: Option<PathBuf>,

    #[clap(long, short = 'o')]
    pub out_path: Option<PathBuf>,

    #[clap(long)]
    pub in_format: Option<Format>,

    #[clap(long)]
    pub out_format: Option<Format>,
}
