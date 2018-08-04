//! Rust implementation of Facebook's DataLoader using futures and tokio.
#![warn(missing_docs, deprecated)]
extern crate futures;
extern crate tokio_core;

use futures::Future;

/// Helpers for cached load operations.
pub mod cached;
/// Helpers for non-cached load operations.
pub mod non_cached;
#[cfg(test)]
mod tests;

pub use non_cached::*;

/// An error wrapping type, representing the state of a failed request.
#[derive(Clone, PartialEq, Debug)]
pub enum LoadError<E> {
    /// Sender dropped connection before the request could be completed.
    SenderDropped,
    /// If the count of returned values does not match the number of keys.
    UnequalKeyValueSize {
        /// The number of expected keys for a given request.
        key_count: usize,
        /// The number of expected values for a given request.
        value_count: usize,
    },
    /// Undifferentiated error type returned by `BatchFn`.
    BatchFn(E),
}

/// A type alias for trait objects returned by `BatchFn`.
pub type BatchFuture<V, E> = Box<Future<Item = Vec<V>, Error = E>>;

/// Shared logic for batch loaders.
pub trait BatchFn<K, V> {
    /// Errors producedby this `BatchFn`.
    type Error;

    /// Load keys from the configured data source.
    fn load(&self, keys: &[K]) -> BatchFuture<V, Self::Error>;

    /// Limit maximum in-flight requests.
    fn max_batch_size(&self) -> usize {
        200
    }
}
