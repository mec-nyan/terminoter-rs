use ratatui::{DefaultTerminal, Frame};

pub fn app(terminal: &mut DefaultTerminal, path: &str) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            render(f, path);
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, path: &str) {
    frame.render_widget(path, frame.area());
}
