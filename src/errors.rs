use hound;

// We derive `Debug` because all types should probably derive `Debug`.
#[derive(Debug)]
pub enum SonogramError {
  Hound(hound::Error),
}

impl From<hound::Error> for SonogramError {
  fn from(err: hound::Error) -> SonogramError {
    SonogramError::Hound(err)
  }
}
