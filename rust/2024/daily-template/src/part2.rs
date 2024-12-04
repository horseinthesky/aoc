#[tracing::instrument(skip(input), ret)]
pub fn process(
    input: &str,
) -> miette::Result<String> {
    Ok(0.to_string())
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
