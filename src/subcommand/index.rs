use super::*;

pub fn run(options: Options) -> Result {
  let index = Index::open(&options)?;

  index.update()?;

  Ok(())
}
