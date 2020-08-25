use colored::*;

pub fn info<S: std::fmt::Display>(msg: S) {
  println!("{} {}", "INFO:".green(), msg);
}

pub fn error<S: std::fmt::Display>(msg: S) {
  println!("{} {}", "ERROR:".red(), msg);
}
