use std::error;
use std::{io, fmt};

use hyper;
use serde_json;

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

  // Database Error
  Database(DatabaseError),

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
      Error::Database(ref err) => write!(f, "{}", err),
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
      Error::Database(ref err) => err.description(),
      Error::Telegram(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Io(ref err) => Some(err),
      Error::Json(ref err) => Some(err),
      Error::Hyper(ref err) => Some(err),
      Error::Kitsu(ref err) => Some(err),
      Error::Database(ref err) => Some(err),
      Error::Telegram(ref err) => Some(err),
    }
  }
}

#[derive(Debug)]
pub struct KitsuError {
  pub description: String,
}

#[derive(Debug)]
pub struct TelegramError {
  pub description: String,
}

#[derive(Debug)]
pub struct DatabaseError {
  pub description: String,
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

macro_rules! impl_error {
  ($v:ty, $t:expr) => {
    impl error::Error for $v {
      fn description(&self) -> &str {
        $t
      }
    }
  }
}

macro_rules! impl_display {
  ($v:ty) => {
    impl fmt::Display for $v {
      fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, ": {}", self.description)?;
        Ok(())
      }
    }
  }
}

impl_from!(Error::Io, io::Error);
impl_from!(Error::Kitsu, KitsuError);
impl_from!(Error::Hyper, hyper::Error);
impl_from!(Error::Json, serde_json::Error);
impl_from!(Error::Database, DatabaseError);
impl_from!(Error::Telegram, TelegramError);

impl_display!(KitsuError);
impl_display!(TelegramError);
impl_display!(DatabaseError);

impl_error!(KitsuError, "Kits API Error");
impl_error!(TelegramError, "Telegram API Error");
impl_error!(DatabaseError, "Database Error");
