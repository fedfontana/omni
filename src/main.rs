use std::{io::Read, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::Parser;
use format::Format;

mod cli;
mod format;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    // TODO: can this be done directly in clap?
    if args.in_path.is_none() && args.in_format.is_none() {
        eprintln!("Error: either --in-path or --in-format must be specified");
        std::process::exit(1);
    }
    if args.out_path.is_none() && args.out_format.is_none() {
        eprintln!("Error: either --out-path or --out-format must be specified");
        std::process::exit(1);
    }

    if let Some(ref path) = args.in_path {
        if !path.exists() || !path.is_file() {
            eprintln!("Error: input file does not exist or is not a file");

            std::process::exit(1);
        }
    }

    let input_format = get_format(args.in_path.as_ref(), args.in_format.as_ref())
        .context("Couldn't determine input format")?;

    let input = match args.in_path {
        Some(ref path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let serde_value = to_value(&input, input_format)?;

    let output_format = get_format(args.out_path.as_ref(), args.out_format.as_ref())
        .context("Couldn't determine output format")?;

    let output = match output_format {
        Format::JSON => serde_json::to_string_pretty(&serde_value)?,
        Format::YAML => serde_yaml::to_string(&serde_value)?,
        Format::TOML => toml::to_string(&serde_value)?,
    };

    match args.out_path {
        Some(path) => std::fs::write(path, output)?,
        None => println!("{}", output),
    }

    Ok(())
}

fn to_value(input: &str, input_format: Format) -> anyhow::Result<Box<dyn erased_serde::Serialize>> {
    match input_format {
        Format::JSON => {
            let value: serde_json::Value = serde_json::from_str(input)?;
            Ok(Box::new(value))
        }
        Format::YAML => {
            let value: serde_yaml::Value = serde_yaml::from_str(input)?;
            Ok(Box::new(value))
        }
        Format::TOML => {
            let value: toml::Value = toml::from_str(input)?;
            Ok(Box::new(value))
        }
    }
}

fn get_format(path: Option<&PathBuf>, format: Option<&Format>) -> Option<Format> {
    if let Some(format) = format {
        Some(*format)
    } else {
        path.and_then(|path| {
            path.extension()
                .and_then(|ext| Format::from_str(ext.to_str()?).ok())
        })
    }
}
