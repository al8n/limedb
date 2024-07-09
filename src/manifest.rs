use super::{Fid, options::ManifestOptions};

use core::mem;

#[cfg(feature = "std")]
mod disk;

mod memory;

mod error;
use aol::Entry;
pub use error::ManifestError;

const MANIFEST_REWRITE_THRESHOLD: usize = 10000;
const ENCODED_SIZE: usize = mem::size_of::<Fid>() + mem::size_of::<u64>();

struct Manifest {
  last: ManifestRecord,
  opts: ManifestOptions,
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ManifestRecord {
  last_fid: Fid,
  last_compact_version: u64,
}


impl aol::Data for ManifestRecord {
  type Error = error::ManifestRecordError;

  fn encoded_size(&self) -> usize {
    mem::size_of::<Fid>() + mem::size_of::<u64>()
  }

  fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
    if buf.len() < ENCODED_SIZE {
      return Err(error::ManifestRecordError::EncodeBufferTooSmall);
    }

    let (fid_bytes, compact_version_bytes) = buf.split_at_mut(mem::size_of::<Fid>());
    fid_bytes.copy_from_slice(&self.last_fid.0.to_le_bytes());
    compact_version_bytes.copy_from_slice(&self.last_compact_version.to_le_bytes());
    Ok(ENCODED_SIZE)
  }

  fn decode(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
    if buf.len() < ENCODED_SIZE {
      return Err(error::ManifestRecordError::NotEnoughBytes);
    }

    let (fid_bytes, compact_version_bytes) = buf.split_at(mem::size_of::<Fid>());
    let fid = Fid(u32::from_le_bytes(fid_bytes.try_into().unwrap()));
    let compact_version = u64::from_le_bytes(compact_version_bytes.try_into().unwrap());
    Ok((ENCODED_SIZE, Self { last_fid: fid, last_compact_version: compact_version }))
  }
}

#[derive(derive_more::From)]
enum ManifestFileKind {
  Memory(memory::MemoryManifest),
  #[cfg(feature = "std")]
  Disk(disk::DiskManifestFile),
}

pub(crate) struct ManifestFile {
  kind: ManifestFileKind,
}

impl ManifestFile {
  #[cfg(feature = "std")]
  pub fn open<P: AsRef<std::path::Path>>(
    dir: Option<P>,
    opts: ManifestOptions,
  ) -> Result<Self, ManifestError> {
    match dir {
      Some(dir) => disk::DiskManifestFile::open(dir, opts.rewrite_threshold, opts.version)
        .map(|file| Self {
          kind: ManifestFileKind::Disk(file),
        })
        .map_err(Into::into),
      None => Ok(Self {
        kind: ManifestFileKind::Memory(memory::MemoryManifest::new(opts)),
      }),
    }
  }

  #[cfg(not(feature = "std"))]
  pub fn open() -> Result<Self, ManifestError> {
    Ok(Self {
      kind: ManifestFileKind::Memory(memory::MemoryManifest::new()),
    })
  }

  #[inline]
  pub(crate) fn append(&mut self, ent: Entry<ManifestRecord>) -> Result<(), ManifestError> {
    match &mut self.kind {
      ManifestFileKind::Memory(m) => {
        m.append(ent);
        Ok(())
      }
      #[cfg(feature = "std")]
      ManifestFileKind::Disk(d) => d.append(ent).map_err(Into::into),
    }
  }

  #[inline]
  pub(crate) fn append_batch(&mut self, entries: Vec<Entry<ManifestRecord>>) -> Result<(), ManifestError> {
    match &mut self.kind {
      ManifestFileKind::Memory(m) => {
        m.append_batch(entries);
        Ok(())
      }
      #[cfg(feature = "std")]
      ManifestFileKind::Disk(d) => d.append_batch(entries).map_err(Into::into),
    }
  }

  #[inline]
  pub(crate) const fn last_record(&self) -> ManifestRecord {
    match &self.kind {
      ManifestFileKind::Memory(m) => m.last_record(),
      #[cfg(feature = "std")]
      ManifestFileKind::Disk(d) => d.last_record(),
    }
  }
}