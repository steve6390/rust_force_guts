use std::cmp;
extern crate rand;
extern crate time;
mod hand;
use hand::Hand;
use hand::card::Card;
use hand::card::Suit;
use hand::card::Rank;

use rand::{thread_rng, Rng};
use time::PreciseTime;
use std::collections::BTreeMap;

fn main() {
    let mut deck = [
        Card { rank : Rank::Two  , suit : Suit::Heart },
        Card { rank : Rank::Three, suit : Suit::Heart },
        Card { rank : Rank::Four , suit : Suit::Heart },
        Card { rank : Rank::Five , suit : Suit::Heart },
        Card { rank : Rank::Six  , suit : Suit::Heart },
        Card { rank : Rank::Seven, suit : Suit::Heart },
        Card { rank : Rank::Eight, suit : Suit::Heart },
        Card { rank : Rank::Nine , suit : Suit::Heart },
        Card { rank : Rank::Ten  , suit : Suit::Heart },
        Card { rank : Rank::Jack , suit : Suit::Heart },
        Card { rank : Rank::Queen, suit : Suit::Heart },
        Card { rank : Rank::King , suit : Suit::Heart },
        Card { rank : Rank::Ace  , suit : Suit::Heart },
        Card { rank : Rank::Two  , suit : Suit::Club },
        Card { rank : Rank::Three, suit : Suit::Club },
        Card { rank : Rank::Four , suit : Suit::Club },
        Card { rank : Rank::Five , suit : Suit::Club },
        Card { rank : Rank::Six  , suit : Suit::Club },
        Card { rank : Rank::Seven, suit : Suit::Club },
        Card { rank : Rank::Eight, suit : Suit::Club },
        Card { rank : Rank::Nine , suit : Suit::Club },
        Card { rank : Rank::Ten  , suit : Suit::Club },
        Card { rank : Rank::Jack , suit : Suit::Club },
        Card { rank : Rank::Queen, suit : Suit::Club },
        Card { rank : Rank::King , suit : Suit::Club },
        Card { rank : Rank::Ace  , suit : Suit::Club },
        Card { rank : Rank::Two  , suit : Suit::Diamond },
        Card { rank : Rank::Three, suit : Suit::Diamond },
        Card { rank : Rank::Four , suit : Suit::Diamond },
        Card { rank : Rank::Five , suit : Suit::Diamond },
        Card { rank : Rank::Six  , suit : Suit::Diamond },
        Card { rank : Rank::Seven, suit : Suit::Diamond },
        Card { rank : Rank::Eight, suit : Suit::Diamond },
        Card { rank : Rank::Nine , suit : Suit::Diamond },
        Card { rank : Rank::Ten  , suit : Suit::Diamond },
        Card { rank : Rank::Jack , suit : Suit::Diamond },
        Card { rank : Rank::Queen, suit : Suit::Diamond },
        Card { rank : Rank::King , suit : Suit::Diamond },
        Card { rank : Rank::Ace  , suit : Suit::Diamond },
        Card { rank : Rank::Two  , suit : Suit::Spade },
        Card { rank : Rank::Three, suit : Suit::Spade },
        Card { rank : Rank::Four , suit : Suit::Spade },
        Card { rank : Rank::Five , suit : Suit::Spade },
        Card { rank : Rank::Six  , suit : Suit::Spade },
        Card { rank : Rank::Seven, suit : Suit::Spade },
        Card { rank : Rank::Eight, suit : Suit::Spade },
        Card { rank : Rank::Nine , suit : Suit::Spade },
        Card { rank : Rank::Ten  , suit : Suit::Spade },
        Card { rank : Rank::Jack , suit : Suit::Spade },
        Card { rank : Rank::Queen, suit : Suit::Spade },
        Card { rank : Rank::King , suit : Suit::Spade },
        Card { rank : Rank::Ace  , suit : Suit::Spade },
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

    let mut hands_played = 0;
    let mut hand_stats = BTreeMap::new();
    let mut hands = Vec::new();

    let start = PreciseTime::now();
    while hands_played < 1_000_000 {
        rng.shuffle(&mut deck);
        //println!("Hand number {}", hands_played);
        hands.clear();

        let mut card_num = 0;

        // Seven players
        for _ in 0..7 {
            hands_played += 1;
            hands.push(Hand::new(deck[card_num], deck[card_num+1], deck[card_num+2]));
            card_num += 3;
        }

        //println!("Showing:\n");

        let mut force_card = Rank::Two;
        for (i, h) in hands.iter().enumerate() {
            //println!("Player {} shows {}", i, h.get_upcard_rank());
            force_card = cmp::max(force_card, h.get_upcard_rank());
        }

        //println!("Force card is {}", force_card );

        let mut forced_players = Vec::new();

        for (i, h) in hands.iter().enumerate() {
            if h.get_upcard_rank() == force_card {
                forced_players.push(i);
            }
            let hr = h.get_hand_rank();
            let count = hand_stats.entry(hr).or_insert(0);
            *count += 1;
            /*
            if h.is_3ofkind() {
                count_3ofkind += 1;
                println!("Hand number {}: Player {} has 3 of a kind! {}. [{}]",
                        hands_played, i, h, count_3ofkind);
            */
        }
/*
        println!("Forced players are: ");
        for i in forced_players {
            println!("{}", i);
        }
*/
    }
    let stop = PreciseTime::now();

    for (hr, count) in &hand_stats {
        let r = *count as f64 / hands_played as f64;
        println!("0x{:X} happend {} times.  Odds {:.4}%", hr, *count, r * 100.0);
    }
    println!("Executed in {}", start.to(stop));
}