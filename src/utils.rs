use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_reader(input: &Path) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input.to_str() == Some("-") {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
