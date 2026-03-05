use std::fmt::Display;

const IS_DEBUG: bool = true;

pub struct Log {}

pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

impl Display for LogLevel {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           LogLevel::DEBUG => write!(f, "[ DEBUG ]"),
           LogLevel::INFO => write!(f, "[ INFO ]"),
           LogLevel::WARNING => write!(f, "[ WARNING ]"),
           LogLevel::ERROR => write!(f, "[ ERROR ]"),
        }
    }
}

impl Log {
   pub fn write(log_level: LogLevel, message: &str) {
        if IS_DEBUG {
            println!("{}: {}", log_level, message);
        }
   }
}
