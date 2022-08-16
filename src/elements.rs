use rand::{self, Rng};
use std::array::IntoIter;

#[derive(Copy, Clone)]
pub enum Suit {
    Hearts,
    Tiles,
    Clovers,
    Pikes,
}

fn suit_to_char(_suit: &Suit) -> char {
    match _suit {
        Suit::Hearts => '♥',
        Suit::Tiles => '♦',
        Suit::Clovers => '♣',
        Suit::Pikes => '♠',
    }
}

fn char_to_suit(_char: char) -> Option<Suit> {
    match _char {
        '♥' => Some(Suit::Hearts),
        '♦' => Some(Suit::Tiles),
        '♣' => Some(Suit::Clovers),
        '♠' => Some(Suit::Pikes),
        _ => None,
    }
}

#[derive(Copy, Clone)]
pub enum CardType {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Jack,
    Queen,
    King,
}

fn card_to_char(_card: &CardType) -> char {
    match _card {
        CardType::Ace => 'A',
        CardType::Two => '2',
        CardType::Three => '3',
        CardType::Four => '4',
        CardType::Five => '5',
        CardType::Six => '6',
        CardType::Seven => '7',
        CardType::Jack => 'J',
        CardType::Queen => 'Q',
        CardType::King => 'K',
    }
}

fn char_to_card(_char: char) -> Option<CardType> {
    match _char {
        'A' => Some(CardType::Ace),
        '2' => Some(CardType::Two),
        '3' => Some(CardType::Three),
        '4' => Some(CardType::Four),
        '5' => Some(CardType::Five),
        '6' => Some(CardType::Six),
        '7' => Some(CardType::Seven),
        'J' => Some(CardType::Jack),
        'Q' => Some(CardType::Queen),
        'K' => Some(CardType::King),
        _ => None,
    }
}

fn card_value(_card: &CardType) -> usize {
    match _card {
        CardType::Ace => 0,
        CardType::Two => 2,
        CardType::Three => 3,
        CardType::Four => 4,
        CardType::Five => 5,
        CardType::Six => 6,
        CardType::Seven => 7,
        CardType::Jack => 8,
        CardType::Queen => 9,
        CardType::King => 10,
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub card_type: CardType,
}

impl Card {
    pub fn to_string(&self) -> String {
        let mut _str: String = String::from("");
        _str.push(suit_to_char(&self.suit));
        _str.push(card_to_char(&self.card_type));
        _str
    }

    pub fn from_string(_card_str: String) -> Option<Card> {
        if _card_str.len() != 2 {
            None
        } else {
            let mut iter = _card_str.chars();
            let suit = char_to_suit(iter.next()?)?;
            let card_type = char_to_card(iter.next()?)?;
            Some(Card { suit, card_type })
        }
    }

    pub fn value(&self) -> usize {
        card_value(&self.card_type)
    }
}

pub struct Deck {
    cards: [Card; 40],
    stack_head: usize,
}

impl Deck {
    pub fn base_deck() -> Deck {
        Deck {
            cards: [
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Ace,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Two,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Three,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Four,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Five,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Six,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Seven,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Jack,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::Queen,
                },
                Card {
                    suit: Suit::Hearts,
                    card_type: CardType::King,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Ace,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Two,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Three,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Four,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Five,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Six,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Seven,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Jack,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::Queen,
                },
                Card {
                    suit: Suit::Tiles,
                    card_type: CardType::King,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Ace,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Two,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Three,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Four,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Five,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Six,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Seven,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Jack,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::Queen,
                },
                Card {
                    suit: Suit::Clovers,
                    card_type: CardType::King,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Ace,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Two,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Three,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Four,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Five,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Six,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Seven,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Jack,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::Queen,
                },
                Card {
                    suit: Suit::Pikes,
                    card_type: CardType::King,
                },
            ],
            stack_head: 0,
        }
    }

    pub fn iter(&self) -> IntoIter<Card, 40> {
        self.cards.into_iter()
    }

    // draw a card from the deck
    pub fn draw(&mut self) -> Option<Card> {
        if self.stack_head >= 40 {
            None
        } else {
            let card = self.cards[self.stack_head];
            self.stack_head += 1;
            Some(card)
        }
    }

    // put a card on top of the deck
    pub fn put(&mut self, card: Card) {
        if self.stack_head > 0 {
            self.stack_head -= 1;
            self.cards[self.stack_head] = card;
        }
    }

    pub fn shuffle(&mut self, iterations: usize) {
        // TODO: works only with 40 cards
        // Riffle Shuffle Algorithm
        let mut rng = rand::thread_rng();
        for _ in 0..iterations {
            // Generates a random float between 0 and 1
            // The float token is used to decide where the remaining deck is sliced
            let token: f64 = rng.gen();
            let slice_point = ((40 - self.stack_head) as f64 * token) as usize + self.stack_head;
            let old_cards: [Card; 40] = self.cards;
            let old_head = self.stack_head;
            self.stack_head = 40;
            let mut ptr_slice_1 = 0;
            let mut ptr_slice_2 = slice_point;
            while ptr_slice_1 < slice_point || ptr_slice_2 < 40 {
                if ptr_slice_2 < 40 {
                    self.put(old_cards[ptr_slice_2]);
                    ptr_slice_2 += 1;
                }
                if ptr_slice_1 < slice_point {
                    self.put(old_cards[ptr_slice_1]);
                    ptr_slice_1 += 1;
                }
            }
            self.stack_head = old_head;
        }
    }

    pub fn shuffle_new() -> Deck {
        let mut deck = Deck::base_deck();
        deck.shuffle(10);
        deck
    }
}
