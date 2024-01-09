use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let scores = input.lines().map(|line| {
        let mut card_score: i32 = 0;

        let mut blocks = line.split(":");
        let _card = blocks
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();

        let mut numbers = blocks.last().unwrap().split("|");
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
                if card_score == 0 {
                    card_score = 1;
                    return;
                }

                card_score *= 2
            }
        });

        card_score
    })
    .sum::<i32>();

    Ok(scores.to_string())
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
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
