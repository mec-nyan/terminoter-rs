mod app;
mod args;

use crate::{app::app, args::parse_args};

fn main() -> std::io::Result<()> {
    // Parse args + initial setup?
    //
    let opts = parse_args();

    let path = opts.file.unwrap_or("a file".into());

    ratatui::run(|t| app(t, path.to_str().unwrap()))?;
    Ok(())
}
