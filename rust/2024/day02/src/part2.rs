use crate::custom_error::AocError;

fn is_safe_orig(report: &Vec<i32>) -> bool {
    let mut direction = 0; // 0 - undefined, 1 - increasing, -1 - decreasing

    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];

        // Check diff condition
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        let current_direction = if diff > 0 { 1 } else { -1 };

        if direction == 0 {
            direction = current_direction;
        }

        // Check direction condition
        if direction != current_direction {
            return false;
        }
    }

    true
}

fn is_safe(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|i| {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        is_safe_orig(&modified_report)
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let safe = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(is_safe)
        .count();

    Ok(safe.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
