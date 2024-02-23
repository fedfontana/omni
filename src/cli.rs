use std::{path::PathBuf, str::FromStr};

use clap::{error::ErrorKind, CommandFactory, Parser};

use crate::format::Format;

#[derive(Parser, Debug)]
pub struct Args {
    /// The input file to convert. If not specified, the input will be read from stdin
    pub in_path: Option<PathBuf>,

    /// The output file to write to. If not specified, the output will be written to stdout
    #[clap(long, short = 'o')]
    pub out_path: Option<PathBuf>,

    /// If specified, the input will be treated as this format (has precedence over the inference
    /// from the file extension). Required when reading from stdin
    #[clap(long, alias = "if")]
    pub in_format: Option<Format>,

    /// If specified, the output will be treated as this format (has precedence over the inference
    /// from the output file extension). Required when writing to stdout
    #[clap(long, alias = "of")]
    pub out_format: Option<Format>,
}

impl Args {
    pub fn validate(&self) -> (Format, Format) {
        let mut cmd = Self::command();
        if self.in_path.is_none() && self.in_format.is_none() {
            cmd.error(
                ErrorKind::MissingRequiredArgument,
                "must specify either an input file or an input format",
            )
            .exit();
        }

        if self.out_path.is_none() && self.out_format.is_none() {
            cmd.error(
                ErrorKind::MissingRequiredArgument,
                "must specify either an output file or an output format",
            )
            .exit();
        }

        if let Some(ref path) = self.in_path {
            if !path.exists() || !path.is_file() {
                cmd.error(
                    ErrorKind::ValueValidation,
                    "input file does not exist or is not a file",
                )
                .exit();
            }
        }

        let input_format = match get_format(self.in_path.as_ref(), self.in_format.as_ref()) {
            Some(format) => format,
            None => cmd
                .error(
                    ErrorKind::ValueValidation,
                    "couldn't determine input format from file extension",
                )
                .exit(),
        };

        let output_format = match get_format(self.out_path.as_ref(), self.out_format.as_ref()) {
            Some(format) => format,
            None => cmd
                .error(
                    ErrorKind::ValueValidation,
                    "couldn't determine output format from file extension",
                )
                .exit(),
        };

        (input_format, output_format)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }
}
