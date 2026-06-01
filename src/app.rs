use ratatui::{DefaultTerminal, Frame};

use crate::notes::Data;

pub fn app(terminal: &mut DefaultTerminal, data: Data) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            render(f, &data);
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, data: &Data) {
    if data.notes.len() == 0 {
        frame.render_widget("Nothing to see here...", frame.area());
    } else {
        frame.render_widget("There are notes!", frame.area());
    }
}
