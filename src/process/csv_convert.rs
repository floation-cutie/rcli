use std::path::PathBuf;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::OutputFormat;

const CSV_CAPACITY: usize = 128;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &PathBuf, output: &PathBuf, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).expect("Failed to open CSV file");

    let mut ret = Vec::with_capacity(CSV_CAPACITY);
    let headers = reader.headers().expect("Failed to read headers").clone();
    // support more generic csv files
    for record in reader.records() {
        let record = record.expect("Failed to read record");
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret).expect("Failed to serialize to YAML"),
    };
    std::fs::write(output, content).expect("Failed to write output file");
    Ok(())
}
