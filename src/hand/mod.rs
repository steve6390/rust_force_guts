use std::fmt;
use std::fmt::Write;
extern crate rand;
pub mod card;
pub use self::card::Card;
pub use self::card::Suit;
pub use self::card::Rank;



pub struct Hand {
    cards : Vec<Card>,
    upcard : Card,
}

impl Hand {
    pub fn new( c0 : Card, c1 : Card, c2 : Card) -> Self {
        let mut cv = vec![c0, c1, c2];
        cv.sort_unstable();
        Hand { cards : cv, upcard : c0 }
    }

    pub fn get_upcard_rank(&self) -> Rank {
        self.upcard.rank
    }

    pub fn is_2ofkind(&self) -> bool {
        (self.cards[0].rank == self.cards[1].rank) ||
        (self.cards[1].rank == self.cards[2].rank)
    }

    pub fn is_3ofkind(&self) -> bool {
        (self.cards[0].rank == self.cards[1].rank) &&
        (self.cards[0].rank == self.cards[2].rank)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        (self.cards[0].rank == other.cards[0].rank) &&
        (self.cards[1].rank == other.cards[1].rank) &&
        (self.cards[2].rank == other.cards[2].rank)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for c in &self.cards {
            write!(&mut output, "{}", c.rank);
        }
        write!(f, "{}", output)
    }
}
