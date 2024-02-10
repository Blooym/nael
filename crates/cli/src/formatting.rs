use colored::{ColoredString, Colorize};

pub fn emphasis_text(s: &str) -> ColoredString {
    s.bold()
}

pub fn selected_value(s: &str) -> ColoredString {
    s.green()
}

pub fn warning_text(s: &str) -> ColoredString {
    s.yellow()
}

pub fn error_text(s: &str) -> ColoredString {
    s.bright_red().bold()
}
