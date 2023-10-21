use crate::Screen;
use termion::event::Key;
use std::fmt;

pub struct KeyBuffer {
    pub text: String,
    pub pos: (u16, u16),
}

impl KeyBuffer {
    pub fn new(pos: (u16, u16)) -> Self {
        KeyBuffer{ text: String::new(), pos}
    }

    pub fn draw(&self, screen: &mut Screen) {
        screen.draw(&self.text, self.pos);
    }

    pub fn erase(&self, screen: &mut Screen) {
        screen.erase(self.pos, (self.pos.0 + self.text.len() as u16, self.pos.1));
    }
}


