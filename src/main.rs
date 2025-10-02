use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help="Input CSV file", value_name="FILE", value_parser = verify_file_exists)]
    input: PathBuf,
    #[arg(
        short,
        long,
        help = "Output JSON file",
        value_name = "FILE",
        default_value = "output.json"
    )] // "output.json".into()
    output: PathBuf,
    /// short option names must be unique for each argument, --help and --header conflict
    #[arg(
        long,
        help = "Indicates that the CSV file has a header row
",
        default_value_t = false
    )]
    header: bool,
    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    delimiter: char,
}

fn verify_file_exists(file: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file);
    if path.exists() { Ok(path) } else { Err(format!("File '{}' does not exist", file)) }
}

// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    // pattern matching on the subcommand
    match &cli.cmd {
        SubCommand::Csv(opts) => {
            println!("{:?}", opts);
            let mut reader = Reader::from_path(&opts.input).expect("Failed to open CSV file");
            if opts.header {
                // reader.set_headers(None);
            }
            /*
            let mut _record = Vec::new();
            // Since it is a borrowed iterator, the reader itself didn't move. However
            for result in reader.deserialize(){
                let record: Player = result.expect("Failed to deserialize record");
                // println!("{:?}", record);
                _record.push(record);
            }
            */
            let records = reader
                .deserialize()
                .map(|result| result.expect("Failed to deserialize record"))
                .collect::<Vec<Player>>();
            println!("{:?}", records);
            let json = to_string_pretty(&records).expect("Failed to serialize to JSON");
            std::fs::write(&opts.output, json).expect("Failed to write JSON file");
            println!("Converted '{}' to '{}'", opts.input.display(), opts.output.display());
        }
    }
    // println!("{:?}", cli);
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
