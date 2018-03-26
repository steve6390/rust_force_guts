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

struct HandStats {
    count : u64,
    forces : u64,
    folds : u64,
    wins : u64,
}

impl HandStats {
    pub fn new() -> Self {
        HandStats {
            count : 0,
            forces : 0,
            folds : 0,
            wins : 0,
        }
    }
}

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

    let mut rng = thread_rng();
    rng.shuffle(&mut deck);

    let mut hands_played = 0;
    let mut hand_stats = BTreeMap::new();
    let mut hands = Vec::new();

    let start = PreciseTime::now();
    while hands_played < 1_000_000 {
        rng.shuffle(&mut deck);

        {
            // card_num tracks the opint from which we're dealing the deck.
            let mut card_num = 0;
            hands.clear();
            // Seven players
            for _ in 0..7 {
                hands_played += 1;
                hands.push(Hand::new(deck[card_num], deck[card_num+1], deck[card_num+2]));
                card_num += 3;
            }
        }

        // Determine the rank of the forced card.
        let force_rank = hands.iter().fold(0, |force_rank, h|
                cmp::max(force_rank, h.get_upcard_rank()));

        for h in hands.iter_mut() {
            let hr = h.get_rank();
            let stats = hand_stats.entry(hr).or_insert(HandStats::new());
            stats.count += 1;

            // check if forced
            // if not forced, then we have the option to fold.
            // If the hand is worse than the force card, fold is automatic.
            if h.get_upcard_rank() == force_rank {
                h.set_forced();
                stats.forces += 1;
            } else if hr <= force_rank {
                h.set_folded();
                stats.folds += 1;
            }
        }

        // All folding algorithms are complete by this point
        // Determine the winning hand rank.
        let win_rank = hands.iter()
                .filter(|h| !h.is_folded())
                .fold(0, |win_rank, h| cmp::max(win_rank, h.get_rank()));

        for h in hands.iter_mut() {
            if h.get_rank() == win_rank {
                if let Some(stats) = hand_stats.get_mut(&win_rank) {
                    stats.wins += 1;
                }
            }
        }
    }

    let stop = PreciseTime::now();

    for (hr, stats) in &hand_stats {
        let r = stats.wins as f64 / (stats.count - stats.folds) as f64;
        println!("0x{:X} dealt {} times, won {} times, forced {} times, folded {} times.  Winning odds {:.4}%",
                hr, stats.count, stats.wins, stats.forces, stats.folds, r * 100.0);
    }
    println!("Executed in {}", start.to(stop));
}