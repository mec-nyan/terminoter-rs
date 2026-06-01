use std::path::PathBuf;

use clap::Parser;
use ratatui::{DefaultTerminal, Frame};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    // Optional file to read/write our notes.
    file: Option<PathBuf>,
}

fn parse_args() -> Options {
    return Options::parse();
}

fn main() -> std::io::Result<()> {
    // Parse args + initial setup?
    //
    let opts = parse_args();

    let path = opts.file.unwrap_or_default();

    ratatui::run(|t| app(t, path.to_str().unwrap().to_string()))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, path: String) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            render(f, path.clone());
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, path: String) {
    frame.render_widget(path, frame.area());
}
