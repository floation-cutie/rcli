use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Ok;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use super::verify_file_exists;
use crate::CmdExecutor;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 Encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 Decode")]
    Decode(Base64DecodeOpts),
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = crate::process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded_bytes = crate::process_decode(&self.input, self.format)?;
        let decoded = String::from_utf8(decoded_bytes)?;
        println!("{}", decoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: PathBuf,
    #[arg(long, default_value = "standard", value_parser = parse_format)]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: PathBuf,
    #[arg(long, default_value = "standard", value_parser = parse_format)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => anyhow::bail!("Unsupported Base64 format: {}", s),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
