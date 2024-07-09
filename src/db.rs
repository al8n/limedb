use super::{manifest::ManifestFile, *};

use parking_lot::Mutex;
use skl::{map::Entry, SkipMap};

mod generic;
pub use generic::GenericDb;

const VERSION0: u64 = 0;

struct Wal<C = Ascend> {
  old: SkipMap<u64, C>,
  latest: Option<SkipMap<u64, C>>,
}

impl<C: Comparator + Clone> Wal<C> {
  #[inline]
  fn contains(&self, key: &[u8]) -> bool {
    self.map().contains_key(VERSION0, &key)
  }

  #[inline]
  fn get(&self, key: &[u8]) -> Option<Entry<u64, C>> {
    self.map().get(VERSION0, &key).map(|entry| entry.to_owned())
  }

  #[inline]
  fn map(&self) -> &SkipMap<u64, C> {
    self.latest.as_ref().unwrap_or(&self.old)
  }
}


/// A plain database, without transaction support.
/// 
/// - If you need transaction support, use implementations in [`transaction`](crate::transaction) module instead.
/// 
/// - If your key and value are structured, use the [`GenericDb`](generic::GenericDb) instead.
pub struct Db<C = Ascend> {
  manifest: Mutex<ManifestFile>,
  wal: Wal<C>,
}

impl<C: Comparator + Clone> Db<C> {
  /// a
  pub fn insert(&self, key: &[u8], value: &[u8]) -> Result<(), ()> {
    Ok(())
  }

  /// Returns `true` if the database contains the key.
  #[inline]
  pub fn contains(&self, key: &[u8]) -> bool {
    self.wal.contains(key)
  }

  /// Get the value of the key.
  #[inline]
  pub fn get<'a, 'b: 'a>(&'a self, key: &'b [u8]) -> Option<Entry<u64, C>> {
    self.wal.get(key)
  }
}
