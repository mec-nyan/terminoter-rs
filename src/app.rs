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
    let main_pane = frame.area();

    let layout = Layout::new(
        ratatui::layout::Direction::Vertical,
        vec![
            Constraint::Length(3), // Title.
            Constraint::Fill(1),   // Notes.
        ],
    )
    .split(main_pane);

    let title = Paragraph::new("== Notes ==")
        .block(Block::new().padding(Padding::uniform(1)))
        .centered();

    frame.render_widget(title, layout[0]);

    if data.notes.is_empty() {
        frame.render_widget("Nothing to see here ...", layout[1]);
    } else {
        let constraints: Vec<Constraint> = vec![Constraint::Min(0); data.notes.len() + 1];
        let layout =
            Layout::new(ratatui::layout::Direction::Vertical, constraints).split(layout[1]);

        for i in 0..data.notes.len() {
            let p = Paragraph::new(data.notes[i].content.clone())
                .block(
                    Block::new()
                        .padding(Padding {
                            left: 1,
                            right: 1,
                            top: 0,
                            bottom: 1,
                        })
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded),
                )
                .style(Style::new().yellow())
                .wrap(Wrap { trim: true });
            frame.render_widget(p, layout[i]);
        }
    }
}
