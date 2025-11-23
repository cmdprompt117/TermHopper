use std::io::{stdout, Write};

// General helpers

/// Clear the terminal
pub fn cls() { print!("{}[2J", 27 as char); }
pub fn print_flush<T: std::fmt::Display>(con: T) { print!("{}", con); stdout().flush().ok(); }