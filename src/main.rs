use std::path::PathBuf;

use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, process_csv, process_decode, process_encode,
    process_genpass,
};

// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    // pattern matching on the subcommand
    match &cli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(ref output) = opts.output {
                output
            } else {
                // to get a &PathBuf from a String
                &PathBuf::from(format!("output.{}", opts.format))
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.special,
            )?;
            println!("Generated password: {}", password);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", decoded);
            }
        },
    }
    // can use cli afterwards
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
