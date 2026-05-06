use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use bowtruckle::{markdown::render_json_markdown, parser::parse_transaction_json};
use clap::{CommandFactory, Parser, error::ErrorKind};

#[derive(Debug, Default)]
struct Args {
    cbor_hex: String,
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(
    name = "bowtruckle",
    version,
    about = "Decode Cardano transaction CBOR into Markdown",
    after_help = "Examples:
  bowtruckle 84a700...
  bowtruckle tx.cbor
  bowtruckle tx.cbor tx.md
  bowtruckle tx.cbor -o tx.md
  bowtruckle 84a700... | nvim -"
)]
struct Cli {
    #[arg(value_name = "RAW_CBOR_HEX|CBOR_FILE")]
    cbor_hex: String,

    #[arg(value_name = "OUTPUT")]
    positional_output: Option<PathBuf>,

    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("bowtruckle: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    if raw_args.is_empty() {
        print_usage();
        return Ok(());
    }

    let args = parse_args(raw_args)?;
    let cbor_hex = read_cbor_argument(&args.cbor_hex)?;
    let transaction = parse_transaction_json(cbor_hex.trim())?;
    let markdown = render_json_markdown(&transaction);

    match args.output {
        Some(path) => fs::write(path, markdown)?,
        None => io::stdout().write_all(markdown.as_bytes())?,
    }

    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Args, Box<dyn std::error::Error>> {
    let args = normalize_args(args);
    let cli = match Cli::try_parse_from(std::iter::once("bowtruckle".to_string()).chain(args)) {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            error.print()?;
            std::process::exit(0);
        }
        Err(error) => return Err(error.into()),
    };

    if cli.positional_output.is_some() && cli.output.is_some() {
        return Err("provide either positional OUTPUT or --output, not both".into());
    }

    Ok(Args {
        cbor_hex: cli.cbor_hex,
        output: cli.output.or(cli.positional_output),
    })
}

fn normalize_args(args: impl IntoIterator<Item = String>) -> Vec<String> {
    args.into_iter()
        .enumerate()
        .filter_map(|(index, arg)| {
            if index == 0 && arg == "." {
                None
            } else {
                Some(arg)
            }
        })
        .collect()
}

fn read_cbor_argument(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = PathBuf::from(value);
    if path.is_file() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(value.to_string())
    }
}

fn print_usage() {
    Cli::command().print_help().expect("help should print");
    println!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{Cli, parse_args, read_cbor_argument};
    use clap::{CommandFactory, Parser};

    #[test]
    fn accepts_raw_cbor_argument() {
        let args = parse_args(["84a700".to_string()]).expect("args should parse");

        assert_eq!(args.cbor_hex, "84a700");
        assert!(args.output.is_none());
    }

    #[test]
    fn ignores_leading_dot_from_cargo_run_dot() {
        let args = parse_args([".".to_string(), "84a700".to_string()]).expect("args should parse");

        assert_eq!(args.cbor_hex, "84a700");
    }

    #[test]
    fn reads_cbor_argument_from_existing_file() {
        let path =
            std::env::temp_dir().join(format!("bowtruckle-test-{}.cbor", std::process::id()));
        fs::write(&path, "84a700\n").expect("fixture should write");

        let cbor = read_cbor_argument(path.to_str().expect("path should be utf8"))
            .expect("file should read");

        assert_eq!(cbor, "84a700\n");
        fs::remove_file(path).ok();
    }

    #[test]
    fn accepts_positional_output_path() {
        let args =
            parse_args(["cc".to_string(), "tx11.md".to_string()]).expect("args should parse");

        assert_eq!(args.cbor_hex, "cc");
        assert_eq!(args.output, Some("tx11.md".into()));
    }

    #[test]
    fn empty_args_still_report_missing_cbor_to_parser() {
        let error = parse_args(Vec::<String>::new()).expect_err("args should fail");

        assert!(error.to_string().contains("required"));
    }

    #[test]
    fn supports_version_flag() {
        let error = Cli::try_parse_from(["bowtruckle", "--version"]).expect_err("version exits");

        assert!(error.to_string().contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn help_includes_examples() {
        let help = Cli::command().render_help().to_string();

        assert!(help.contains("Examples:"));
        assert!(help.contains("bowtruckle tx.cbor -o tx.md"));
    }
}
