#[macro_use]
extern crate guid;

//#[cfg(windows)]
mod native;
mod error;
mod checks;

//#[cfg(windows)]
mod api;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

pub use error::{
  Result,
  Error
};

pub use api::{
  Application,
  Project,
  Version,
  Category,
  EntityTypes,
  Entity,
  EntityCollection
};

pub mod meta {
  /// Renga version this crate is based on.
  pub static RENGA_VERSION: crate::Version = crate::Version::new(8, 1, 0);
}