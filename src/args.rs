use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Share directory
    #[arg(short, long, value_name = "FILE")]
    pub dir: PathBuf,
}
