use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::many0,
    sequence::{delimited, preceded, tuple},
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
    fn new(nred: u32, ngreen: u32, nblue: u32) -> Self {
        Self {
            red: nred,
            green: ngreen,
            blue: nblue,
        }
    }

    /// Is this greater or equal than `other`
    fn ge(&self, other: Draw) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

/// `draw = draw + number_of_cubes_red;` (etc.) is neat feature for our API here.
impl Add<Ncubes> for Draw {
    type Output = Self;

    fn add(self, rhs: Ncubes) -> Self {
        let (r, g, b) = match rhs {
            Ncubes::Red(r) => (r, 0, 0),
            Ncubes::Green(g) => (0, g, 0),
            Ncubes::Blue(b) => (0, 0, b),
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
    let bag = Draw::new(12, 13, 14);

    let sum_ids: u32 = input
        .lines()
        .filter_map(|line| {
            let (draws, id) = parse_game(line).expect("Line should start with Game n: ...");
            if all_draws_possible(draws, &bag) {
                Some(id)
            } else {
                None
            }
        })
        .sum();

    Ok(sum_ids.to_string())
}

/// Parse start of line, then return draws and game ID
fn parse_game(i: &str) -> IResult<&str, u32> {
    let (i, (_, id, _)) = tuple((tag("Game "), digit1, tag(":")))(i)?;

    let id = u32::from_str_radix(id, 10).unwrap(); // Result guaranteed by nom

    Ok((i, id))
}

/// Are all draws possible when checked against a draw emptying the bag?
fn all_draws_possible(i: &str, bag: &Draw) -> bool {
    let mut possible = true;

    let (_, (draw, many0_draws)) = tuple((draw, many0(preceded(tag(";"), draw))))(i).unwrap();

    possible = possible && bag.ge(draw);
    for draw in many0_draws {
        possible = possible && bag.ge(draw);
    }
    possible
}

/// Parse draws
fn _parse_draws(i: &str) -> IResult<&str, Vec<Draw>> {
    let mut draws: Vec<Draw> = vec![];

    let (_, (draw, many0_draws)) = tuple((draw, many0(preceded(tag(";"), draw))))(i)?;

    draws.push(draw);
    for draw in many0_draws {
        draws.push(draw);
    }
    Ok(("", draws))
}

/// Parse into a single `Draw` from raw data
fn draw(i: &str) -> IResult<&str, Draw> {
    let mut draw = Draw::default();

    let (i, (cubes, many0_cubes)) = tuple((cubes, many0(preceded(tag(","), cubes))))(i)?;

    draw = draw + cubes;
    for cubes in many0_cubes {
        draw = draw + cubes;
    }
    Ok((i, draw))
}

/// Parser's representation of cubes; number of... with a color variant
enum Ncubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn cubes(i: &str) -> IResult<&str, Ncubes> {
    let (i, (n, label)) = tuple((
        delimited(tag(" "), digit1, tag(" ")),
        alt((tag("red"), tag("green"), tag("blue"))),
    ))(i)?;

    let n = u32::from_str_radix(n, 10).unwrap(); // Result guaranteed by nom

    let cube = match label {
        "red" => Ncubes::Red(n),
        "green" => Ncubes::Green(n),
        _ => Ncubes::Blue(n), // Parser guarantees again
    };
    Ok((i, cube))
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
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
