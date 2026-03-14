pub fn is_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(char::is_numeric)
}

// cursed things to match js
pub fn chr(num: f32) -> String {
    let n = num.trunc() as i64;
    let code_unit = i128::from(n).rem_euclid(65536) as u16;
    String::from_utf16(&[code_unit]).unwrap()
}

pub fn ord(str: &str) -> f32 {
    if str.is_empty() {
        f32::NAN
    } else {
        str.chars().next().unwrap() as u32 as f32
    }
}
