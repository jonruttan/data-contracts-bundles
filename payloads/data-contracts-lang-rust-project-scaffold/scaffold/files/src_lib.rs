pub fn parse_expression(input: &str) -> Result<&str, String> {
    if input.trim().is_empty() {
        return Err("expression must be non-empty".to_string());
    }
    Ok(input)
}
