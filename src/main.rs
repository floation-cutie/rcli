use std::fs;
use std::path::PathBuf;

use clap::Parser;
use rcli::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify,
};
use zxcvbn::zxcvbn;

// rcli csv -i input.csv -o output.json --header -d ','
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
            let estimate = zxcvbn(&password, &[]);
            if estimate.score().to_string().parse::<u8>().unwrap() < 3 {
                // Put Info to stderr, not affect the stdout for pipe
                eprintln!(
                    "Warning: The generated password is weak (score: {}). Consider increasing the length or adding more character types.",
                    estimate.score()
                );
            }
            eprintln!("Password strength: {:?}", estimate.score());
            // For stdout redirection to file, just print password solely
            println!("{}", password);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded_bytes = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded_bytes)?;
                println!("{}", decoded);
            }
        },
        // deal with symmetric and asymmetric text signing and verification
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed =
                    process_text_sign(&opts.input, opts.key.to_str().unwrap(), opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                // Placeholder for text verification logic
                let verified = process_text_verify(
                    &opts.input,
                    opts.key.to_str().unwrap(),
                    opts.format,
                    &opts.sig,
                )?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_text_generate(opts.format)?;
                let output_path = &opts.output;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let output_path = output_path.join("blake3.txt");
                        fs::write(output_path, &keys[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        fs::write(output_path.join("ed25519.sk"), &keys[0])?;
                        fs::write(output_path.join("ed25519.pk"), &keys[1])?;
                    }
                }
            }
        },
        // static http file server
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(&opts.dir, opts.port).await?;
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
