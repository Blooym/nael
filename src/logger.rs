// macro_rules! debug {
//     ($($arg:tt)*) => {
//         use colored::Colorize;
//         println!("{}", format!($($arg)*).dimmed());
//     };
// }
// pub(crate) use debug;

macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*));
    };
}
pub(crate) use info;

// macro_rules! warning {
//     ($($arg:tt)*) => {
//         use colored::Colorize;
//         eprintln!("{}", format!($($arg)*).yellow());
//     };
// }
// pub(crate) use warning;

macro_rules! error {
    ($($arg:tt)*) => {
        use colored::Colorize;
        eprintln!("{}", format!($($arg)*).red());
    };
}
pub(crate) use error;
