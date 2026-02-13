use std::path::PathBuf;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use super::verify_path;
use crate::CmdExecutor;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "Start a simple HTTP server to serve files from a directory")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_http_serve(&self.dir, self.port).await
    }
}
