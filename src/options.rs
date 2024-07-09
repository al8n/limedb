
/// The options for opening a manifest file.
#[viewit::viewit(getters(style = "move"), setters(prefix = "with"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestOptions {
  /// The version of the lime manifest file. Default is `0`.
  #[viewit(
    getter(const, attrs(doc = "Returns the version of the manifest file.")),
    setter(attrs(doc = "Sets the version of the manifest file."))
  )]
  version: u16,
  /// The rewrite threshold for the manifest file. Default is `10000`.
  /// 
  /// If the manifest file exceeds this threshold, it will be rewritten.
  #[viewit(
    getter(
      const,
      attrs(doc = "Returns the rewrite threshold for the manifest file.")
    ),
    setter(attrs(doc = "Sets the rewrite threshold for the manifest file."))
  )]
  rewrite_threshold: usize,

  /// The maximum size for the manifest file. Default is `1KB`.
  /// 
  /// If the manifest file exceeds this size, it will be rewritten.
  #[viewit(
    getter(
      const,
      attrs(doc = "Returns the maximum size for the manifest file.")
    ),
    setter(attrs(doc = "Sets the maximum size for the manifest file."))
  )]
  maximum_size: usize,
}

impl Default for ManifestOptions {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl ManifestOptions {
  /// Creates a new manifest options with the default values.
  #[inline]
  pub const fn new() -> Self {
    Self {
      version: 0,
      rewrite_threshold: 10000,
      maximum_size: 1024,
    }
  }
}