use crate::custom_error::AocError;

#[tracing::instrument(skip(input), ret)]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
