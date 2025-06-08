use crate::{Card, Rank, Suit};

#[cfg(test)]
mod tests;

pub trait UnicodeDisplay {
    fn to_unicode(&self) -> String;
}

impl UnicodeDisplay for Suit {
    fn to_unicode(&self) -> String {
        match self {
            Suit::Clubs => "♣".into(),
            Suit::Diamonds => "♦".into(),
            Suit::Hearts => "♥".into(),
            Suit::Spades => "♠".into(),
        }
    }
}

impl UnicodeDisplay for Rank {
    fn to_unicode(&self) -> String {
        match self {
            Rank::Ace => "A".into(),
            Rank::Two => "2".into(),
            Rank::Three => "3".into(),
            Rank::Four => "4".into(),
            Rank::Five => "5".into(),
            Rank::Six => "6".into(),
            Rank::Seven => "7".into(),
            Rank::Eight => "8".into(),
            Rank::Nine => "9".into(),
            Rank::Ten => "10".into(),
            Rank::Jack => "J".into(),
            Rank::Queen => "Q".into(),
            Rank::King => "K".into(),
            #[cfg(feature = "jokers")]
            Rank::Joker => "🃏".into(),
        }
    }
}

impl UnicodeDisplay for Card {
    fn to_unicode(&self) -> String {
        #[cfg(feature = "jokers")]
        if self.rank == Rank::Joker {
            return "🃏".into();
        }

        match (self.rank.to_unicode(), self.suit) {
            (rank, Some(suit)) => format!("{rank}{}", suit.to_unicode()),
            _ => String::from("?"),
        }
    }
}