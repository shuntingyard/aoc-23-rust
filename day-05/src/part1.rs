use std::ops::Range;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{self, multispace1, space1},
    multi::{fold_many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Router<'a> {
    _label: &'a str,
    // src_range, dst_start
    routes: Vec<(Range<u64>, u64)>,
}

impl Router<'_> {
    fn route(&self, src: u64) -> u64 {
        if let Some(route) = self.routes.iter().find(|(range, _)| range.contains(&src)) {
            let (src_range, dst_start) = route;
            let dst_offset = src - src_range.start;

            dst_start + dst_offset
        } else {
            src
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Don't know yet... let's start.
    let (i, seeds) = dbg!(seeds(input).unwrap());

    let (i, seed_router) = dbg!(router(i, "seed-to-soil map:").unwrap());
    let (i, soil_router) = dbg!(router(i, "soil-to-fertilizer map:").unwrap());
    let (i, water_router) = dbg!(router(i, "fertilizer-to-water map:").unwrap());
    let (i, light_router) = dbg!(router(i, "water-to-light map:").unwrap());
    let (i, temperature_router) = dbg!(router(i, "light-to-temperature map:").unwrap());
    let (i, humidity_router) = dbg!(router(i, "temperature-to-humidity map:").unwrap());
    let (_, location_router) = dbg!(router(i, "humidity-to-location map:").unwrap());

    let soils = seeds
        .iter()
        .map(|seed| seed_router.route(*seed))
        .collect::<Vec<u64>>();

    let fertilizers = soils
        .iter()
        .map(|soil| soil_router.route(*soil))
        .collect::<Vec<u64>>();

    let water = fertilizers
        .iter()
        .map(|fertilizer| water_router.route(*fertilizer))
        .collect::<Vec<u64>>();

    let lights = water
        .iter()
        .map(|water| light_router.route(*water))
        .collect::<Vec<u64>>();

    let temperatures = lights
        .iter()
        .map(|light| temperature_router.route(*light))
        .collect::<Vec<u64>>();

    let humidity = temperatures
        .iter()
        .map(|temperature| humidity_router.route(*temperature))
        .collect::<Vec<u64>>();

    let locations = dbg!(humidity
        .iter()
        .map(|humidity| location_router.route(*humidity))
        .collect::<Vec<u64>>());

    Ok(locations.iter().min().unwrap().to_string())
}

fn seeds(i: &str) -> IResult<&str, Vec<u64>> {
    Ok(preceded(
        tag("seeds: "),
        separated_list1(space1, complete::u64),
    )(i)?)
}

fn router<'a>(i: &'a str, label: &'a str) -> IResult<&'a str, Router<'a>> {
    let (i, (_, routes)) = tuple((
        preceded(multispace1, tag(label)),
        fold_many1(
            preceded(
                multispace1,
                tuple((
                    complete::u64,
                    preceded(tag(" "), complete::u64),
                    preceded(tag(" "), complete::u64),
                )),
            ),
            Vec::new,
            |mut routes: Vec<_>, (dst, src_start, range)| {
                routes.push((src_start..src_start + range, dst));
                routes
            },
        ),
    ))(i)?;

    Ok((
        i,
        Router {
            routes,
            _label: label,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
