
#[derive(Debug)]
pub struct RTRLog {
    pub kind: RTRLogKind,
    pub text: String,
}

impl RTRLog {
    pub fn format(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug)]
pub enum RTRLogKind {
    Info,
    Log,
    Warn,
    Error
}
