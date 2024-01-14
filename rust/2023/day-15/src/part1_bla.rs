use crate::custom_error::AocError;

fn hash(input: &str) -> u8 {
    let mut result = 0;
    input.chars().for_each(|c| {
        result += c as u16;
        result *= 17;
        result %= 256;
    });
    result as u8
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let r1 = input
        .split(',')
        .map(|step| hash(step) as u32)
        .collect::<Vec<_>>()
        .iter()
        .sum::<u32>();

    Ok(r1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
