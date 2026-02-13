use clap::Parser;
use zxcvbn::zxcvbn;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    // 特殊字符 、数字 、大写字母、小写字母
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = false)]
    pub special: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.special,
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
        Ok(())
    }
}
