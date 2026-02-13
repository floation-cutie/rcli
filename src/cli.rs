mod base64;
mod csv;
mod genpass;
mod http;
mod text;
use std::path::{Path, PathBuf};

pub use base64::*;
use clap::{Parser, Subcommand};
pub use csv::*;
use enum_dispatch::enum_dispatch;
pub use genpass::*;
pub use http::*;
pub use text::*;

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text signing/verification")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server for file serving and directory listing")]
    Http(HttpSubCommand),
}

fn verify_file_exists(filename: &str) -> Result<PathBuf, String> {
    // if input is a valid file path(File Read) or a '-'(stdin)
    if filename == "-" || Path::new(filename).exists() {
        Ok(PathBuf::from(filename))
    } else {
        Err(format!("File '{}' does not exist", filename))
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(PathBuf::from(path))
    } else {
        Err(format!("Path '{}' is not a valid directory", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_path() {
        assert!(verify_file_exists("non_existent_file.txt").is_err());
        assert_eq!(verify_file_exists("src/lib.rs").unwrap(), PathBuf::from("src/lib.rs"));
        assert_eq!(verify_file_exists("-").unwrap(), PathBuf::from("-"));
    }
}
