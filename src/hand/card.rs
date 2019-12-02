use std::fmt;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Heart,
    Club,
    Diamond,
    Spade
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart   => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade   => write!(f, "♠"),
            Suit::Club    => write!(f, "♣"), 
       }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Two   => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four  => write!(f, "4"),
            Rank::Five  => write!(f, "5"),
            Rank::Six   => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine  => write!(f, "9"),
            Rank::Ten   => write!(f, "T"),
            Rank::Jack  => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King  => write!(f, "K"),
            Rank::Ace   => write!(f, "A"),
       }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit : Suit,
    pub rank : Rank
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

// Cards compare with each other only by rank
impl Ord for Card {
    fn cmp(&self, other : &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Cards compare with each other only by rank
impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}
