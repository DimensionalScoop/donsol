use std::string::ToString;

#[derive(Clone, PartialOrd, Eq, PartialEq, Ord, Debug)]
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

        let mut all_faces = Vec::with_capacity(13);
        for n in 2..11 {
            all_faces.push(Face::Number(n));
        }
        all_faces.push(Face::Jack);
        all_faces.push(Face::Queen);
        all_faces.push(Face::King);
        all_faces.push(Face::Ace);

        for suit in &[Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds] {
            let values_of_suit = match suit {
                Suit::Clubs | Suit::Spades => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 16, 18],
                Suit::Hearts | Suit::Diamonds => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 11, 11],
                Suit::Blank => panic!("this is handled blow??"),
            };

            let both = values_of_suit.iter().zip(all_faces.iter());
            for (value, face) in both {
                let card = Card {
                    suit: *suit,
                    value: *value,
                    face: face.clone(),
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

        pile
    }
}

#[derive(Clone, PartialOrd, Eq, PartialEq, Ord, Debug)]
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

#[derive(Copy, PartialOrd, Eq, PartialEq, Ord, Clone, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    Blank, // Only Jokers are blank
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_names() {
        let card = Card {
            suit: Suit::Hearts,
            face: Face::King,
            value: 18,
        };
        assert_eq!(card.name(), "White Mage".to_string());

        let card = Card {
            suit: Suit::Spades,
            face: Face::King,
            value: 18,
        };
        assert_eq!(card.name(), "Regnant".to_string());

        let card = Card {
            suit: Suit::Spades,
            face: Face::Number(3),
            value: 3,
        };
        assert_eq!(card.name(), "Rustacean".to_string());

        let card = Card {
            suit: Suit::Diamonds,
            face: Face::Number(5),
            value: 5,
        };
        assert_eq!(card.name(), "Medium Shield".to_string());
    }

    #[test]
    fn donsol_deck() {
        let mut deck = Card::generate_donsol_deck();

        // should be a whole deck
        assert_eq!(deck.len(), 54);

        // every card should be named
        for c in &deck {
            assert_ne!(c.name(), "")
        }

        // only the Joker should occur twice
        // every other card should be unique
        deck.sort_unstable();
        deck.dedup();
        assert_eq!(deck.len(), 53);
    }
}
