use std::{fmt::Display, str::Chars};
use termion::{terminal_size, cursor, clear};

use crate::utils::print;

pub struct Screen {
    pixels: Vec<Vec<char>>,
    pub size: (u16, u16),
    changed: bool
}

impl Screen {
    pub fn new() -> Self {
        let mut pixels = Vec::new();
        let size = terminal_size().unwrap();

        for y in 0..size.1 {
            pixels.push(Vec::new());
            for _ in 0..size.0 {
                pixels[y as usize].push(' ');
            }
        }

        Screen { pixels, size, changed: false }
    }

    pub fn draw(&mut self, draw_string: impl Display, pos: (u16, u16)) {
        let draw_string = draw_string.to_string();
        let lines: Vec<Chars> = draw_string.lines().map(|x| x.chars()).collect();

        for y in 0..lines.len() {
            let line: Vec<char> = lines[y].clone().collect();

            for x in 0..line.len() {
                self.pixels[y + pos.1 as usize][x + pos.0 as usize] = line[x];
            }
        }

        self.changed = true;
    }

    pub fn erase(&mut self, start: (u16, u16), end: (u16, u16)) {
        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                self.pixels[y as usize][x as usize] = ' ';
            }
        }

    }
    
    pub fn render(&mut self) {
        if self.changed {
            print(cursor::Goto(1, 1));
            // print(clear::All);

            for (y, line) in self.pixels.iter().enumerate() {
                let line: String = line.iter().collect();
                print(cursor::Goto(1, y as u16));
                print(line);
            }

            self.changed = false;
        }
    }
}
