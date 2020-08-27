/// Convenience `Err` creation function.
pub fn err<S: Into<String>, O>(msg: S) -> anyhow::Result<O> {
  Err(anyhow::anyhow!("{}", msg.into()))
}
