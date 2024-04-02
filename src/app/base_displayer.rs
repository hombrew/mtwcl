use std::{io::Stderr, marker::PhantomData};

pub trait Displayer {
    fn show(error_writer: &mut Stderr, text: &str);
}

pub struct Display<'a, DISPLAYER: Displayer> {
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
