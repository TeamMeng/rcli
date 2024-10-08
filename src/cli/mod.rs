use clap::Parser;
use csv_opt::CsvOpts;
use std::path::{Path, PathBuf};

mod base64_opt;
mod csv_opt;
mod gen_pass_opt;
mod http_opt;
mod text_opt;

pub use base64_opt::{Base64Format, Base64SubCommand};
pub use csv_opt::OutputFormat;
pub use gen_pass_opt::GenPassOpts;
pub use http_opt::HttpSubCommand;
pub use text_opt::{TextSignFormat, TextSubCommand};

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
        assert_eq!(verify_file("*"), Err("File does not exist"));
    }
}
