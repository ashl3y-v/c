use std::fmt::Display;

use termion::{clear, cursor};

pub fn print(x: impl Display) {
    print!("{x}");
}

pub fn quit() {
    print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show);
}
