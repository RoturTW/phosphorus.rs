pub fn is_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(char::is_numeric)
}
