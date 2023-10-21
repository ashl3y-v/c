#![allow(unused)]

mod card;
mod deck;
mod game;
mod hand;
mod key_buffer;
mod pile;
mod screen;
mod utils;

extern crate termion;

use card::Card;
use deck::Deck;
use game::Game;
use hand::Hand;
use key_buffer::KeyBuffer;
use screen::Screen;
use std::io::{stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use utils::{print, quit};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    stdout.flush().unwrap();

    let mut game = Game::new();

    game.deck.generate();
    game.deck.shuffle();

    game.hand.insert_cards(game.deck.draw_cards(5));

    while !game.quit {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            game.input(&key);
        }

        game.draw();

        game.screen.render();

        stdout.lock().flush().unwrap();

        // thread::sleep(time::Duration::from_millis(50));
    }

    quit();
}
