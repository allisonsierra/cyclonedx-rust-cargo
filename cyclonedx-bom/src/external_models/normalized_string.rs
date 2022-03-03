/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use validator::{Validate, ValidationError};

/// A string that does not contain carriage return, line feed, or tab characters.
/// Defined via the [XML schema](https://www.w3.org/TR/xmlschema-2/#normalizedString)
#[derive(Debug, Default, PartialEq, Validate)]
pub struct NormalizedString {
    #[validate(custom = "validate_normalized")]
    value: String,
}

impl NormalizedString {
    pub fn new(value: &str) -> Self {
        let value = value
            .replace("\r\n", " ")
            .replace("\r", " ")
            .replace("\n", " ")
            .replace("\t", " ");
        NormalizedString { value }
    }

    /// Allow for the existence of invalid inputs from other data sources
    pub(crate) fn new_unchecked(value: String) -> Self {
        NormalizedString { value }
    }
}

impl ToString for NormalizedString {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

fn validate_normalized(value: &str) -> Result<(), ValidationError> {
    if value.contains("\r\n")
        || value.contains("\r")
        || value.contains("\n")
        || value.contains("\t")
    {
        return Err(ValidationError::new("contains invalid whitespace"));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_normalize_strings() {
        assert_eq!(
            NormalizedString {
                value: "no_whitespace".to_string()
            },
            NormalizedString::new("no_whitespace")
        );
        assert_eq!(
            NormalizedString {
                value: "spaces and tabs".to_string()
            },
            NormalizedString::new("spaces and\ttabs")
        );
        assert_eq!(
            NormalizedString {
                value: "carriage returns and linefeeds".to_string()
            },
            NormalizedString::new("carriage\r\nreturns\rand\nlinefeeds")
        );
    }

    #[test]
    fn it_should_validate_a_normalized_string() {
        let normalized_string = NormalizedString::new_unchecked("test with spaces".to_string());

        assert_eq!(normalized_string.validate(), Ok(()));
    }

    #[test]
    fn it_should_reject_invalid_strings() {
        let test_with_tabs = NormalizedString::new_unchecked("test\twith\ttabs".to_string())
            .validate()
            .expect_err("Should have failed validation due to tabs");

        assert_eq!(format!("{}", test_with_tabs), "value: Validation error: contains invalid whitespace [{\"value\": String(\"test\\twith\\ttabs\")}]");

        let test_with_newlines = NormalizedString::new_unchecked(
            "test\ncarriage\r\nreturns\rand\nlinefeeds".to_string(),
        )
        .validate()
        .expect_err("Should have failed validation due to newlines");

        assert_eq!(
            format!("{}", test_with_newlines),
            "value: Validation error: contains invalid whitespace [{\"value\": String(\"test\\ncarriage\\r\\nreturns\\rand\\nlinefeeds\")}]"
        );
    }
}
