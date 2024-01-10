use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

use crate::custom_error::AocError;

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_times(
    input: &str,
) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, (times, distances)) =
        parse_times(input).expect("a valid parse");

    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, record_distance)| {
            (0..time)
                .filter_map(|attempt_time| {
                    let speed = attempt_time; // Each millisecond of holding a button gives us a
                                              // speed of 1 millimeter per millisend
                    let attempt_distance = (time
                        - attempt_time)
                        * speed;

                    (attempt_distance > record_distance)
                        .then_some(attempt_distance)
                })
                .count()
        })
        .product::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
