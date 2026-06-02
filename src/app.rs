use crossterm::event::{self, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::notes::Data;

pub struct App {
    data: Data,
    current_note: usize,
    _current_page: usize,
    quit: bool,
    show_debuggin_info: bool,
}

impl App {
    pub fn new(data: Data) -> Self {
        return Self {
            data,
            current_note: 0,
            _current_page: 0,
            quit: false,
            show_debuggin_info: false,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.quit {
            terminal.draw(|f| {
                self.render(f);
            })?;
            // For our usecase, blocking I/O is just what we need.
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            event::Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => {
                self.handle_keys(key_ev);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_keys(&mut self, ev: KeyEvent) {
        match ev.code {
            event::KeyCode::Char('q') | event::KeyCode::Esc => self.quit = true,
            event::KeyCode::Char('j') => self.next(),
            event::KeyCode::Char('k') => self.previous(),
            event::KeyCode::Char('d') => self.toggle_debuggin_info(),
            _ => {}
        }
    }

    fn next(&mut self) {
        if !self.data.notes.is_empty() {
            if self.current_note < self.data.notes.len() - 1 {
                self.current_note += 1;
            }
        }
    }

    fn previous(&mut self) {
        if self.current_note > 0 {
            self.current_note -= 1;
        }
    }

    fn toggle_debuggin_info(&mut self) {
        self.show_debuggin_info = !self.show_debuggin_info;
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

                let note_style = if i == self.current_note {
                    Style::new().yellow()
                } else {
                    Style::new().blue()
                };

                let p = Paragraph::new(self.data.notes[i].content.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Rounded)
                            .padding(Padding::proportional(1))
                            .title(format!(" #{} ", i + 1)),
                    )
                    .style(note_style)
                    .wrap(Wrap { trim: true });

                let rect = rects[y][x];

                frame.render_widget(&p, rect);

                if self.show_debuggin_info {
                    let lines = p.line_count(rect.width);
                    let p_rows = rect.width;
                    let p_cols = rect.height;

                    let bottom_line = Layout::new(
                        ratatui::layout::Direction::Vertical,
                        vec![Constraint::Fill(1), Constraint::Length(1)],
                    )
                    .horizontal_margin(4)
                    .split(rect);

                    let info = Line::raw(format!(
                        " i: {}l ({}x{}) ({},{}) ",
                        lines, p_cols, p_rows, x, y
                    ));

                    frame.render_widget(info, bottom_line[1]);
                }
            }
        }
    }
}
