use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

use crate::notes::{Data, Note, save_data};

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
    Delete,
}

enum SaveOnEnter {
    None,
    Once,
    Save,
}

pub struct App {
    data_new: Data,
    data_old: Data,

    mode: Mode,
    save_on_enter: SaveOnEnter,

    tmp_content: String,

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
            data_new: data.clone(),
            data_old: data,
            mode: Mode::Normal,
            save_on_enter: SaveOnEnter::None,
            tmp_content: String::new(),
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

        // Don't overwrite the file if nothing has changed.
        // Sadly, we'll still need to write if only the current note index has changed,
        // in order to preserve the application's state.
        // TODO: If that's not useful (saving `current`), remove it.
        if self.save_on_quit && self.has_changed() {
            save_data("data.json", &self.data_new)?;
        }

        Ok(())
    }

    fn has_changed(&self) -> bool {
        self.data_new != self.data_old
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
        match self.mode {
            Mode::Normal => {
                match ev.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
                    KeyCode::Char('j') => self.next(),
                    KeyCode::Char('k') => self.previous(),
                    // TODO: Use a more common keybinding (i.e. "ctrl+u" or "ctrl+p" etc).
                    KeyCode::Char('u') => self.up(),
                    KeyCode::Char('d') => self.down(),
                    KeyCode::Char('i') => self.toggle_insert_mode(),
                    KeyCode::Char('b') => self.toggle_debugging_info(),
                    _ => {}
                }
            }
            Mode::Insert => match ev.code {
                // Exit insert mode and discard note.
                // TODO: Add confirmation dialog.
                KeyCode::Esc => self.discard(),
                // Add new note to list of notes and go back to normal mode.
                KeyCode::Enter => match self.save_on_enter {
                    SaveOnEnter::None => {
                        self.save_on_enter = SaveOnEnter::Once;
                        self.tmp_content.push('\n');
                    }
                    SaveOnEnter::Once => {
                        self.save_on_enter = SaveOnEnter::Save;
                        self.tmp_content.push('\n');
                    }
                    SaveOnEnter::Save => {
                        self.save_on_enter = SaveOnEnter::None;
                        self.save_new_note();
                    }
                },
                // Type the new note...
                KeyCode::Backspace => {
                    if self.tmp_content.len() > 0 {
                        self.tmp_content.pop();
                    }
                }
                KeyCode::Tab => {
                    self.tmp_content.push('\t');
                }
                KeyCode::Char(c) => {
                    if c.is_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                        self.tmp_content.push(c);
                    }
                }

                _ => {}
            },
            Mode::Delete => {}
        }
    }

    fn toggle_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }

    fn discard(&mut self) {
        self.tmp_content.clear();
        self.mode = Mode::Normal;
    }

    fn save_new_note(&mut self) {
        self.mode = Mode::Normal;
        // Remove last '\n'.
        self.tmp_content.pop();
        let new_note = Note {
            content: self.tmp_content.clone(),
        };
        self.data_new.notes.push(new_note);
        self.tmp_content.clear();
    }

    fn reset_offset(&mut self) {
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn next(&mut self) {
        if !self.data_new.notes.is_empty() {
            if self.data_new.current < self.data_new.notes.len() - 1 {
                self.data_new.current += 1;
            }
        }
        // NOTE: Once each note saves its own offset, remove this code:
        self.reset_offset();
    }

    fn previous(&mut self) {
        if self.data_new.current > 0 {
            self.data_new.current -= 1;
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

        if self.data_new.notes.is_empty() {
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
            for i in 0..self.data_new.notes.len() {
                // TODO: Don't use magic numbers!
                let y = i / 3;
                let x = i - (y * 3);

                let note_style = if self.mode == Mode::Insert {
                    Style::new().dark_gray()
                } else if i == self.data_new.current {
                    Style::new().yellow()
                } else {
                    Style::new().blue()
                };

                let offset = if i == self.data_new.current {
                    (self.offset_y, self.offset_x)
                } else {
                    (0, 0)
                };

                let p = Paragraph::new(self.data_new.notes[i].content.clone())
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

        // TODO: This is just a draft implementation.
        if self.mode == Mode::Insert {
            let note_number = self.data_new.notes.len();
            let y = note_number / 3;
            let x = note_number - (y * 3);

            let note_style = Style::new().green();
            let p = Paragraph::new(self.tmp_content.clone() + "_")
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
                        .title(format!(" (temp) #{} ", note_number + 1)),
                )
                .style(note_style)
                // .scroll(offset)
                .wrap(Wrap { trim: false });

            let rect = rects[y][x];

            frame.render_widget(&p, rect);
        }
    }
}
