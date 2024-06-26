use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::io::{stderr, stdout, Stderr, Write};

mod assets_dir;
mod base_displayer;
mod displayer;
mod end_displayer;
mod image_displayer;
mod markdown_displayer;
mod text_displayer;

pub struct App {
    error_writer: Stderr,
    index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            index: 0,
            error_writer: stderr(),
        }
    }

    fn run(&mut self) {
        loop {
            self.error_writer
                .queue(Clear(ClearType::All))
                .unwrap()
                .queue(MoveTo(0, 0))
                .unwrap();
            self.error_writer.flush().unwrap();

            displayer::display(&mut self.error_writer, self.index);

            let current_event = event::read().expect("failed to read keyboard input");
            if let Event::Key(event_key) = current_event {
                match event_key {
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => {
                        self.index += 1;
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => {
                        if self.index > 0 {
                            self.index -= 1;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q'),
                        ..
                    } => {
                        break;
                    }
                    _ => {}
                };
            }
        }
    }

    pub fn start(&mut self) {
        enable_raw_mode().unwrap();
        execute!(stdout(), terminal::SetSize(80, 35)).unwrap();
        queue!(self.error_writer, Hide).unwrap();
        self.run();
    }
}

impl Drop for App {
    fn drop(&mut self) {
        queue!(self.error_writer, Show).unwrap();
        disable_raw_mode().unwrap();
    }
}
