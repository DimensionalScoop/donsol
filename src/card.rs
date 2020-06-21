use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Card {
    suit: Suit,
    face: Face,
    value: usize,
}

impl Card {
    pub fn name(&self) -> String {
        let names_by_suit = match self.suit {
            Suit::Clubs => [
                "Rat", "Bat", "Imp", "Goblin", "Orc", "Ogre", "Beholder", "Medusa", "Demon",
                "Consort", "Queen", "Regnant", "Empress",
            ],
            Suit::Spades => [
                "Slime", // 2
                "Rustacean",
                "Rust Mite",
                "Fiend",
                "Drake",
                "Specter",
                "Ghost",
                "Elemental",
                "Witch", // 10
                "Consort",
                "Queen",
                "Regnant",
                "Empress",
            ],
            Suit::Diamonds => [
                "Buckler",
                "Buckler",
                "Buckler",
                "Medium Shield",
                "Medium Shield",
                "Medium Shield",
                "Scutum",
                "Scutum",
                "Scutum",
                "Red Mage",
                "Red Mage",
                "Red Mage",
                "Red Mage",
            ],
            Suit::Hearts => [
                "Bandages",
                "Bandages",
                "Bandages",
                "Potion",
                "Potion",
                "Potion",
                "Healing Scroll",
                "Healing Scroll",
                "Healing Scroll",
                "White Mage",
                "White Mage",
                "White Mage",
                "White Mage",
            ],
            Suit::Blank => ["Donsol"; 13],
        };

        match self.face {
            Face::Number(num) => names_by_suit[num - 2], // lowest card value is 2
            Face::Jack => names_by_suit[11 - 2],
            Face::Queen => names_by_suit[12 - 2],
            Face::King => names_by_suit[13 - 2],
            Face::Ace => names_by_suit[14 - 2],
            Face::Joker => "Donsol",
        }
        .to_string()
    }

    pub fn generate_donsol_deck() -> Vec<Card> {
        let mut pile = Vec::with_capacity(52);

        for suit in &[Suit::Clubs,Suit::Spades,Suit::Hearts,Suit::Diamonds]{
            let values_of_suit = match suit {
                Suit::Clubs | Suit::Spades => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 16, 18],
                Suit::Hearts | Suit::Diamonds => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 11, 11],
                Suit::Blank => panic!("this is handled blow??")
            };

            for value in &values_of_suit {
                let face = match value {
                    18 => Face::Ace,
                    15 => Face::King,
                    13 => Face::Queen,
                    11 => Face::Jack,
                    2..=10 => Face::Number(*value),
                    _ => panic!("not a valid Donsol card value")
                };

                let card = Card{
                    suit:*suit,
                    value:*value,
                    face
                };
                pile.push(card)
            }
        }

        let donsol = Card {
            suit: Suit::Blank,
            value: 21,
            face: Face::Joker,
        };
        pile.push(donsol.clone());
        pile.push(donsol);

        assert_eq!(pile.len(), 52);
        pile
    }
}

#[derive(Clone, Debug)]
enum Face {
    Number(usize),
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match self {
            Face::Number(num) => num.to_string(),
            Face::Jack => "Jack".into(),
            Face::Queen => "Queen".into(),
            Face::King => "King".into(),
            Face::Ace => "Ace".into(),
            Face::Joker => "Joker".into(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    Blank, // Only Jokers are blank
}