use anyhow::{bail, Result};

use crate::models::{ParamType, QueryParam};

const MAX_PARAM_LENGTH: usize = 512;

pub fn validate_param(param: &QueryParam) -> Result<()> {
    check_empty(&param.value)?;
    check_length(&param.value)?;
    check_null_bytes(&param.value)?;
    check_encoding(&param.value)?;
    check_type_constraints(param)?;
    Ok(())
}

fn check_empty(value: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("parameter value must not be empty");
    }
    Ok(())
}

fn check_length(value: &str) -> Result<()> {
    if value.len() > MAX_PARAM_LENGTH {
        bail!(
            "parameter value exceeds maximum length of {} bytes",
            MAX_PARAM_LENGTH
        );
    }
    Ok(())
}

fn check_null_bytes(value: &str) -> Result<()> {
    if value.bytes().any(|b| b == 0) {
        bail!("parameter value contains null bytes");
    }
    Ok(())
}

fn check_encoding(value: &str) -> Result<()> {
    let mut prev_was_backslash = false;
    for ch in value.chars() {
        if prev_was_backslash && ch == '\0' {
            bail!("escaped null sequence detected");
        }
        prev_was_backslash = ch == '\\';

        if ch == '\u{FFFD}' {
            bail!("value contains unicode replacement character indicating encoding error");
        }

        if ('\u{FDD0}'..='\u{FDEF}').contains(&ch) {
            bail!("value contains unicode noncharacter");
        }
    }
    Ok(())
}

fn check_type_constraints(param: &QueryParam) -> Result<()> {
    match param.param_type {
        ParamType::Integer => {
            if param.value.parse::<i64>().is_err() {
                bail!("expected integer value, got: {}", param.value);
            }
        }
        ParamType::Boolean => {
            let lower = param.value.to_lowercase();
            if !matches!(lower.as_str(), "true" | "false" | "0" | "1") {
                bail!("expected boolean value, got: {}", param.value);
            }
        }
        ParamType::Identifier => {
            if !param
                .value
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_')
   {
                bail!("identifier contains invalid characters: {}", param.value);
            }
        }
        ParamType::Text => {}
    }
    Ok(())
}

pub fn sanitize_input(value: &str) -> String {
    value
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_text_param() {
        let p = QueryParam::text("name", "hello world");
        assert!(validate_param(&p).is_ok());
    }

    #[test]
    fn test_empty_rejected() {
        let p = QueryParam::text("name", "   ");
        assert!(validate_param(&p).is_err());
    }

    #[test]
    fn test_null_byte_rejected() {
        let p = QueryParam::text("name", "hello\0world");
        assert!(validate_param(&p).is_err());
    }

    #[test]
    fn test_encoding_check_replacement_char() {
        let p = QueryParam::text("name", "bad\u{FFFD}data");
        assert!(validate_param(&p).is_err());
    }

    #[test]
    fn test_sanitize_strips_control() {
        let result = sanitize_input("hello\x07world\nnext");
        assert_eq!(result, "helloworld\nnext");
    }
}
