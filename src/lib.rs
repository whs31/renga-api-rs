/// This crate provides (incomplete) bindings for the [Renga](https://www.rengabim.com) COM API.
/// 
/// Only Windows is supported at the moment.
/// 
/// See [official documentation](https://help.rengabim.com/) for more details.
/// 
/// # Logging
/// 
/// This crate uses `log` crate interface to log messages.
/// You must provide your own logger implementation in order to see logs.
/// 
/// See [log](https://crates.io/crates/log) for more details.
/// 
/// # Compatibility
/// 
/// Rust version at least **1.65** is required.
/// This crate is compatible with Renga 8.0.0 and higher.
/// 
/// Todo: examples

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
  ProjectTransaction,
  Version,
  Category,
  EntityTypes,
  Entity,
  EntityCollection
};

/// Meta information about this crate.
pub mod meta {
  /// Renga version this crate is based on.
  pub static RENGA_VERSION: crate::Version = crate::Version::new(8, 1, 0);
}