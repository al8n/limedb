#[cfg(feature = "std")]
use core::convert::Infallible;
#[cfg(feature = "std")]
use skl::either::Either;
#[cfg(feature = "std")]
use super::Manifest;


#[cfg(feature = "std")]
#[derive(Debug)]
pub(crate) enum ManifestRecordError {
  /// Encode buffer is too small.
  EncodeBufferTooSmall,
  /// Not enough bytes to decode.
  NotEnoughBytes,
}

#[cfg(feature = "std")]
impl core::fmt::Display for ManifestRecordError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::EncodeBufferTooSmall => write!(f, "buffer too small to encode manifest record."),
      Self::NotEnoughBytes => write!(f, "not enough bytes to decode manifest record."),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for ManifestRecordError {}

/// Errors for manifest file.
pub struct ManifestError {
  #[cfg(feature = "std")]
  source: Either<Infallible, aol::fs::Error<Manifest>>,
}

impl core::fmt::Debug for ManifestError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    #[cfg(feature = "std")]
    match self.source {
      Either::Left(e) => e.fmt(f),
      Either::Right(ref e) => e.fmt(f),
    }

    #[cfg(not(feature = "std"))]
    write!(f, "ManifestError")
  }
}

#[cfg(feature = "std")]
impl From<aol::fs::Error<Manifest>> for ManifestError {
  fn from(e: aol::fs::Error<Manifest>) -> Self {
    Self {
      source: Either::Right(e),
    }
  }
}

#[cfg(feature = "std")]
impl From<Infallible> for ManifestError {
  fn from(e: Infallible) -> Self {
    Self {
      source: Either::Left(e),
    }
  }
}

impl core::fmt::Display for ManifestError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    #[cfg(feature = "std")]
    match self.source {
      Either::Left(ref e) => e.fmt(f),
      Either::Right(ref e) => e.fmt(f),
    }

    #[cfg(not(feature = "std"))]
    write!(f, "ManifestError")
  }
}

#[cfg(feature = "std")]
impl std::error::Error for ManifestError {}