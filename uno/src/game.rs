use termion::cursor;
use termion::event::Key;

use crate::utils::print;
use crate::Deck;
use crate::Hand;
use crate::KeyBuffer;
use crate::Screen;

pub struct Game {
    pub screen: Screen,
    pub hand: Hand,
    pub deck: Deck,
    pub key_buffer: KeyBuffer,
    pub quit: bool,
}

impl Game {
    pub fn new() -> Self {
        print(cursor::Hide);
        let screen = Screen::new();
        let key_buffer = KeyBuffer::new((5, screen.size.1 - 5));
        Game {
            screen,
            hand: Hand::new(),
            deck: Deck::new(),
            key_buffer,
            quit: false,
        }
    }

    pub fn input(&mut self, key: &Key) {
        match key {
            Key::Char('\n') => {
                match self.key_buffer.text.as_str() {
                    "quit" => self.quit = true,
                    "d" => {
                        self.hand.insert_card(self.deck.draw_card().unwrap());
                        self.key_buffer.text.clear()
                    }
                    "l" => {
                        self.hand.move_displayed(1);
                        self.key_buffer.text.clear()
                    }
                    "h" => {
                        self.hand.move_displayed(-1);
                        self.key_buffer.text.clear()
                    }
                    _ => self.key_buffer.text.clear(),
                }
                self.key_buffer.erase(&mut self.screen);
            }
            Key::Char(chr) => self.key_buffer.text += chr.to_string().as_str(),
            Key::Backspace => {
                self.key_buffer.erase(&mut self.screen);
                self.key_buffer.text.pop();
            }
            Key::Esc => {
                self.key_buffer.erase(&mut self.screen);
                self.key_buffer.text.clear();
            }
            _ => (),
        }
    }

    pub fn draw(&mut self) {
        self.hand.draw(&mut self.screen);

        self.screen.draw(&self.deck, (15, 5));

        self.key_buffer.draw(&mut self.screen);
    }
}
