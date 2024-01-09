use std::collections::BTreeMap;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct Card {
    win: usize,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let mut win = 0;

            let blocks = line.split(":");

            let mut numbers =
                blocks.last().unwrap().split("|");

            let (winning, mine) = (
                numbers
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
                numbers
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
            );

            mine.iter().for_each(|n| {
                if winning.contains(n) {
                    win += 1
                }
            });

            Card { win }
        })
        .collect();

    let data = cards
        .iter()
        .map(|card| card.win)
        .collect::<Vec<_>>();

    let store = (0..cards.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();

    let result = data
        .iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)..(index + 1 + *card_score)
            {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
