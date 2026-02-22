use crate::shared::color::Color;

#[derive(Debug)]
pub struct Theme {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub text: Color,
    pub accent: Color,
    
    // TODO: devtools stuff?
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::hex("#090a0b").unwrap(),
            primary: Color::hex("#1a1d28").unwrap(),
            secondary: Color::hex("#313f4e").unwrap(),
            tertiary: Color::hex("#4f617d").unwrap(),
            text: Color::hex("#b3cbf9").unwrap(),
            accent: Color::hex("#a600ff").unwrap()
        }
    }
}
