use std::path::PathBuf;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string_pretty};

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

pub fn process_csv(input: &PathBuf, output: &PathBuf) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).expect("Failed to open CSV file");

    let mut ret = Vec::with_capacity(CSV_CAPACITY);
    let headers = reader.headers().expect("Failed to read headers").clone();
    // support more generic csv files
    for record in reader.records() {
        let record = record.expect("Failed to read record");
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let json = to_string_pretty(&ret).expect("Failed to serialize to JSON");
    std::fs::write(output, json).expect("Failed to write JSON file");
    Ok(())
}
