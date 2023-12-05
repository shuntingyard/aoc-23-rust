use std::collections::BTreeMap;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use tracing::debug;

use crate::custom_error::AocError;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // It's BTreeMap time as well: trying to keep track of card_ids and their copies
    let mut scratchcards: BTreeMap<u32, u32> = BTreeMap::new();

    let points = input
        .lines()
        .map(move |i| {
            let (_, (card_id, winning_numbers, numbers_you_have)) = parse_card(i).unwrap();
            if let Some(copies) = scratchcards.insert(card_id.clone(), 1) {
                scratchcards.insert(card_id.clone(), copies + 1);
            }

            // Count numbers winning something on card's rhs
            let count = numbers_you_have
                .iter()
                .filter(|number| winning_numbers.contains(number))
                .map(|_| 1)
                .sum::<u32>();

            // Retrieve number of copies for the current card
            let repetitions = scratchcards.get(&card_id);
            if repetitions.is_none() {
                panic!("Card {card_id:3} should have been registered by now...");
            }
            let repetitions = repetitions.unwrap().clone();

            if count < 1 {
                debug!("Card {card_id:3} occurs {repetitions:7} times");
            } else {
                for id in card_id + 1..card_id + 1 + count {
                    if let Some(copies) = scratchcards.get(&id) {
                        scratchcards.insert(id, repetitions + copies);
                    } else {
                        scratchcards.insert(id, repetitions);
                    }
                }
                debug!("Card {card_id:3} occurs {repetitions:7} times");
            }
            repetitions // <- This is what counts in part2
        })
        .sum::<u32>();

    Ok(points.to_string())
}

/// Result rhs now holds: (card_id, winning_numbers, numbers_you_have)
fn parse_card(i: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    // Now we need the header's card number
    let (i, (_, _, card_id, _)) = tuple((tag("Card"), multispace1, complete::u32, tag(":")))(i)?;

    // Then get what's required to eval cards.
    let (i, (_, winning_numbers, _, _, numbers_you_have)) = tuple((
        multispace1, // *Must* eat space before e.g. '30  1 29'...
        separated_list1(multispace1, complete::u32),
        tag(" |"),
        multispace1,
        separated_list1(multispace1, complete::u32),
    ))(i)?;

    if i != "" {
        panic!("Parsing problem: Card had unrecognized tail '{i}'");
    }
    Ok((i, (card_id, winning_numbers, numbers_you_have)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
