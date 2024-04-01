use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::io::{stderr, Stderr, Write};

pub struct App {
    writer: Stderr,
    index: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            index: 0,
            writer: stderr(),
        }
    }

    fn run(&mut self) {
        loop {
            self.writer
                .queue(Clear(ClearType::All))
                .unwrap()
                .queue(MoveTo(0, 0))
                .unwrap();
            self.writer.flush().unwrap();

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
                        code: KeyCode::Esc, ..
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
        queue!(self.writer, Hide).unwrap();
        self.run();
    }
}

impl Drop for App {
    fn drop(&mut self) {
        queue!(self.writer, Show).unwrap();
        disable_raw_mode().unwrap();
    }
}
