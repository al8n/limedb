use aol::{memory::Snapshot, Entry};

use super::*;

impl Snapshot for Manifest {
  type Data = ManifestRecord;

  type Options = ManifestOptions;

  type Error = core::convert::Infallible;

  fn new(opts: Self::Options) -> Result<Self, Self::Error> {
    Ok(Self {
      last: ManifestRecord::default(),
      opts,
    })
  }

  fn options(&self) -> &Self::Options {
    &self.opts
  }

  fn should_rewrite(&self) -> bool {
    false
  }

  fn insert(&mut self, entry: Entry<Self::Data>) -> Result<(), Self::Error> {
    let record = entry.data();
    self.last.last_fid.max_assign(record.last_fid);
    self.last.last_compact_version = self.last.last_compact_version.max(record.last_compact_version);
    Ok(())
  }

  fn insert_batch(&mut self, entries: impl Iterator<Item = Entry<Self::Data>>,) -> Result<(), Self::Error> {
    for entry in entries {
      self.insert(entry)?;
    }
    Ok(())
  }

  fn into_iter(self) -> impl Iterator<Item = Entry<Self::Data>> {
    core::iter::once(Entry::creation(self.last))
  }
}

pub(crate) struct MemoryManifest {
  manifest: Manifest,
}

impl MemoryManifest {
  #[inline]
  pub fn new(opts: ManifestOptions) -> Self {
    Self {
      manifest: Manifest {
        last: ManifestRecord::default(),
        opts,
      },
    }
  }

  #[inline]
  pub fn append(&mut self, entry: Entry<ManifestRecord>) {
    self.manifest.insert(entry).unwrap();
  }

  #[inline]
  pub fn append_batch(&mut self, entries: Vec<Entry<ManifestRecord>>) {
    self.manifest.insert_batch(entries.into_iter()).unwrap();
  }

  #[inline]
  pub const fn last_record(&self) -> ManifestRecord {
    self.manifest.last
  }
}
