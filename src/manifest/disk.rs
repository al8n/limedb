
use std::{fs::OpenOptions, path::Path};

use aol::{fs::{AppendLog, Error, Options, Snapshot}, Entry};

use super::*;

const MANIFEST_FILENAME: &str = "MANIFEST";

impl aol::fs::Snapshot for Manifest {
  type Data = ManifestRecord;

  type Options = ManifestOptions;

  type Error = core::convert::Infallible;

  fn new(opts: Self::Options) -> Result<Self, Self::Error> {
    Ok(Self {
      last: ManifestRecord::default(),
      opts,
    })
  }

  fn should_rewrite(&self, size: u64) -> bool {
    self.opts.rewrite_threshold > MANIFEST_REWRITE_THRESHOLD || size > self.opts.maximum_size as u64
  }

  fn insert(&mut self, entry: Entry<Self::Data>) -> Result<(), Self::Error> {
    let record = entry.data();
    self.last.last_fid.max_assign(record.last_fid);
    self.last.last_compact_version = self.last.last_compact_version.max(record.last_compact_version);
    Ok(())
  }

  fn insert_batch(&mut self, entries: Vec<Entry<Self::Data>>) -> Result<(), Self::Error> {
    for entry in entries {
      self.insert(entry)?;
    }
    Ok(())
  }

  fn clear(&mut self) -> Result<(), Self::Error> {
    self.last = ManifestRecord::default();
    Ok(())
  }
}

pub(super) struct DiskManifestFile {
  log: AppendLog<Manifest>,
}

impl DiskManifestFile {
  /// Open and replay the manifest file.
  pub(super) fn open<P: AsRef<Path>>(
    path: P,
    rewrite_threshold: usize,
    version: u16,
  ) -> Result<Self, Error<Manifest>> {
    let path = path.as_ref().join(MANIFEST_FILENAME);
    let mut open_options = OpenOptions::new();
    open_options.read(true).create(true).append(true);
    let log = AppendLog::open(
      &path,
      ManifestOptions::new().with_rewrite_threshold(rewrite_threshold),
      open_options,
      Options::new().with_magic_version(version),
    )?;

    Ok(Self { log })
  }

  #[inline]
  pub(super) fn append(&mut self, ent: Entry<ManifestRecord>) -> Result<(), Error<Manifest>> {
    self.log.append(ent)
  }

  #[inline]
  pub(super) fn append_batch(&mut self, entries: Vec<Entry<ManifestRecord>>) -> Result<(), Error<Manifest>> {
    self.log.append_batch(entries)
  }

  #[inline]
  pub(super) const fn last_record(&self) -> ManifestRecord {
    self.log.snapshot().last
  }
}