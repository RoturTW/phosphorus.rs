use std::fmt::{Display, Formatter};
use colored;
use colored::Colorize;

pub enum LogKind {
    Info,
    Log,
    Warn,
    Error
}

impl Display for LogKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogKind::Info =>
                write!(f, "i"),
            LogKind::Log =>
                write!(f, "l"),
            LogKind::Warn =>
                write!(f, "!"),
            LogKind::Error =>
                write!(f, "e"),
        }
    }
}

pub enum LogSource {
    None,
    Other,
    Rtr,
    Rwl
}

impl Display for LogSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogSource::None =>
                write!(f, "."),
            LogSource::Other =>
                write!(f, "?"),
            LogSource::Rtr =>
                write!(f, "RTR"),
            LogSource::Rwl =>
                write!(f, "RWL")
        }
    }
}

pub struct Log {
    pub kind: LogKind,
    pub source: LogSource,
    pub text: String
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !matches!(self.source, LogSource::None) {
            // [source]
            write!(f, "{} ", format!("[{}]", self.source).bright_cyan().bold())?;
        }
        // little log type char
        //write!(f, "{} ", self.log_kind.to_string().bold());
        // text
        write!(f, "{}", self.text)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! print_raw {
    ($kind: expr, $source: expr, $text: expr) => {
        print_log(&Log {
            kind: $kind,
            source: $source,
            text: $text
        })
    };
}

#[macro_export]
macro_rules! print_info {
    ($source: expr, $($arg:tt)*) => {
        print_raw!(LogKind::Info, $source, format!($($arg)*))
    };
}
#[macro_export]
macro_rules! print_log {
    ($source: expr, $($arg:tt)*) => {
        print_raw!(LogKind::Log, $source, format!($($arg)*))
    };
}
#[macro_export]
macro_rules! print_warn {
    ($source: expr, $($arg:tt)*) => {
        print_raw!(LogKind::Warn, $source, format!($($arg)*))
    };
}
#[macro_export]
macro_rules! print_error {
    ($source: expr, $($arg:tt)*) => {
        print_raw!(LogKind::Error, $source, format!($($arg)*))
    };
}

pub fn print_log(log: &Log) {
    let txt = log.to_string();
    match log.kind {
        LogKind::Info =>
            println!("{}", txt.bright_cyan()),
        LogKind::Log =>
            println!("{}", txt.bright_white()),
        LogKind::Warn =>
            eprintln!("{}", txt.bright_yellow()),
        LogKind::Error =>
            eprintln!("{}", txt.bright_red()),
    }
}
