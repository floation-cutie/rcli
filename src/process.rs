use std::path::PathBuf;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

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

pub fn process_csv(input: &PathBuf, output: &PathBuf) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).expect("Failed to open CSV file");
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
    std::fs::write(output, json).expect("Failed to write JSON file");
    Ok(())
}
