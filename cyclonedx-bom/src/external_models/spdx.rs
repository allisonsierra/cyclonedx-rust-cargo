use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct SpdxIdentifier(String);

impl TryFrom<String> for SpdxIdentifier {
    type Error = SpdxIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match license_exprs::validate_license_expr(&value) {
            Ok(()) => Ok(Self(value)),
            Err(e) => Err(SpdxIdentifierError::InvalidSpdxExpression(format!("{}", e))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SpdxIdentifierError {
    InvalidSpdxExpression(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_succeed_in_converting_an_spdx_expression() {
        let actual = SpdxIdentifier::try_from("MIT OR Apache-2.0".to_string())
            .expect("Failed to parse as a license");
        assert_eq!(actual, SpdxIdentifier("MIT OR Apache-2.0".to_string()));
    }

    #[test]
    fn it_should_fail_to_convert_an_invalid_spdx_expression() {
        let actual = SpdxIdentifier::try_from("not a real license".to_string())
            .expect_err("Should have failed to parse as a license");
        assert_eq!(
            actual,
            SpdxIdentifierError::InvalidSpdxExpression(
                "unknown license or other term: not".to_string()
            )
        );
    }
}
