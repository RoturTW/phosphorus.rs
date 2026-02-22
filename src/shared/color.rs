use macroquad::color::Color as MQColor;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn to_mq(self) -> MQColor {
        self.into()
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

impl From<Color> for MQColor {
    fn from(c: Color) -> Self {
        MQColor::from_rgba(c.r, c.g, c.b, c.a)
    }
}

impl From<MQColor> for Color {
    fn from(c: MQColor) -> Self {
        Color {
            r: (c.r * 255.0) as u8,
            g: (c.g * 255.0) as u8,
            b: (c.b * 255.0) as u8,
            a: (c.a * 255.0) as u8,
        }
    }
}