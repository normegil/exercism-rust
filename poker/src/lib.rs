use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_vec: Vec<Hand> = Vec::new();
    for hand in hands {
        match Hand::new(hand) {
            Ok(h) => hands_vec.push(h),
            Err(e) => panic!("{}", e)
        }
    }
    hands_vec.sort();

    let winning_hand = &hands_vec[0];
    hands_vec.iter()
        .filter(|h| h == &winning_hand)
        .map(|h| h.representation)
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds
}

#[derive(PartialEq, Eq, Debug)]
struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    fn new(representation: &str) -> Result<Card, String> {
        let nb_chars = representation.chars().count();
        let value: String = representation.chars().take(nb_chars - 1).collect();
        let value = match Value::new(&value) {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        let suit_repr = representation.chars().last().unwrap();
        match suit_repr {
            'H' => Result::Ok(Card { suit: Suit::Hearts, value: value }),
            'S' => Result::Ok(Card { suit: Suit::Spades, value: value }),
            'C' => Result::Ok(Card { suit: Suit::Clubs, value: value }),
            'D' => Result::Ok(Card { suit: Suit::Diamonds, value: value }),
            _ => Result::Err(format!("'{}' suit not recognized", suit_repr))
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    } 
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    } 
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Value {
    fn new(representation: &str) -> Result<Value, String> {
        match representation {
            "1" => Result::Ok(Value::Ace),
            "K" => Result::Ok(Value::King),
            "Q" => Result::Ok(Value::Queen),
            "J" => Result::Ok(Value::Jack),
            "10" => Result::Ok(Value::Ten),
            "9" => Result::Ok(Value::Nine),
            "8" => Result::Ok(Value::Eight),
            "7" => Result::Ok(Value::Seven),
            "6" => Result::Ok(Value::Six),
            "5" => Result::Ok(Value::Five),
            "4" => Result::Ok(Value::Four),
            "3" => Result::Ok(Value::Three),
            "2" => Result::Ok(Value::Two),
            _ => Result::Err(format!("'{}' value not recognized", representation)) 
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair(Value, Value),
    OnePair(Value),
    HighCard(Value),
}

#[derive(Debug, Eq)]
struct Hand<'a> {
    representation: &'a str,
    hand: Vec<Card>
}

impl<'a> Hand<'a> {
    fn new(representation: &str) -> Result<Hand, String> {
        let mut hand: Vec<Card> = Vec::new();
        for card in representation.split_whitespace() {
            match Card::new(card) {
                Ok(suit) => hand.push(suit),
                Err(e) => return Err(e)
            }    
        }
        hand.sort();
        Result::Ok(Hand { representation, hand })
    }

    fn best_hand(&self) -> Combination {
        if self.is_five_of_a_kind() {
            return Combination::FiveOfAKind;
        } else if let Option::Some((a, b)) = self.highest_two_pair() {
            return Combination::TwoPair(a, b);
        } else if let Option::Some(c) = self.highest_pair() {
            return Combination::OnePair(c);
        } else {
            return Combination::HighCard(self.highest_value())
        }
    }

    fn is_five_of_a_kind(&self) -> bool {
        let x = &self.hand[0];
        self.hand.iter().all(|y| y == x)
    }

    fn highest_two_pair(&self) -> Option<(Value, Value)> {
        let mut found_pair: Option<Value> = Option::None;
        for (index, c) in self.hand.iter().enumerate() {
            let count = self.hand[index..].iter()
                .filter(|y|c.value == y.value)
                .count();
            if count == 2 {
                match found_pair {
                    None => found_pair = Option::Some(c.value),
                    Some(first_pair) => {
                        let mut pairs = vec![first_pair, c.value];
                        pairs.sort();
                        return Option::Some((pairs[0], pairs[1]))
                    },
                }
            }
        }
        return Option::None;
    }

    fn highest_pair(&self) -> Option<Value> {
        for (index, c) in self.hand.iter().enumerate() {
            let count = self.hand[index..].iter()
                .filter(|y|c.value == y.value)
                .count();
            if count == 2 {
                return Option::Some(c.value);
            }
        }
        return Option::None;
    }

    // fn is_straight_flush(&self) -> bool {
    //     let x = &self.hand[0];
    //     self.hand.iter().all(|y| y == x)
    // }

    fn highest_value(&self) -> Value {
        self.hand[0].value
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.best_hand().cmp(&other.best_hand())
    } 
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    } 
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        for (index, c) in self.hand.iter().enumerate() {
            if c.value != other.hand[index].value {
                return false;
            }
        }
        return true;
    }
}