use std::error;
use std::{io, fmt};

use hyper;
use serde_json;
use types::kitsu::ApiError;

#[derive(Debug)]
pub enum Error {
  // IO Error
  Io(io::Error),

  // Hyper Error
  Hyper(hyper::Error),

  // JSON Error
  Json(serde_json::Error),

  // Kitsu API Error
  Kitsu(KitsuError),

  // Telegram API Error
  Telegram(TelegramError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Error::Io(ref err) => write!(f, "{}", err),
      Error::Json(ref err) => write!(f, "{}", err),
      Error::Hyper(ref err) => write!(f, "{}", err),
      Error::Kitsu(ref err) => write!(f, "{}", err),
      Error::Telegram(ref err) => write!(f, "{}", err),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Io(ref err) => err.description(),
      Error::Json(ref err) => err.description(),
      Error::Hyper(ref err) => err.description(),
      Error::Kitsu(ref err) => err.description(),
      Error::Telegram(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Io(ref err) => Some(err),
      Error::Json(ref err) => Some(err),
      Error::Hyper(ref err) => Some(err),
      Error::Kitsu(ref err) => Some(err),
      Error::Telegram(ref err) => Some(err),
    }
  }
}

macro_rules! impl_from {
  ($v:path, $t:ty) => {
    impl From<$t> for Error {
      fn from(err: $t) -> Self {
        $v(err)
      }
    }
  }
}

impl_from!(Error::Io, io::Error);
impl_from!(Error::Kitsu, KitsuError);
impl_from!(Error::Hyper, hyper::Error);
impl_from!(Error::Json, serde_json::Error);
impl_from!(Error::Telegram, TelegramError);

#[derive(Debug)]
pub struct KitsuError {
  pub errors: Vec<ApiError>,
}

impl error::Error for KitsuError {
  fn description(&self) -> &str {
    "Kits API Error"
  }
}

impl fmt::Display for KitsuError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, ": ")?;
    for error in &self.errors {
      write!(f, "[{}: {}]", error.title, error.detail)?;
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct TelegramError {
  pub description: String,
}

impl error::Error for TelegramError {
  fn description(&self) -> &str {
    "Telegram API Error"
  }
}

impl fmt::Display for TelegramError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, ": {}", self.description)?;
    Ok(())
  }
}
