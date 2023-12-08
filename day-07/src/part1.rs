use std::{
    cmp::{Ord, Ordering},
    collections::BTreeMap,
    convert::TryInto,
};

use itertools::Itertools;
use nom::{
    self,
    character::complete::{self, anychar, space1},
    sequence::tuple,
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
struct Card {
    #[allow(dead_code)]
    label: char,
    value: u64,
}

impl Card {
    fn new(label: char) -> Result<Self, AocError> {
        match label {
            'A' => Ok(Self { label, value: 0xE }),
            'K' => Ok(Self { label, value: 0xD }),
            'Q' => Ok(Self { label, value: 0xC }),
            'J' => Ok(Self { label, value: 0xB }),
            'T' => Ok(Self { label, value: 0xA }),
            '9' => Ok(Self { label, value: 0x9 }),
            '8' => Ok(Self { label, value: 0x8 }),
            '7' => Ok(Self { label, value: 0x7 }),
            '6' => Ok(Self { label, value: 0x6 }),
            '5' => Ok(Self { label, value: 0x5 }),
            '4' => Ok(Self { label, value: 0x4 }),
            '3' => Ok(Self { label, value: 0x3 }),
            '2' => Ok(Self { label, value: 0x2 }),
            other => Err(AocError::BadLabelError(other.to_string())),
        }
    }
}

enum HType {
    HighCard = 0x1_00_00_00_00_00,
    OnePair = 0x2_00_00_00_00_00,
    TwoPair = 0x3_00_00_00_00_00,
    ThreeOfAKind = 0x4_00_00_00_00_00,
    FullHouse = 0x5_00_00_00_00_00,
    FourOfAKind = 0x6_00_00_00_00_00,
    FiveOfAKind = 0x7_00_00_00_00_00,
}

impl TryInto<u64> for HType {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            HType::HighCard => Ok(HType::HighCard as u64),
            HType::OnePair => Ok(HType::OnePair as u64),
            HType::TwoPair => Ok(HType::TwoPair as u64),
            HType::ThreeOfAKind => Ok(HType::ThreeOfAKind as u64),
            HType::FullHouse => Ok(HType::FullHouse as u64),
            HType::FourOfAKind => Ok(HType::FourOfAKind as u64),
            HType::FiveOfAKind => Ok(HType::FiveOfAKind as u64),
        }
    }
}

#[derive(Debug)]
struct Hand {
    #[allow(dead_code)]
    cards: [Card; 5],
    bid: u32,
    value: u64,
}

impl Hand {
    fn new(cards: &[Card; 5], bid: u32) -> Self {
        // Determine the type of this hand
        let mut bins: BTreeMap<u64, u8> = BTreeMap::new();

        for card in cards {
            if let Some(occurs) = bins.get(&card.value) {
                bins.insert(card.value, occurs + 1);
            } else {
                bins.insert(card.value, 1);
            }
        }

        let bins = bins.values().sorted().rev().collect_vec();

        // TODO: Somewhat ugly to have to do this without a match clause. Try better!
        let mut value: u64 = if bins == vec![&5u8] {
            HType::FiveOfAKind.try_into().unwrap()
        } else if bins == vec![&4u8, &1u8] {
            HType::FourOfAKind.try_into().unwrap()
        } else if bins == vec![&3u8, &2u8] {
            HType::FullHouse.try_into().unwrap()
        } else if bins == vec![&3u8, &1u8, &1u8] {
            HType::ThreeOfAKind.try_into().unwrap()
        } else if bins == vec![&2u8, &2u8, &1u8] {
            HType::TwoPair.try_into().unwrap()
        } else if bins == vec![&2u8, &1u8, &1u8, &1u8] {
            HType::OnePair.try_into().unwrap()
        } else {
            HType::HighCard.try_into().unwrap()
        };

        value += &cards[0].value * 0x1_00_00_00_00;
        value += &cards[1].value * 0x1_00_00_00;
        value += &cards[2].value * 0x1_00_00;
        value += &cards[3].value * 0x1_00;
        value += &cards[4].value * 0x1;

        Self {
            cards: cards.to_owned(),
            bid,
            value,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let winnings: u32 = input
        .lines()
        .map(|line| parse_line(line))
        .sorted()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            rank as u32 * hand.bid
        })
        .sum();
    /*
       let hands_by_rank = input
           .lines()
           .map(|line| parse_line(line))
           .sorted()
           .enumerate()
           .map(|(i, hand)| {
               let rank = i + 1;
               (rank, hand)
           })
           .collect_vec();
       dbg!(hands_by_rank);
    */
    Ok(winnings.to_string())
}

fn parse_line(line: &str) -> Hand {
    let (_, hand) = hand(line).unwrap();
    hand
}

fn hand(i: &str) -> IResult<&str, Hand> {
    let (i, (l1, l2, l3, l4, l5, _, bid)) = tuple((
        anychar,
        anychar,
        anychar,
        anychar,
        anychar,
        space1,
        complete::u32,
    ))(i)?;
    Ok((
        i,
        Hand::new(
            &[
                Card::new(l1).unwrap(),
                Card::new(l2).unwrap(),
                Card::new(l3).unwrap(),
                Card::new(l4).unwrap(),
                Card::new(l5).unwrap(),
            ],
            bid,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
