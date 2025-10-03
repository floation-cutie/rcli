use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help="Input CSV file", value_name="FILE", value_parser = verify_file_exists)]
    pub input: PathBuf,
    #[arg(
        short,
        long,
        help = "Output JSON file",
        value_name = "FILE",
        default_value = "output.json"
    )] // "output.json".into()
    pub output: PathBuf,
    /// short option names must be unique for each argument, --help and --header conflict
    #[arg(
        long,
        help = "Indicates that the CSV file has a header row
",
        default_value_t = false
    )]
    pub header: bool,
    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    pub delimiter: char,
}

fn verify_file_exists(file: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file);
    if path.exists() { Ok(path) } else { Err(format!("File '{}' does not exist", file)) }
}
