use rand::{rngs::ThreadRng, thread_rng, seq::SliceRandom};

use crate::card::{self, Card};
use std::fmt;
use std::iter;

pub struct Deck {
    cards: Vec<Card>,
    rng: ThreadRng
}

impl Deck {
    pub fn new() -> Deck {
        Self {cards: Vec::new(), rng: thread_rng()}
    }

    pub fn generate(&mut self) {
        for v in card::VALUES {
            for s in card::SUITS {
                self.cards.push(Card::new(v, s));
            }
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_cards(&mut self, n: usize) -> Vec<Card> {
        let mut drawn_cards = Vec::new();
        print!("{}", self.cards.len());
        for _ in 0..n {
            drawn_cards.push(self.draw_card().unwrap());
        }

        drawn_cards
    }

    pub fn insert_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards)
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards_number = self.cards.len().to_string() + iter::repeat(" ".chars().next().unwrap()).take(5 - self.cards.len().to_string().len()).collect::<String>().as_str();

        write!(
            f,
            " _____
|{cards_number}|
|     |
|     |
|     |
|     |
|_____|"
        )
    }
}
