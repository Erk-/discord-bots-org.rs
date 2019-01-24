//! Sync and async Reqwest implementations.

#[cfg(feature = "reqwest-async-support")]
pub mod r#async;

#[cfg(feature = "reqwest-sync-support")]
pub mod sync;
