use itertools::Itertools;
use nom::{
    self,
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

fn extrapolate(v_tup_int: Vec<i32>) -> Vec<i32> {
    if v_tup_int.iter().all(|i| *i == 0i32) {
        // Termination
        vec![0]
    } else {
        // Build a vector of pairwise differences and pass it down in the recursion.
        let v_results = extrapolate(
            v_tup_int
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<i32>>(),
        );
        // Then subtract the value just extrapolated by recursion from the leftmostmost in the current row...
        let res = v_tup_int.first().unwrap() - v_results.last().unwrap();
        let mut v_results = v_results;
        // ... and append the difference to results returned.
        v_results.push(res);
        v_results
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let (_, v_tup_int) = parse(line).unwrap();
            dbg!(extrapolate(v_tup_int)).last().unwrap().clone()
        })
        .sum::<i32>()
        .to_string())
}

fn parse(i: &str) -> IResult<&str, Vec<i32>> {
    Ok(separated_list1(space1, complete::i32)(i)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
