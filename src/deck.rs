use rand::rng;
use rand::seq::SliceRandom;

static SUITS: [Suit; 4] = [
    Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds
];

static RANKS: [Rank; 13] = [
    Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
    Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
    Rank::Jack, Rank::Queen, Rank::King
];

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: Vec::new()
        }
    }

    pub fn draw(&mut self) -> Card {
        if self.cards.is_empty() {
            self.populate();
        }

        self.cards.remove(0)
    }

    fn populate(&mut self) {
        for suit in SUITS {
            for rank in RANKS {
                let card = Card {
                    suit, rank
                };

                self.cards.push(card);
            }
        }

        self.cards.shuffle(&mut rng());
    }
}

pub struct Card {
    suit: Suit,
    pub rank: Rank
}

impl Card {
    pub fn name(&self) -> String {
        format!("{}{}", self.suit.symbol(), self.rank.name())
    }
}

#[derive(Copy, Clone)]
enum Suit {
    Spades, Clubs, Hearts, Diamonds
}

impl Suit {
    const fn symbol(self) -> &'static str {
        match self {
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦"
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Rank {
    Ace, Two, Three, Four, Five,
    Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King
}

impl Rank {
    const fn name(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K"
        }
    }

    pub const fn value(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}