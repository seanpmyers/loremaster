use anyhow::Result;

pub fn character_is_invisible_unicode(input: &char) -> bool {
    *input as u32 > 0x7F
}

pub fn remove_invisible_characters(input: String) -> Result<String> {
    Ok(input
        .chars()
        .filter(|character| !character_is_invisible_unicode(character))
        .collect())
}

pub fn sanitize_user_input_string(mut input: String) -> Result<String> {
    input = remove_invisible_characters(input)?;
    let mut result: String = input.trim().to_string().chars().collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::utility::constants::unicode;

    use super::sanitize_user_input_string;

    #[test]
    fn verify_whitespace() -> Result<()> {
        assert!(
            sanitize_user_input_string(String::from(unicode::invisible::LEFT_TO_RIGHT_MARK))?
                .is_empty()
        );
        assert!(sanitize_user_input_string(String::from(
            unicode::invisible::CHARACTER_TABULATION
        ))?
        .is_empty());
        Ok(())
    }
}
