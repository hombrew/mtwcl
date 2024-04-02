use std::io::Stderr;

use super::end_displayer::EndSlide;
use super::image_displayer::ImageSlide;
use super::markdown_displayer::MarkdownSlide;
use super::text_displayer::TextSlide;

enum SlideTypes<'a> {
    Text(&'a str),
    Markdown(&'a str),
    Image(&'a str),
    End(),
}

pub fn display(error_writer: &mut Stderr, index: usize) {
    let slides: Vec<SlideTypes> = vec![
        SlideTypes::Text("holi"),
        SlideTypes::Text("this is just a test"),
        SlideTypes::Markdown("example.md"),
        SlideTypes::Image("logo.png"),
        SlideTypes::End(),
    ];

    if index < slides.len() {
        match slides[index] {
            SlideTypes::Text(text) => TextSlide::new(error_writer).display(text),
            SlideTypes::Markdown(text) => MarkdownSlide::new(error_writer).display(text),
            SlideTypes::Image(text) => ImageSlide::new(error_writer).display(text),
            SlideTypes::End() => EndSlide::new(error_writer).display("end"),
        }
    }
}
