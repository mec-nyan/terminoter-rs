use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
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
    let rows = Layout::new(
        ratatui::layout::Direction::Vertical,
        vec![
            Constraint::Length(2), // App title.
            // TODO: Don't use magic numbers!
            Constraint::Fill(1), // rows...
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ],
    )
    // .spacing(1)
    .horizontal_margin(1)
    .split(frame.area());

    let mut rects = Vec::new();

    // TODO: Don't use magic numbers!
    for i in 1..5 {
        let columns = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(1); 3],
        )
        .spacing(1)
        .split(rows[i]);
        rects.push(columns);
    }

    let title = Paragraph::new(" Notes ✨").style(Style::new().yellow());

    frame.render_widget(title, rows[0]);

    if data.notes.is_empty() {
        frame.render_widget("Nothing to see here ...", rects[0][0]);
    } else {
        for i in 0..data.notes.len() {
            // TODO: Don't use magic numbers!
            let y = i / 3;
            let x = i - (y * 3);

            let p = Paragraph::new(data.notes[i].content.clone())
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .padding(Padding::proportional(1))
                        .title(format!(" #{} ({},{}) ", i + 1, x, y)),
                )
                .style(Style::new().yellow())
                .wrap(Wrap { trim: true });

            frame.render_widget(p, rects[y][x]);
        }
    }
}
