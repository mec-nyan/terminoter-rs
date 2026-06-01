use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    // Optional file to read/write our notes.
    #[arg(short, long)]
    pub file: Option<String>,
}

pub fn parse_args() -> Options {
    return Options::parse();
}
