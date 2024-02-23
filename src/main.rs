use std::io::Read;

use clap::Parser;
use format::Format;

mod cli;
mod format;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let (input_format, output_format) = args.validate();

    let input = match args.in_path {
        Some(ref path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let serde_value = to_value(&input, input_format)?;

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
