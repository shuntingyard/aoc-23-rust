use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

/// A draw (aka subset) in a game
#[derive(Default, Debug)]
struct Draw {
    red: u32,   // Number of red cubes in draw
    green: u32, // Number of green cubes in draw
    blue: u32,  // Number of blue cubes in draw
}

impl Draw {
    /// Max every internal value by comparing self and other and return result as `Draw`
    fn max(&self, other: Draw) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    /// Return all internal values multiplied
    fn pow(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

/// `draw = draw + number_of_cubes_red;` (etc.) is neat feature for our API here.
impl Add<ColorCount> for Draw {
    type Output = Self;

    fn add(self, rhs: ColorCount) -> Self {
        let (r, g, b) = match rhs {
            ColorCount::Red(r) => (r, 0, 0),
            ColorCount::Green(g) => (0, g, 0),
            ColorCount::Blue(b) => (0, 0, b),
        };

        Self {
            red: self.red + r,
            green: self.green + g,
            blue: self.blue + b,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sum_ids: u32 = input
        .lines()
        .map(|line| {
            let (draws, _) = parse_game(line).expect("Line should start with Game n: ...");
            let (_, pow) = get_min_set_power(draws).expect("Error parsing draws");
            pow
        })
        .sum();

    Ok(sum_ids.to_string())
}

/// Parse start of line, then return draws and game ID
fn parse_game(i: &str) -> IResult<&str, u32> {
    Ok(delimited(tag("Game "), complete::u32, tag(": "))(i)?)
}

/// For this game, find the minimum set of cubes that must have been present
/// and return its power (as nred * ngreen * nblue)
fn get_min_set_power(i: &str) -> IResult<&str, u32> {
    let mut set = Draw::default();

    let (_, sets) = separated_list1(tag("; "), draw)(i)?;

    for other in sets {
        set = set.max(other)
    }
    Ok(("", set.pow()))
}

/// Parse into a single `Draw` from raw data
fn draw(i: &str) -> IResult<&str, Draw> {
    let mut draw = Draw::default();

    let (i, color_counts) = separated_list1(tag(", "), color_item)(i)?;

    for count in color_counts {
        draw = draw + count;
    }
    Ok((i, draw))
}

/// Parser's representation of cubes; number of... with a color variant
enum ColorCount {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn color_item(i: &str) -> IResult<&str, ColorCount> {
    let (i, (n, label)) = separated_pair(
        complete::u32,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(i)?;

    let color_count = match label {
        "red" => ColorCount::Red(n),
        "green" => ColorCount::Green(n),
        _ => ColorCount::Blue(n), // Parser guarantees again
    };
    Ok((i, color_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
