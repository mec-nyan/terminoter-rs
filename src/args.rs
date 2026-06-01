use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    // Optional file to read/write our notes.
    pub file: Option<PathBuf>,
}

pub fn parse_args() -> Options {
    return Options::parse();
}
