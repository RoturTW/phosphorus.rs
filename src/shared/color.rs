use raylib::color::Color as RLColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub fn hex(input: &str) -> Result<Color, &'static str> {
        parse_hex_color(input)
    }
}

pub fn parse_hex_color(input: &str) -> Result<Color, &'static str> {
    let hex = input.strip_prefix('#').unwrap_or(input);
    
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| "invalid hex")?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| "invalid hex")?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| "invalid hex")?;
            Ok(Color::rgb(r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex")?;
            Ok(Color::rgb(r, g, b))
        }
        _ => Err("expected #rgb or #rrggbb"),
    }
}

// raylib utils
pub fn col_to_raylib(col: Color) -> RLColor {
    RLColor {
        r: col.r,
        g: col.g,
        b: col.b,
        a: col.a,
    }
}
pub fn raylib_to_col(col: RLColor) -> Color {
    Color {
        r: col.r,
        g: col.g,
        b: col.b,
        a: col.a,
    }
}