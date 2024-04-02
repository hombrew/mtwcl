use std::io::Stderr;

use super::base_displayer::{Display, Displayer};

pub struct EndDisplayer;

impl Displayer for EndDisplayer {
    fn show(_error_writer: &mut Stderr, _text: &str) {
        panic!("bye")
    }
}

pub type EndSlide<'a> = Display<'a, EndDisplayer>;
