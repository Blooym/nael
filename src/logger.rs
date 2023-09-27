#![allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)*) => {
        use colored::Colorize;
        println!("[DEBUG] {}", format!($($arg)*).dimmed());
    };
}
#[allow(unused_imports)]
pub(crate) use debug;

macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*));
    };
}
pub(crate) use info;

#[allow(unused_macros)]
macro_rules! warning {
    ($($arg:tt)*) => {
        use colored::Colorize;
        eprintln!("[WARN] {}", format!($($arg)*).yellow());
    };
}
#[allow(unused_imports)]
pub(crate) use warning;

macro_rules! error {
    ($($arg:tt)*) => {
        use colored::Colorize;
        eprintln!("[ERROR] {}", format!($($arg)*).red());
    };
}
pub(crate) use error;
