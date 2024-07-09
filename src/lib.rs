//! A template for creating Rust open-source repo on GitHub
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc as std;

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("limedb requires either the 'std' or 'alloc' feature to be enabled.");

#[doc(inline)]
pub use skl::{Ascend, Descend, Comparator};

/// Errors for the database.
pub mod error;

/// Options for configuring the database.
pub mod options;

mod db;
pub use db::*;
/// Transaction database.
pub mod transaction;

mod manifest;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Fid(u32);

impl Fid {
  #[inline]
  const fn next(&self) -> Self {
    Self(self.0 + 1)
  }

  #[inline]
  fn next_assign(&mut self) {
    self.0 += 1;
  }

  #[inline]
  fn max(&self, other: Self) -> Self {
    Self(self.0.max(other.0))
  }

  #[inline]
  fn max_assign(&mut self, other: Self) {
    self.0 = self.0.max(other.0);
  }
}