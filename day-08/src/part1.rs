use std::collections::BTreeMap;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Automaton<'a> {
    transitions: &'a BTreeMap<&'a str, (&'a str, &'a str)>,
    state: String,     // This requires mutability.
    accepting: String, // We just need one.
    i: usize,          // Index for reading
    read_result: Vec<u32>,
}

impl<'a> Automaton<'a> {
    /// Accept a reference to a (ordered) map of transitions. Panics if map is empty!
    fn new(transitions: &'a mut BTreeMap<&'a str, (&'a str, &'a str)>) -> Self {
        let state = transitions.first_entry().unwrap().key().to_string();
        let accepting = transitions.last_entry().unwrap().key().to_string();

        Self {
            transitions,
            state,
            accepting,
            i: 0,
            read_result: vec![],
        }
    }

    /// Reads a string slice containing combinations of L and R. Repeats reading while not in a
    /// accepting state. Panics on any symbol not L or R. Returns Iterator.
    fn read(&mut self, path: &str) -> std::slice::Iter<'_, u32> {
        assert!(path.len() > 0);

        while self.state != self.accepting {
            let next_state = if path[self.i..=self.i] == *"L" {
                let (transit_l, _) = self.transitions.get(self.state.as_str()).unwrap();
                transit_l.to_string()
            } else if path[self.i..=self.i] == *"R" {
                let (_, transit_r) = self.transitions.get(self.state.as_str()).unwrap();
                transit_r.to_string()
            } else {
                panic!("Illegal symbol in path");
            };
            /*
            println!(
                "{} -> {next_state} {}",
                self.state,
                path[..=self.i].to_string()
            );
             */
            self.state = next_state.to_string();
            // Update index
            self.i = if self.i == path.len() - 1 {
                0
            } else {
                self.i + 1
            };

            // Allow caller to sum transitions.
            self.read_result.push(1);
        }
        self.read_result.iter()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Haunted Wasteland, repeat instructions!
    let (input, path) = get_path(input).unwrap();
    let mut transitions: BTreeMap<&str, (&str, &str)> = input
        .lines()
        .map(|line| {
            let (_, st) = get_transition(line).unwrap();
            st
        })
        .collect();

    let mut walker = Automaton::new(&mut transitions);
    Ok(walker.read(path).sum::<u32>().to_string())
}

fn get_path(input: &str) -> IResult<&str, &str> {
    let (i, (path, _)) = tuple((alpha1, multispace1))(input)?;

    Ok((i, path))
}

fn get_transition(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (_, (state, lr)) = tuple((
        alpha1,
        preceded(
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alpha1, tag(", "), alpha1),
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
