use std::{io::Read, str::FromStr};

use anyhow::bail;
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

    let input_format = get_input_format(&args)?;

    let input = match args.in_path {
        Some(ref path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let serde_value = to_value(&input, input_format)?;

    let output_format = get_output_format(&args)?;

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

//TODO: decide what should have precedence if the format can be inferred

fn get_input_format(args: &cli::Args) -> anyhow::Result<Format> {
    match args.in_path {
        Some(ref path) => {
            if !path.exists() || !path.is_file() {
                bail!("Error: input file does not exist or is not a file");
            }

            let extension = path
                .extension()
                .ok_or_else(|| anyhow::anyhow!("Error: input file has no extension"))?;

            extension
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Error: input file extension is not valid UTF-8"))
                .map(Format::from_str)?
        }
        None => args.in_format.ok_or_else(|| {
            anyhow::anyhow!("Error: --in-format must be specified if --in-path is not specified")
        }),
    }
}

fn get_output_format(args: &cli::Args) -> anyhow::Result<Format> {
    match args.out_path {
        Some(ref path) => {
            if !path.exists() || !path.is_file() {
                bail!("Error: output file does not exist or is not a file");
            }

            let extension = path
                .extension()
                .ok_or_else(|| anyhow::anyhow!("Error: output file has no extension"))?;

            extension
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Error: output file extension is not valid UTF-8"))
                .map(Format::from_str)?
        }
        None => args.out_format.ok_or_else(|| {
            anyhow::anyhow!(
                "Error: --output-format must be specified if --output-path is not specified"
            )
        }),
    }
}
