#[macro_use]
extern crate guid;

//#[cfg(windows)]
mod native;
mod error;

//#[cfg(windows)]
mod api;

pub use error::{
  Result,
  Error
};

pub use api::{
  Application,
  Version,
  Category,
  EntityTypes
};

pub mod meta {
  pub static RENGA_VERSION: crate::Version = crate::Version::new(8, 1, 0);
}