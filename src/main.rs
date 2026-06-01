mod app;
mod args;
mod notes;

use crate::{app::app, args::parse_args, notes::load_data};

fn main() -> std::io::Result<()> {
    // Parse args + initial setup?
    let opts = parse_args();
    let file_path = opts.file.or(Some(String::from("a_file"))).unwrap();

    let data = load_data(&file_path).expect("Oops!");

    ratatui::run(|t| app(t, data))?;
    Ok(())
}
