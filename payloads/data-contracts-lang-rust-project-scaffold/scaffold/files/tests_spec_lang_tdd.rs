use std::result::Result;

#[test]
fn parse_expression_rejects_empty() {
    let result: Result<&str, String> = data_contracts_lang::parse_expression(" ");
    assert!(result.is_err());
}
