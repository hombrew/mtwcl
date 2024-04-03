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
        SlideTypes::Text("Hi, everyone from Tailwind Labs 🫡"),
        SlideTypes::Text("Who am I?"),
        SlideTypes::Markdown("1.md"),
        SlideTypes::Image("1_ela.jpeg"),
        SlideTypes::Text("How did I get here?"),
        SlideTypes::Markdown("2.md"),
        SlideTypes::Image("2_react_vs_angular.jpeg"),
        SlideTypes::Text("Why should I be The Chosen One?"),
        SlideTypes::Image("3_neo.jpeg"),
        SlideTypes::Markdown("3.md"),
        SlideTypes::Markdown("3.1.md"),
        SlideTypes::Image("3_winsports.jpeg"),
        SlideTypes::Image("3_ondamedia.jpeg"),
        SlideTypes::Markdown("3.2.md"),
        SlideTypes::Image("3_boreal.jpeg"),
        SlideTypes::Markdown("3.3.md"),
        SlideTypes::Image("3_i_love_ts.png"),
        SlideTypes::Text("What lies ahead"),
        SlideTypes::Markdown("4.md"),
        SlideTypes::Text(
            "End\n\nThank you for taking the time to read about my journey.\n\nI hope I already convinced you enough to start a conversation.\n\n\n\n\n\nMade with ♥️  by Manuel Maldonado.",
        ),
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
