use std::io::Stderr;

use crossterm::terminal;
use termimad::{Area, MadSkin};

use super::assets_dir::ASSETS_DIR;
use super::base_displayer::{Display, Displayer};

fn text_size(text: &str) -> usize {
    let width = 1 + text.lines().fold(0, |acc, line| acc.max(line.len()));

    width
}

pub struct MarkdownDisplayer;

impl Displayer for MarkdownDisplayer {
    fn show(_error_writer: &mut Stderr, text: &str) {
        let (terminal_width, terminal_height) = terminal::size().unwrap();
        let margin = 4_usize;

        let markdown = ASSETS_DIR.get_file(text).unwrap().contents_utf8().unwrap();
        let text_width = text_size(markdown);

        let content_width = text_width.min(terminal_width as usize - margin) as u16;
        let content_height = terminal_height - (margin as u16);

        let x = (terminal_width - content_width) / 2;
        let y = (terminal_height - content_height) / 2;

        MadSkin::default()
            .write_in_area(markdown, &Area::new(x, y, content_width, content_height))
            .unwrap();
    }
}

pub type MarkdownSlide<'a> = Display<'a, MarkdownDisplayer>;
