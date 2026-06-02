use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::notes::Data;

pub struct App {
    data: Data,
    current_note: u16,
    current_page: u16,
    quit: bool,
}

impl App {
    pub fn new(data: Data) -> Self {
        return Self {
            data,
            current_note: 0,
            current_page: 0,
            quit: false,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| {
                self.render(f);
            })?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
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

        if self.data.notes.is_empty() {
            // TODO: Show a help message here.
            let p = Paragraph::new("There are no notes ...")
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .padding(Padding::proportional(1)),
                )
                .style(Style::new().dark_gray())
                .wrap(Wrap { trim: true });

            frame.render_widget(p, rects[0][0]);
        } else {
            for i in 0..self.data.notes.len() {
                // TODO: Don't use magic numbers!
                let y = i / 3;
                let x = i - (y * 3);

                let p = Paragraph::new(self.data.notes[i].content.clone())
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
}
