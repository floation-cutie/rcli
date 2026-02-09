use std::fmt::Display;
use std::path::PathBuf;

use clap::Parser;

use super::verify_file_exists;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help="Input CSV file", value_name="FILE", value_parser = verify_file_exists)]
    pub input: PathBuf,
    #[arg(short, long, help = "Output JSON file", value_name = "FILE")] // "output.json".into()
    pub output: Option<PathBuf>,
    #[arg(long, help = "Supported Output Format", value_parser = parse_format, default_value = "json")]
    /// When we need a immediate `default_value_t`, we must implement `Copy` and `ToString` trait for the type
    pub format: OutputFormat,
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

fn parse_format(format: &str) -> Result<OutputFormat, String> {
    OutputFormat::try_from(format).map_err(|e| e.to_string())
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// a tryfrom<&str> basically equals to FromStr trait
// Type Conversion verus String Parsing
impl TryFrom<&str> for OutputFormat {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Unsupported format: {}", value),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}
