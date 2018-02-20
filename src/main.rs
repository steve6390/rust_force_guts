use std::fmt;

enum Suit {
    Heart,
    Club,
    Diamond,
    Spade
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
       }
    }
}

enum Rank {
    Two,
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
            Rank::Ten   => write!(f, "10"),
            Rank::Jack  => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King  => write!(f, "K"),
            Rank::Ace   => write!(f, "A"),
       }
    }
}

struct Card {
    suit : Suit,
    rank : Rank
}

impl Card {
    fn new( r : Rank, s : Suit ) -> Self {
        Self { rank : r, suit : s }
    }

    fn toString(&self) -> String {
        format!("{} of {}", self.rank, self.suit )
    }
}

fn main() {
    use Suit::*;
    use Rank::*;

    let deck = [
        Card { rank : Two  , suit : Heart },
        Card { rank : Three, suit : Heart },
        Card { rank : Four , suit : Heart },
        Card { rank : Five , suit : Heart },
        Card { rank : Six  , suit : Heart },
        Card { rank : Seven, suit : Heart },
        Card { rank : Eight, suit : Heart }, 
        Card { rank : Nine , suit : Heart },
        Card { rank : Ten  , suit : Heart },
        Card { rank : Jack , suit : Heart },
        Card { rank : Queen, suit : Heart },
        Card { rank : King , suit : Heart },
        Card { rank : Ace  , suit : Heart },
        Card { rank : Two  , suit : Club },
        Card { rank : Three, suit : Club },
        Card { rank : Four , suit : Club },
        Card { rank : Five , suit : Club },
        Card { rank : Six  , suit : Club },
        Card { rank : Seven, suit : Club },
        Card { rank : Eight, suit : Club },
        Card { rank : Nine , suit : Club },
        Card { rank : Ten  , suit : Club },
        Card { rank : Jack , suit : Club },
        Card { rank : Queen, suit : Club },
        Card { rank : King , suit : Club },
        Card { rank : Ace  , suit : Club },
        Card { rank : Two  , suit : Diamond },
        Card { rank : Three, suit : Diamond },
        Card { rank : Four , suit : Diamond },
        Card { rank : Five , suit : Diamond },
        Card { rank : Six  , suit : Diamond },
        Card { rank : Seven, suit : Diamond },
        Card { rank : Eight, suit : Diamond },
        Card { rank : Nine , suit : Diamond },
        Card { rank : Ten  , suit : Diamond },
        Card { rank : Jack , suit : Diamond },
        Card { rank : Queen, suit : Diamond },
        Card { rank : King , suit : Diamond },
        Card { rank : Ace  , suit : Diamond },
        Card { rank : Two  , suit : Spade },
        Card { rank : Three, suit : Spade },
        Card { rank : Four , suit : Spade },
        Card { rank : Five , suit : Spade },
        Card { rank : Six  , suit : Spade },
        Card { rank : Seven, suit : Spade },
        Card { rank : Eight, suit : Spade },
        Card { rank : Nine , suit : Spade },
        Card { rank : Ten  , suit : Spade },
        Card { rank : Jack , suit : Spade },
        Card { rank : Queen, suit : Spade },
        Card { rank : King , suit : Spade },
        Card { rank : Ace  , suit : Spade },
    ];

    println!("Force Guts 0.1");
    for card in deck.iter() {
        println!("Card is {}", card.toString());
    }
}
