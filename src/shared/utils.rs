pub fn is_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_numeric())
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

pub fn remove_indent(input: &str) -> String {
    let lines: Vec<&str> = input.split('\n').collect();
    
    let non_empty: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    if non_empty.is_empty() {
        return input.to_string();
    }
    
    let min_indent = non_empty
        .iter()
        .map(|l| l.chars().take_while(|c| *c == ' ').count())
        .min()
        .unwrap();
    
    lines
        .iter()
        .map(|l| {
            if l.len() >= min_indent {
                &l[min_indent..]
            } else {
                ""
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}