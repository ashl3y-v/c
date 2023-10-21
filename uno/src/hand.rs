use crate::Card;
use crate::Screen;
use crate::card;
use termion::terminal_size;
use std::{fmt, fs};

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    displayed_cards: (u16, u16)
}

impl Hand {
    pub fn new() -> Self {
        let size = terminal_size().unwrap();
        let displayed_cards: u16 = (size.0 - 30) / card::SIZE.0 - 1;
        Self { cards: Vec::new(), displayed_cards: (0, displayed_cards) }
    }
    
    pub fn draw_card(&mut self, i: usize) -> Card {
        self.cards.remove(i)
    }

    pub fn insert_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    
    pub fn insert_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }

    pub fn move_displayed(&mut self, n: i16) {
        if self.displayed_cards.0 as i16 + n > 0 && self.displayed_cards.1 as i16 + n < self.cards.len() as i16 + 1 {
            println!("done");
            self.displayed_cards.0 = (self.displayed_cards.0 as i16 + n) as u16;
            self.displayed_cards.1 = (self.displayed_cards.1 as i16 + n) as u16;
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        let pos = (15, 15);
        let card_size = card::SIZE;
        let displayed_cards = (self.displayed_cards.0, { if self.displayed_cards.1 <= (self.cards.len() - 1) as u16 { self.displayed_cards.1 } else { (self.cards.len()) as u16 }});
        
        screen.erase((pos.0, pos.1), (pos.0 + card_size.0 * (displayed_cards.1 - displayed_cards.0 + 1), pos.1 + card_size.1));

        for i in displayed_cards.0..displayed_cards.1 {
            let rel_pos = i - displayed_cards.0;

            screen.draw(&self.cards[i as usize], (pos.0 + card_size.1 * rel_pos, pos.1));
        }
    }
}
