use crate::app::{RequestError, RequestResult};

pub fn trimmed_required(value: &str, field: &str, min: usize, max: usize) -> RequestResult<String> {
    let trimmed = value.trim();

    if trimmed.len() < min {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must contain at least {min} characters"
        )));
    }

    if trimmed.len() > max {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must contain at most {max} characters"
        )));
    }

    Ok(trimmed.to_owned())
}

pub fn trimmed_optional(
    value: Option<String>,
    field: &str,
    max: usize,
) -> RequestResult<Option<String>> {
    match value {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }

            if trimmed.len() > max {
                return Err(RequestError::unprocessable_entity(format!(
                    "{field} must contain at most {max} characters"
                )));
            }

            Ok(Some(trimmed.to_owned()))
        }
        None => Ok(None),
    }
}

pub fn normalized_email(value: &str, field: &str) -> RequestResult<String> {
    let normalized = value.trim().to_lowercase();

    let has_valid_shape = normalized
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));

    if !has_valid_shape {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must be a valid email"
        )));
    }

    if normalized.len() > 320 {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must contain at most 320 characters"
        )));
    }

    Ok(normalized)
}

pub fn phone_number(value: &str, field: &str) -> RequestResult<String> {
    let trimmed = value.trim();
    let digits_count = trimmed.chars().filter(|char| char.is_ascii_digit()).count();

    if !(10..=15).contains(&digits_count) {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must contain from 10 to 15 digits"
        )));
    }

    if trimmed.len() > 32 {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must contain at most 32 characters"
        )));
    }

    Ok(trimmed.to_owned())
}

pub fn positive_i32(value: i32, field: &str) -> RequestResult<i32> {
    if value <= 0 {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must be greater than zero"
        )));
    }

    Ok(value)
}

pub fn non_negative_f64(value: f64, field: &str) -> RequestResult<f64> {
    if value < 0.0 {
        return Err(RequestError::unprocessable_entity(format!(
            "{field} must be greater than or equal to zero"
        )));
    }

    Ok(value)
}
