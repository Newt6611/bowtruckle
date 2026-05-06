use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use bowtruckle::{markdown::render_json_markdown, parser::parse_transaction_json};

#[derive(Debug, Default)]
struct Args {
    cbor_hex: String,
    output: Option<PathBuf>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args(env::args().skip(1))?;
    let cbor_hex = read_cbor_argument(&args.cbor_hex)?;
    let transaction = parse_transaction_json(cbor_hex.trim())?;
    let markdown = render_json_markdown(&transaction);

    match args.output {
        Some(path) => fs::write(path, markdown)?,
        None => io::stdout().write_all(markdown.as_bytes())?,
    }

    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut parsed = Args::default();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "." if parsed.cbor_hex.is_empty() => {}
            "-o" | "--output" => {
                parsed.output = Some(next_path(&mut args, &arg)?);
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown option `{value}`"));
            }
            value => {
                if !parsed.cbor_hex.is_empty() {
                    if parsed.output.is_some() {
                        return Err("multiple output paths provided".to_string());
                    }
                    parsed.output = Some(PathBuf::from(value));
                    continue;
                }
                parsed.cbor_hex = value.to_string();
            }
        }
    }

    if parsed.cbor_hex.is_empty() {
        return Err("missing raw CBOR hex argument".to_string());
    }

    Ok(parsed)
}

fn next_path(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<PathBuf, String> {
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("missing path after `{flag}`"))
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
    println!(
        "Usage: bowtruckle <RAW_CBOR_HEX|CBOR_FILE> [-o OUTPUT]\n\n\
         Renders Cardano transaction CBOR hex as markdown.\n\n\
         Examples:\n  \
         bowtruckle 84a700... > tx.md\n  \
         bowtruckle tx.cbor > tx.md\n  \
         bowtruckle tx.cbor tx.md\n  \
         bowtruckle 84a700... | nvim -\n  \
         bowtruckle 84a700... -o tx.md"
    );
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{parse_args, read_cbor_argument};

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
}
