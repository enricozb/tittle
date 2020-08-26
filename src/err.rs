/// Convenience `Err` creation function.
pub fn err<S: Into<String>>(msg: S) -> anyhow::Result<()> {
  Err(anyhow::anyhow!("{}", msg.into()))
}
