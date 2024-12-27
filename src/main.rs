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
        .sort_by(|a, b| a.rank.as_i32().cmp(&b.rank.as_i32()));

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

impl TryFrom<i32> for Rank {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ace),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Jack),
            12 => Ok(Self::Queen),
            13 => Ok(Self::King),
            _ => Err(format!("Invalid rank value: {}", value))
        }
    }
}

impl From<Rank> for i32 {
    fn from(val: Rank) -> Self {
        match val {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

impl Rank {
    pub fn as_i32(self) -> i32 {
        i32::from(self)
    }
}

// Define a struct to represent the deck
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck with 52 cards
    fn new() -> Deck {
        let mut deck = Deck { cards: Vec::new() };

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.cards.push(Card { suit, rank });
            }
        }

        deck
    }

    /// Shuffle the deck
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deal a card from the top of the deck
    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
enum PokerHand {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
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
        for (_, amount) in suits {
            if amount >= 5 {
                has_flush = true;
                break;
            }
        }

        let mut pairs = 0;
        let mut has_three_of_a_kind = false;
        let mut has_four_of_a_kind = false;
        for (_, amount) in ranks {
            if amount == 4 {
                has_four_of_a_kind = true;
            } else if amount == 3 {
                has_three_of_a_kind = true;
            }

            if amount >= 2 {
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

                if current_highest.rank.as_i32() - next_card.rank.as_i32() == -1 {
                    sequential_cards += 1;
                    current_highest = next_card;
                } else {
                    sequential_cards = 1;
                }
            }
        }

        let mut best_hand = PokerHand::HighCard;

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
