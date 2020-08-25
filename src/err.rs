pub fn err<S: Into<String>>(msg: S) -> std::io::Result<()> {
  Err(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
}
