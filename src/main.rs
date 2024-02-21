use std::collections::HashMap;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    deck.shuffle();

    for _ in 0..10 {
        hand.add_card(deck.deal().expect("failed to deal a card from the deck!"));
    }

    hand.cards
        .sort_by(|a, b| a.rank.to_i32().cmp(&b.rank.to_i32()));

    hand.show();
    println!();
    hand.analyze_hand();
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Rank {
    Ace,
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
}

impl Suit {
    fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
            .iter()
            .cloned()
    }
}

impl Rank {
    fn iter() -> impl Iterator<Item = Rank> {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
        .iter()
        .cloned()
    }
}

// Define a struct to represent a single card
#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

impl From<i32> for Rank {
    fn from(n: i32) -> Self {
        if let Some(rank) = Rank::iter().nth((n - 1) as usize) {
            rank
        } else {
            eprintln!("integer could not be converted into a Rank! returning an Ace");
            Rank::Ace
        }
    }
}

impl Rank {
    fn to_i32(self) -> i32 {
        if let Some(rank) = Rank::iter().position(|rank| rank == self) {
            rank as i32 + 1
        } else {
            eprintln!("Rank could not be converted into an integer! returning 1.");
            1
        }
    }
}

// Define a struct to represent the deck
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Create a new deck with 52 cards
    fn new() -> Deck {
        let mut deck = Deck { cards: Vec::new() };

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.cards.push(Card { suit, rank });
            }
        }

        deck
    }

    // Shuffle the deck
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // Deal a card from the top of the deck
    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
enum PokerHand {
    None,
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
}

impl Hand {
    fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn show(&self) {
        println!("My hand has:");
        for card in &self.cards {
            println!("The {card}");
        }
    }

    fn analyze_hand(&self) {
        let cards = self.cards.clone();

        // Check for Flush
        let mut suits: HashMap<Suit, i32> = HashMap::new();
        let mut ranks: HashMap<Rank, i32> = HashMap::new();
        for card in cards.iter() {
            *suits.entry(card.suit).or_insert(0) += 1;
            *ranks.entry(card.rank).or_insert(0) += 1;
        }

        let mut has_flush = false;
        for suit in suits.keys() {
            if *suits.get(suit).expect("failed to get suit!") >= 5 {
                has_flush = true;
                break;
            }
        }

        let mut pairs = 0;
        let mut has_three_of_a_kind = false;
        let mut has_four_of_a_kind = false;
        for rank in ranks.keys() {
            let n_rank = *ranks.get(rank).expect("failed to get rank!");
            if n_rank == 4 {
                has_four_of_a_kind = true;
            } else if n_rank == 3 {
                has_three_of_a_kind = true;
            }

            if n_rank >= 2 {
                pairs += 1;
            }
        }

        let mut sequential_cards = 1;
        for mut current_highest in cards.iter() {
            for next_card in cards.iter() {
                if sequential_cards == 5 {
                    break;
                }

                if current_highest.rank == next_card.rank {
                    continue;
                }

                if current_highest.rank.to_i32() - next_card.rank.to_i32() == -1 {
                    sequential_cards += 1;
                    current_highest = next_card;
                } else {
                    sequential_cards = 1;
                }
            }
        }

        let mut best_hand = PokerHand::None;

        if pairs == 1 {
            println!("This hand has a one pair");
            best_hand = PokerHand::OnePair;
        } else if pairs >= 2 {
            println!("This hand has a two pair");
            best_hand = PokerHand::TwoPair;
        }

        if has_three_of_a_kind {
            println!("This hand has a three of a kind");
            best_hand = PokerHand::ThreeOfAKind;
        }

        if sequential_cards == 5 {
            println!("This hand has a straight");
            best_hand = PokerHand::Straight;
        }

        if has_flush {
            println!("This hand has a flush");
            best_hand = PokerHand::Flush;
        }

        if pairs > 1 && has_three_of_a_kind || has_four_of_a_kind {
            println!("This hand has a full house");
            best_hand = PokerHand::FullHouse;
        }

        if has_four_of_a_kind {
            println!("This hand has a four of a kind");
            best_hand = PokerHand::FourOfAKind;
        }

        println!("\nThe best available hand is: {:?}", best_hand)
    }
}
