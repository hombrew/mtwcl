use std::io::{Stderr, Write};

use crossterm::{cursor, terminal, QueueableCommand};

use super::base_displayer::{Display, Displayer};

pub struct TextDisplayer;

impl Displayer for TextDisplayer {
    fn show(error_writer: &mut Stderr, text: &str) {
        let (width, height) = terminal::size().unwrap();
        let top = height.saturating_sub(text.lines().count() as u16) / 2;

        for (index, line) in text.lines().enumerate() {
            let x = width.saturating_sub(line.len() as u16) / 2;
            error_writer
                .queue(cursor::MoveTo(x, top + index as u16))
                .unwrap();
            error_writer.write_all(line.as_bytes()).unwrap();
        }

        error_writer.flush().unwrap();
    }
}

pub type TextSlide<'a> = Display<'a, TextDisplayer>;
