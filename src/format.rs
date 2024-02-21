use std::fmt::Display;

use anyhow::{anyhow, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    JSON,
    YAML,
    TOML,
}

impl std::str::FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::JSON),
            "yaml" | "yml" => Ok(Format::YAML),
            "toml" => Ok(Format::TOML),
            _ => Err(anyhow!("Unknown format: {}", s)),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::JSON => write!(f, "json"),
            Format::YAML => write!(f, "yaml"),
            Format::TOML => write!(f, "toml"),
        }
    }
}
