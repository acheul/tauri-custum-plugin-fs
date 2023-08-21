use serde::{Serializer, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("etc-error: {0}")]
  Etc(String),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      serializer.serialize_str(self.to_string().as_ref())
  }
}

pub type Result<T> = std::result::Result<T, Error>;