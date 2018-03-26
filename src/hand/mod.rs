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
        // hands are always sorted, c0 = lowest card, c2 = highest card
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

    /*
     Rank of 3-of-kind is triple << 16
     Rank of 2-of-kind is pair << 12 + kicker
     Ranks of high card is (Highest << 8) + (middle << 4) + lowest
    */
    pub fn get_hand_rank(&self) -> u32 {
        if self.is_3ofkind() {
            return (self.cards[0].rank as u32) << 16;
        }
        if self.is_2ofkind() {
            // hands are sorted by rank, so either c0==c1 or c1==c2
            if self.cards[0].rank == self.cards[1].rank {
                return ((self.cards[0].rank as u32) << 12)
                       + self.cards[2].rank as u32; // c2 is kicker
            } else {
                return ((self.cards[1].rank as u32) << 12)
                       + self.cards[0].rank as u32; // c0 is kicker
            }
        }

        // hands are sorted, c2 is always highest card
        return ((self.cards[2].rank as u32) << 8)
             + ((self.cards[1].rank as u32) << 4)
             +   self.cards[0].rank as u32;
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
