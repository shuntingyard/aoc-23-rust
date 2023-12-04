use crate::custom_error::AocError;

/// Parse ASCII digit from byte
fn parse_digit(b: u8) -> Option<u8> {
    // Decide if is ascii digit
    if 47 < b && b < 58 {
        Some(b - 48)
    } else {
        None
    }
}

/// Concatenate leftmost and rightmost digit in line
fn cat_lmd_rmd(line: &str) -> u128 {
    // We can afford to inspect single bytes for this problem.
    let line = line.as_bytes();

    let mut lmd: Option<u8> = None;
    let mut rmd: Option<u8> = None;

    for l in 0..line.len() {
        let r = line.len() - l - 1;

        if lmd.is_none() {
            lmd = parse_digit(line[l]);
        }
        if rmd.is_none() {
            rmd = parse_digit(line[r]);
        }
        // Break early if you can.
        if lmd.is_some() && rmd.is_some() {
            break;
        }
    }

    // Return values
    if lmd.is_none() || rmd.is_none() {
        0
    } else {
        10 * lmd.unwrap() as u128 + 1 * rmd.unwrap() as u128
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Just use an iterator to sum calibration values.
    let cal_sum: u128 = input.lines().map(|line| cat_lmd_rmd(line)).sum();
    Ok(format!("{cal_sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
