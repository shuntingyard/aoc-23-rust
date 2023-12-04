use nom::{branch::alt, bytes::complete::tag, error::ErrorKind, error_position, IResult};

use crate::custom_error::AocError;

/// Parse ASCII digit from input `nom` style
fn parse_digit(i: &str) -> IResult<&str, u8> {
    let b = i.as_bytes();
    if b.len() > 0 {
        // See if is ascii digit (1 to 9)
        if 48 < b[0] && b[0] < 58 {
            Ok((&i[1..], b[0] - 48))
        } else {
            // Not digit
            let e = error_position!(i, ErrorKind::IsNot);
            Err(nom::Err::Error(e))
        }
    } else {
        // Zero length
        let e = error_position!(i, ErrorKind::Eof);
        Err(nom::Err::Error(e))
    }
}

/// Parse numeral (one to nine) from input `nom` style
fn parse_numeral(i: &str) -> IResult<&str, u8> {
    let (i, numeral) = alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ))(i)?;

    match numeral {
        "one" => Ok((i, 1)),
        "two" => Ok((i, 2)),
        "three" => Ok((i, 3)),
        "four" => Ok((i, 4)),
        "five" => Ok((i, 5)),
        "six" => Ok((i, 6)),
        "seven" => Ok((i, 7)),
        "eight" => Ok((i, 8)),
        "nine" => Ok((i, 9)),
        _ => Ok((i, 0)),
    }
}

/// Concatenate leftmost and rightmost symbol in line where a symbol is:
///     1) ASCII digit (probably 1 to 9 instead of 0 to 9)
///     2) one of the numerals: one, two, three, four, five, six, seven, eight, nine
fn cat_lms_rms(line: &str) -> u128 {
    let mut lms: Option<u8> = None;
    let mut rms: Option<u8> = None;

    for l in 0..line.len() {
        let r = line.len() - l - 1;

        let lwin = &line[l..]; // A slice starting at the leftmost position and shrinking
        let rwin = &line[r..]; // A slice starting at the rightmost position and growing

        if lms.is_none() {
            if let Some((_, n)) = alt((parse_numeral, parse_digit))(lwin).ok() {
                lms = Some(n);
            }
        }
        if rms.is_none() {
            if let Some((_, n)) = alt((parse_numeral, parse_digit))(rwin).ok() {
                rms = Some(n);
            }
        }
        // Break early if you can.
        if lms.is_some() && rms.is_some() {
            break;
        }
    }

    // Return value
    10 * lms.unwrap() as u128 + 1 * rms.unwrap() as u128
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Just use an iterator to sum calibration values.
    let cal_sum: u128 = input.lines().map(|line| cat_lms_rms(line)).sum();

    Ok(format!("{cal_sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
