use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Ok;
use clap::{Parser, Subcommand};

use super::{verify_file_exists, verify_path};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(about = "Sign text data with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "verify signed text data with a public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key for text signing")]
    Generate(TextGenOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: PathBuf,
    /// Input and Key can't read from stdin at the same time
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: PathBuf,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: PathBuf,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: PathBuf,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    /// To allow special characters, sig should allow hyphen values
    #[arg(short, long, allow_hyphen_values = true)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextGenOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    /// Authentication Only (Shared Key)
    Blake3,
    /// Public Signing (Asymmetric)
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => anyhow::bail!("Unsupported Text Sign format: {}", s),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}
