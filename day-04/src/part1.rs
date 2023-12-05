use nom::{
    self,
    bytes::complete::tag,
    character::complete::{self, digit1, multispace1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // It's nom day again :D
    let points = input.lines().map(|line| eval_card(line)).sum::<u32>();

    Ok(points.to_string())
}

/// Parse a card and return its points
fn eval_card(i: &str) -> u32 {
    let (_, (winning_numbers, numbers_you_have)) = parse_card(i).expect("Card had bad format!");
    // Count numbers winning something on card's rhs
    let count = numbers_you_have
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .map(|_| 1)
        .sum::<u32>();

    if count < 1 {
        0 // no winning numbers, no points
    } else {
        2u32.pow(count - 1)
        // 1 -> 2^0 or 1 point, 2 -> 2^1 or 2 points, ... 4 -> 2^3 or 8 points, etc.
    }
}

/// Result rhs holds: (winning_numbers, numbers_you_have)
fn parse_card(i: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    // Do away with header stuff...
    let (i, _) = tuple((tag("Card"), multispace1, digit1, tag(":")))(i)?;

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
    Ok((i, (winning_numbers, numbers_you_have)))
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
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
