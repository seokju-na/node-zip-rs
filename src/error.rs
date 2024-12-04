#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Zip(#[from] zip::result::ZipError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Napi(#[from] napi::Error),
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    match value {
      Error::Zip(e) => napi::Error::new(napi::Status::GenericFailure, format!("zip error: {e}")),
      Error::Io(e) => napi::Error::new(napi::Status::GenericFailure, format!("io error: {e}")),
      Error::Napi(e) => e,
    }
  }
}

impl From<Error> for napi::JsError {
  fn from(value: Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
