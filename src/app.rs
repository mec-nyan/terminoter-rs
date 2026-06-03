use crossterm::event::{self, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

use crate::notes::{Data, save_data};

pub struct App {
    data: Data,
    _current_page: usize,
    quit: bool,
    save_on_quit: bool,
    // TODO: Remove on production builds once the app reaches a usable state.
    show_debugging_info: bool,

    // TODO: This will be stored in the note's "metadata" so we can keep each note's state
    // independently.
    offset_x: u16,
    offset_y: u16,
}

impl App {
    pub fn new(data: Data) -> Self {
        return Self {
            data,
            _current_page: 0,
            quit: false,
            // TODO: Add a confirmation dialog to save the changes.
            save_on_quit: true,
            show_debugging_info: false,
            offset_x: 0,
            offset_y: 0,
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

        if self.save_on_quit {
            save_data("data.json", &self.data)?;
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
            // TODO: Use a more common keybinding (i.e. "ctrl+u" or "ctrl+p" etc).
            event::KeyCode::Char('u') => self.up(),
            event::KeyCode::Char('d') => self.down(),
            event::KeyCode::Char('i') => self.toggle_debugging_info(),
            _ => {}
        }
    }

    fn reset_offset(&mut self) {
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn next(&mut self) {
        if !self.data.notes.is_empty() {
            if self.data.current < self.data.notes.len() - 1 {
                self.data.current += 1;
            }
        }
        // NOTE: Once each note saves its own offset, remove this code:
        self.reset_offset();
    }

    fn previous(&mut self) {
        if self.data.current > 0 {
            self.data.current -= 1;
        }
        // NOTE: Once each note saves its own offset, remove this code:
        self.reset_offset();
    }

    fn up(&mut self) {
        self.offset_y += 1;
    }

    fn down(&mut self) {
        if self.offset_y > 0 {
            self.offset_y -= 1;
        }
    }

    fn toggle_debugging_info(&mut self) {
        self.show_debugging_info = !self.show_debugging_info;
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

                let note_style = if i == self.data.current {
                    Style::new().yellow()
                } else {
                    Style::new().blue()
                };

                let offset = if i == self.data.current {
                    (self.offset_y, self.offset_x)
                } else {
                    (0, 0)
                };

                let p = Paragraph::new(self.data.notes[i].content.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Rounded)
                            .padding(Padding {
                                left: 2,
                                right: 2,
                                top: 1,
                                bottom: 1,
                            })
                            .title(format!(" #{} ", i + 1)),
                    )
                    .style(note_style)
                    .scroll(offset)
                    .wrap(Wrap { trim: false });

                let rect = rects[y][x];

                frame.render_widget(&p, rect);

                // Show a scrollbar if the content don't fit on the rect.
                let lines = p.line_count(rect.width);
                let p_rows = rect.height;
                let p_cols = rect.width;

                if lines >= p_rows as usize {
                    let scrollbar =
                        Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                            .begin_symbol(None)
                            .end_symbol(None)
                            .track_style(Style::new().black());

                    let bar_rect = rect.inner(ratatui::layout::Margin {
                        horizontal: 0,
                        vertical: 2,
                    });

                    let mut bar_state = ScrollbarState::new(lines - (p_rows - 4) as usize)
                        .position(self.offset_y.into());

                    frame.render_stateful_widget(scrollbar, bar_rect, &mut bar_state);
                }

                if self.show_debugging_info {
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
