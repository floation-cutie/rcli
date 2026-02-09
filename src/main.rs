use std::path::PathBuf;

use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand, process_csv,
    process_decode, process_encode, process_genpass, process_text_sign,
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
            // For stdout redirection to file, just print password solely
            println!("{}", password);
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
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                match opts.format {
                    TextSignFormat::Blake3 => {
                        process_text_sign(&opts.input, opts.key.to_str().unwrap(), opts.format)?
                    }
                    TextSignFormat::Ed25519 => {
                        // Placeholder for Ed25519 signing logic
                        println!(
                            "Signing text from {:?} using key {:?} with format {:?}",
                            opts.input, opts.key, opts.format
                        );
                    }
                }
            }
            TextSubCommand::Verify(opts) => {
                // Placeholder for text verification logic
                println!(
                    "Verifying text from {:?} using key {:?} with format {:?} and signature {}
                ",
                    opts.input, opts.key, opts.format, opts.sig
                );
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
