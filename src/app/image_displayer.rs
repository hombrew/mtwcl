use std::io::Stderr;

use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use viuer::Config;

use super::assets_dir::ASSETS_DIR;
use super::base_displayer::{Display, Displayer};

pub struct ImageDisplayer;

impl Displayer for ImageDisplayer {
    fn show(_error_writer: &mut Stderr, text: &str) {
        disable_raw_mode().unwrap();
        let markdown = ASSETS_DIR.get_file(text).unwrap();
        let (terminal_width, terminal_height) = terminal::size().unwrap();

        let img = image::load_from_memory(markdown.contents()).unwrap();
        let viuer_config = Config {
            width: Some(terminal_width as u32),
            height: Some(terminal_height as u32),
            ..Default::default()
        };
        viuer::print(&img, &viuer_config).unwrap();
        enable_raw_mode().unwrap();
    }
}

pub type ImageSlide<'a> = Display<'a, ImageDisplayer>;
