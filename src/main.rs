use std::fmt;
use std::fmt::Write;
use std::cmp::Ordering;
use std::cmp;
extern crate rand;
mod cardlib;
use cardlib::*;


struct Hand {
    cards : Vec<Card>,
    upcard : Card,
}

impl Hand {
    fn new( c0 : Card, c1 : Card, c2 : Card) -> Self {
        let mut cv = vec![c0, c1, c2];
        cv.sort();
        Hand { cards : cv, upcard : c0 }
    }

    fn get_upcard_rank(&self) -> Rank {
        self.upcard.rank
    }

    fn is_3ofkind(&self) -> bool {
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

use rand::{thread_rng, Rng};

fn main() {
    use Suit::*;
    use Rank::*;

    let mut deck = [
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
        println!("Card is {}", card.to_string());
    }

    let mut rng = thread_rng();
    println!("After shuffling:");
    rng.shuffle(&mut deck);

    for card in deck.iter() {
        println!("Card is {}", card);
    }

    //let hands = vec![Hand::new(deck[0],deck[1],deck[2])];

    let mut hands = Vec::new();

    let mut card_num = 0;

    // Seven players
    for _ in 0..7 {
        hands.push(Hand::new(deck[card_num], deck[card_num+1], deck[card_num+2]));
        card_num += 3;
    }

    println!("Showing:\n");

    let mut force_card  = Two;
    for (i, h) in hands.iter().enumerate() {
        println!("Player {} shows {}", i, h.get_upcard_rank());
        force_card = cmp::max(force_card, h.get_upcard_rank());
    }

    println!("Force card is {}", force_card );

    let mut forced_players = Vec::new();

    for (i, h) in hands.iter().enumerate() {
        if h.get_upcard_rank() == force_card {
            forced_players.push(i);
        }
    }

    println!("Forced players are: ");
    for i in forced_players {
        println!("{}", i);
    }



}