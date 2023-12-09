use std::collections::BTreeMap;

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, multispace1},
    multi::many1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Automaton<'a> {
    state_count: usize,   // Know with how many states in parallel we're dealing.
    states: Vec<&'a str>, // This requires mutability.
    i: usize,             // Index for reading
    read_result: Vec<u32>,
}

impl<'a> Automaton<'a> {
    /// Accept a reference to a (ordered) map of transitions.
    fn new(transitions: &'a BTreeMap<&'a str, (&'a str, &'a str)>) -> Self {
        // For initialization collect start states.
        let states: Vec<&'a str> = dbg!(transitions
            .keys()
            .filter(|s| s.ends_with("A"))
            .map(|s| *s)
            .collect());

        Self {
            state_count: states.len(),
            states,
            i: 0,
            read_result: vec![],
        }
    }

    /// Reads a vector of Left and Right and applies it to a collection of states/transitions.
    /// repeats reading until all transitions reach an accepting state in parallel. Returns
    /// Iterator.
    fn read<'b: 'a>(
        &mut self,
        path: Vec<Direction>,
        transitions: &'b BTreeMap<&'b str, (&'b str, &'b str)>,
    ) -> std::slice::Iter<'_, u32> {
        assert!(path.len() > 0);
        let mut iterations = 0u32;

        use Direction::*;
        while self.states.iter().filter(|s| s.ends_with("Z")).count() < self.state_count {
            // Accumulate transition count.
            iterations += 1;

            let mut working_states: Vec<&str> = vec![];

            for (no, state) in self.states.iter().enumerate() {
                let next = transitions.get(state).unwrap();

                let next_state = match path[self.i] {
                    Left => next.0,
                    Right => next.1,
                };

                // println!("{state} -> {next_state}",);
                if next_state.ends_with("Z") {
                    println!("{next_state} reached by {no} in iteration {iterations}")
                }

                working_states.push(&next_state);
            }
            // Transitions now become new states.
            self.states = working_states;

            // Update index
            self.i = if self.i == path.len() - 1 {
                0
            } else {
                self.i + 1
            };
        }
        // Return value
        self.read_result.push(iterations);
        self.read_result.iter()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, path) = get_path(input).unwrap();
    let transitions: BTreeMap<&str, (&str, &str)> = input
        .lines()
        .map(|line| {
            let (_, st) = get_transition(line).unwrap();
            st
        })
        .collect();

    let mut walker = Automaton::new(&transitions);
    Ok(walker.read(path, &transitions).sum::<u32>().to_string())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// A nicer parser. This one is inpired by Chris Biscardi.
fn get_path(input: &str) -> IResult<&str, Vec<Direction>> {
    let (i, (path, _)) = tuple((
        many1(alt((
            complete::char('L').map(|_| Direction::Left),
            complete::char('R').map(|_| Direction::Right),
        ))),
        multispace1,
    ))(input)?;

    Ok((i, path))
}

// The only change here is to alphanumeric1.
fn get_transition(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (_, (state, lr)) = tuple((
        alphanumeric1,
        preceded(
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
    ))(line)?;

    Ok(("", (state, lr)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("six", process(input)?);
        Ok(())
    }
}
