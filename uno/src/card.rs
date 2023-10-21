use std::fmt;
use std::iter;

pub static VALUES: [&str; 13] = ["K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2", "A"];
pub static SUITS: [&str; 4] = ["C", "D", "H", "S"];
pub static SIZE: (u16, u16) = (7, 7);

#[derive(Debug)]
pub struct Card {
    pub value: String,
    pub suit: String,
    pub size: (u16, u16)
}

impl Card {
    pub fn new(value: impl ToString, suit: impl ToString) -> Card {
        Self {value: value.to_string(), suit: suit.to_string(), size: SIZE}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = self.value.clone() + iter::repeat(" ".chars().next().unwrap()).take(5 - self.value.len()).collect::<String>().as_str();
        let suit_str = iter::repeat("_".chars().next().unwrap()).take(5 - self.suit.len()).collect::<String>() + self.suit.as_str();

        write!(
            f,
            " _____
|{value_str}|
|     |
|     |
|     |
|     |
|{suit_str}|"
        )
    }
}
