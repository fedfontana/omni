use std::io::Read;

use clap::Parser;
use format::Format;

mod cli;
mod format;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let (input_format, output_format) = args.validate();

    let mut input: Box<dyn Read> = match args.in_path {
        Some(ref path) => Box::new(std::fs::File::open(path)?),
        None => Box::new(std::io::stdin()),
    };

    let serde_value = to_value(&mut input, input_format)?;

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

fn to_value(
    input: &mut impl Read,
    input_format: Format,
) -> anyhow::Result<Box<dyn erased_serde::Serialize>> {
    match input_format {
        Format::JSON => {
            let value: serde_json::Value = serde_json::from_reader(input)?;
            Ok(Box::new(value))
        }
        Format::YAML => {
            let value: serde_yaml::Value = serde_yaml::from_reader(input)?;
            Ok(Box::new(value))
        }
        Format::TOML => {
            let mut toml_string = String::new();
            input.read_to_string(&mut toml_string)?;
            let value: toml::Value = toml::from_str(&toml_string)?;
            Ok(Box::new(value))
        }
    }
}
