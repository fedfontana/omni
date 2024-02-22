use clap::ValueEnum;
use clap::{self, CommandFactory};
use clap_complete::{generate_to, Shell};
use std::env;

#[path = "src/format.rs"]
mod format;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let mut cmd = cli::Args::command();

    let out_dir =
        std::path::PathBuf::from(env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(&out_dir.join("omni.1"), buffer)?;

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "omni", &out_dir)?;
    }

    Ok(())
}
