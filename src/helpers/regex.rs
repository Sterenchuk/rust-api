use once_cell::sync::Lazy;
use regex::Regex;

/// Username must be 3–20 characters, alphanumeric or underscore
pub fn is_name_valid(name: &str) -> bool {
    static NAME_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,20}$").expect("Invalid NAME regex"));

    NAME_REGEX.is_match(name)
}

/// Simple email validation
pub fn is_email_valid(email: &str) -> bool {
    static EMAIL_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").expect("Invalid EMAIL regex"));

    EMAIL_REGEX.is_match(email)
}

/// Password must be at least 8 chars and include:
/// - lowercase
/// - uppercase
/// - digit
/// - special character
pub fn is_password_valid(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_lowercase())
        && password.chars().any(|c| c.is_uppercase())
        && password.chars().any(|c| c.is_ascii_digit())
        && password.chars().any(|c| !c.is_alphanumeric())
}
