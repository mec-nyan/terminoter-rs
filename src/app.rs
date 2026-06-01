use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

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
        let mut constraints: Vec<Constraint> = Vec::new();
        for _ in 0..data.notes.len() {
            constraints.push(Constraint::Min(1));
        }
        constraints.push(Constraint::Fill(2));
        let layout =
            Layout::new(ratatui::layout::Direction::Vertical, constraints).split(frame.area());

        for i in 0..data.notes.len() {
            let note = data.notes[i].clone();
            let p = Paragraph::new(note.content)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .padding(Padding::new(2, 2, 1, 1)),
                )
                .style(Style::new().yellow());
            frame.render_widget(p, layout[i]);
        }
    }
}
