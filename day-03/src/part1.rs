use std::{collections::BTreeMap, fmt::Display};

use crate::custom_error::AocError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Point {
    y: i32, // As part of a key value, y is higher in ordering - think numerals: ..123..
    x: i32,
}

#[derive(Debug)]
enum Symbol {
    Digit(u32),
    Neighbor,
    NoSymbol, // for '.'s
}

/// Ordered, i.e. low must be closer to the plain's (0,0) coordinate
struct Rect {
    low: Point,
    high: Point,
}

impl Rect {
    /// Gets all * as points in e.g.
    /// 0....+....1
    /// .   *****
    /// .   *123*
    /// .   *****
    fn get_hull(&self) -> Vec<Point> {
        let mut hull: Vec<Point> = vec![];
        // Upper part
        for x in self.low.x - 1..=self.high.x + 1 {
            hull.push(Point {
                y: self.low.y - 1,
                x,
            });
        }
        // Nine-o-clock
        hull.push(Point {
            y: self.low.y,
            x: self.low.x - 1,
        });
        // Three-o-clock
        hull.push(Point {
            y: self.high.y,
            x: self.high.x + 1,
        });
        // Lower part
        for x in self.low.x - 1..=self.high.x + 1 {
            hull.push(Point {
                y: self.low.y + 1,
                x,
            });
        }

        hull
    }
}

/// Just reflect the length of a rectangle's hull
impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for _ in self.get_hull() {
            buf += "+";
        }
        write!(f, "{buf}")
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // The whole idea: treat the input as 2D object.
    //

    // Parse into some sort of table as
    //  key: 2D point
    //  value: digit or symbol
    let plain = input
        .lines()
        .enumerate()
        // Stole this, as I never thought about flat maps before!
        .flat_map(|(row, line)| {
            // The idea to populate a sparse plain fails. To save time we always return Some 😩
            line.chars().enumerate().filter_map(move |(col, symbol)| {
                let point = Point {
                    x: col as i32,
                    y: row as i32,
                };
                match symbol {
                    n if n.is_ascii_digit() => Some((
                        point,
                        Symbol::Digit(n.to_digit(10).unwrap()), // Always succeeds at this point
                    )),
                    '.' => Some((point, Symbol::NoSymbol)),
                    _ => Some((point, Symbol::Neighbor)),
                }
            })
        })
        .collect::<BTreeMap<Point, Symbol>>();

    // Assemble adjacent digits in the plain
    let mut inventory: Vec<u32> = vec![];
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let mut part_no = 0u32;

    for entry in &plain {
        // dbg!(entry);
        match entry {
            (point, Symbol::Digit(n)) if start.is_some() => {
                part_no *= 10; // shift
                part_no += n;
                end = Some(point.clone());
            }
            (point, Symbol::Digit(n)) => {
                part_no = *n; // Start a new part number
                start = Some(point.clone());
                end = Some(point.clone());
            }
            (_, _) => {
                if start.is_some() && end.is_some() {
                    let rect = Rect {
                        low: start.clone().unwrap(),
                        high: end.clone().unwrap(),
                    };
                    if register_part(part_no, &rect, &plain, &mut inventory) {
                        // println!("{rect} {part_no} has at least one friendly neighbor and was registered");
                    } else {
                        // println!("{part_no} has no neighbors and was not registered");
                    }
                }
                // Reset
                start = None;
                end = None;
                part_no = 0;
            }
        }
    }
    // Plain might hold a last part number bottom-right that had not been triggered.
    if start.is_some() && end.is_some() {
        let rect = Rect {
            low: start.clone().unwrap(),
            high: end.clone().unwrap(),
        };
        if register_part(part_no, &rect, &plain, &mut inventory) {
            println!("last {part_no} has at least one friendly neighbor and was registered");
        } else {
            println!("last {part_no} has no neighbors and was not registered");
        }
    }

    // plain.iter().for_each(|entry| println!("{entry:?}"));
    Ok(inventory.iter().sum::<u32>().to_string())
}

/// Scan a rectangle's environment like:
///
///         p   p   ...     p
///
///         p   rectangle   p
///
///         p   p   ...     p
///
/// with p denoting possible symbols in its environment. Return value indicates if numeral in the
/// rectangle has been registered.
fn register_part(
    part_no: u32,
    rectangle: &Rect,
    plain: &BTreeMap<Point, Symbol>,
    inventory: &mut Vec<u32>,
) -> bool {
    for point in rectangle.get_hull() {
        if let Some(symbol) = plain.get(&point) {
            match symbol {
                Symbol::Neighbor => {
                    // Has some friendly neighbor, so it counts.
                    inventory.push(part_no);
                    return true;
                }
                _ => {}
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
