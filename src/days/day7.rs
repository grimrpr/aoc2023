use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Represents each Card value
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
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
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Unknown Card value."),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl AsRef<HandType> for Hand {
    fn as_ref(&self) -> &HandType {
        let mut cards_count = [0u8; (Card::Ace as usize + 1)];
        for card in self.cards {
            cards_count[card as usize] += 1;
        }

        let mut heap = BinaryHeap::from(cards_count);
        match heap.pop() {
            Some(5) => &HandType::FiveOfAKind,
            Some(4) => &HandType::FourOfAKind,
            Some(3) => match heap.pop() {
                Some(2) => &HandType::FullHouse,
                _ => &HandType::ThreeOfAKind,
            },
            Some(2) => match heap.pop() {
                Some(2) => &HandType::TwoPair,
                _ => &HandType::OnePair,
            },
            _ => &HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs_hand_type: &HandType = self.as_ref();
        let rhs_hand_type: &HandType = other.as_ref();
        match lhs_hand_type.cmp(&rhs_hand_type) {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let order = self.cards[i].cmp(&other.cards[i]);
                    if order != Ordering::Equal {
                        return Some(order);
                    }
                }
                Some(Ordering::Equal)
            }
            ordering => Some(ordering),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = value
            .chars()
            .map(Card::try_from)
            .filter_map(Result::ok)
            .enumerate()
            .try_fold(
                (
                    0usize,
                    Hand {
                        cards: [Card::Ace; 5],
                    },
                ),
                |(_, mut hand), (idx, card)| {
                    if idx < hand.cards.len() {
                        hand.cards[idx] = card;
                        return Ok((idx, hand));
                    } else {
                        return Err("Too many Cards for Hand.");
                    }
                },
            )?;
        if res.0 != res.1.cards.len() - 1 {
            return Err("Not enough Cards for Hand.");
        }
        Ok(res.1)
    }
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day7").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let hands_n_bids = BTreeMap::from_iter(
        lines
            .iter()
            .filter_map(|line| line.trim().split_once(char::is_whitespace))
            .map(|(hand, bid)| (Hand::try_from(hand).unwrap(), bid.parse::<u64>().unwrap())),
    );

    let sum_rank_mul_bids: u64 = hands_n_bids
        .values()
        .zip(1..)
        .map(|(bid, rank)| bid * rank)
        .sum();

    println!("Sum of bid rank products: {}", sum_rank_mul_bids);
}
