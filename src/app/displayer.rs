use std::{
    io::{Stderr, Write},
    marker::PhantomData,
};

use crossterm::{cursor, terminal, QueueableCommand};

trait Displayer {
    fn show(error_writer: &mut Stderr, text: &str);
}

struct Display<'a, DISPLAYER: Displayer> {
    displayer: PhantomData<DISPLAYER>,
    error_writer: &'a mut Stderr,
}

impl<'a, DISPLAYER: Displayer> Display<'a, DISPLAYER> {
    pub fn new(error_writer: &'a mut Stderr) -> Self {
        Self {
            error_writer,
            displayer: PhantomData,
        }
    }

    pub fn display(&mut self, text: &str) {
        DISPLAYER::show(self.error_writer, text);
    }
}

struct TextDisplayer;
type TextSlide<'a> = Display<'a, TextDisplayer>;
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

struct EndDisplayer;
type EndSlide<'a> = Display<'a, EndDisplayer>;
impl Displayer for EndDisplayer {
    fn show(_error_writer: &mut Stderr, _text: &str) {
        panic!("bye")
    }
}

enum SlideTypes<'a> {
    Text(&'a str),
    End(),
}

pub fn display(error_writer: &mut Stderr, index: usize) {
    let slides: Vec<SlideTypes> = vec![
        SlideTypes::Text("holi"),
        SlideTypes::Text("this is just a test"),
        SlideTypes::End(),
    ];

    if index < slides.len() {
        match slides[index] {
            SlideTypes::Text(text) => TextSlide::new(error_writer).display(text),
            SlideTypes::End() => EndSlide::new(error_writer).display("end"),
        }
    }
}
