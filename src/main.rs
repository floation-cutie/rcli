use clap::Parser;
use rcli::{CmdExecutor, Opts};

// rcli csv -i input.csv -o output.json --header -d ','
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Opts::parse();
    cli.cmd.execute().await?;

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
