# omni-cli

Exposes a single `omni` command to convert between commonly used configuration formats.

```sh
Usage: omni [OPTIONS] [IN_PATH]

Arguments:
  [IN_PATH]  The input file to convert. If not specified, the input will be read from stdin

Options:
  -o, --out-path <OUT_PATH>      The output file to write to. If not specified, the output will be written to stdout
      --in-format <IN_FORMAT>    If specified, the input will be treated as this format (has precedence over the inference from the file extension). Required when reading from stdin
      --out-format <OUT_FORMAT>  If specified, the output will be treated as this format (has precedence over the inference from the output file extension). Required when writing to stdout
  -h, --help                     Print help
```
