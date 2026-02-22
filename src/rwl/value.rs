use crate::shared::color::Color;
use crate::shared::theme::Theme;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f32),
    Percentage(f32),
    Color(Color),
    Property(PropertyPath)
}

#[derive(Debug, Clone)]
pub enum PropertyPath {
    Theme(ThemeProperty)
}

#[derive(Debug, Clone)]
pub enum ThemeProperty {
    Back,
    Prim,
    Seco,
    Tert,
    Text,
    Accent
}

impl Value {
    pub fn get_type(&self) -> &str {
        match self {
            Value::Str(..) => "str",
            Value::Num(..) => "num",
            Value::Percentage(..) => "percentage",
            Value::Color(..) => "color",
            
            #[allow(clippy::match_same_arms)]
            Value::Property(PropertyPath::Theme(..)) =>
                "color"
        }
    }
    
    pub fn get_str(&self) -> &String {
        match self {
            Value::Str(str) => Some(str),
            _ => None
        }.unwrap()
    }
    pub fn get_num(&self) -> &f32 {
        match self {
            Value::Num(num) => Some(num),
            _ => None
        }.unwrap()
    }
    pub fn get_percentage(&self) -> &f32 {
        match self {
            Value::Percentage(percentage) => Some(percentage),
            _ => None
        }.unwrap()
    }
    pub fn get_color(&self, theme: &Theme) -> Color {
        match self {
            Value::Color(color) => *color,
            
            Value::Property(PropertyPath::Theme(ThemeProperty::Back)) =>
                theme.background,
            Value::Property(PropertyPath::Theme(ThemeProperty::Prim)) =>
                theme.primary,
            Value::Property(PropertyPath::Theme(ThemeProperty::Seco)) =>
                theme.secondary,
            Value::Property(PropertyPath::Theme(ThemeProperty::Tert)) =>
                theme.tertiary,
            Value::Property(PropertyPath::Theme(ThemeProperty::Text)) =>
                theme.text,
            Value::Property(PropertyPath::Theme(ThemeProperty::Accent)) =>
                theme.accent,
            
            _ => panic!("attempt to get color out of non color")
        }
    }
}