use core::marker::PhantomData;

/// Generic database, support generic key and value.
/// 
/// - If you need transaction support, use implementations in [`transaction`](crate::transaction) module instead.
/// 
/// - If your key and value are just bytes, use the [`Db`](super::Db) instead.
pub struct GenericDb<K, V> {
  _k: PhantomData<K>,
  _v: PhantomData<V>,
}