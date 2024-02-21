use std::{any::Any, collections::HashMap, io::Read};

use clap::Parser;
use format::Format;

mod cli;
mod format;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    println!("{:?}", args);

    // TODO: can this be done directly in clap?
    if args.in_path.is_none() && args.in_format.is_none() {
        eprintln!("Error: either --in-path or --in-format must be specified");
        std::process::exit(1);
    }
    if args.out_path.is_none() && args.out_format.is_none() {
        eprintln!("Error: either --out-path or --out-format must be specified");
        std::process::exit(1);
    }

    // TODO: check that files exists and are readable/writable and that we can infer the foprmat
    // from the file extension

    let input = match args.in_path {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    //TODO: should use the real format
    let serde_value = to_value(&input, args.in_format.unwrap())?;

    let output = match args.out_format {
        Some(Format::JSON) => serde_json::to_string_pretty(&serde_value)?,
        Some(Format::YAML) => serde_yaml::to_string(&serde_value)?,
        Some(Format::TOML) => toml::to_string(&serde_value)?,
        None => {
            eprintln!("Error: --out-format must be specified");
            std::process::exit(1);
        }
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
