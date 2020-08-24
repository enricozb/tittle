use colored::*;

pub fn info<S: std::fmt::Display>(msg: S) {
  println!("{} {}", "INFO:".green(), msg);
}
