//! Bridging support between the library and various HTTP clients.

#[cfg(any(feature = "reqwest-sync-support", feature = "reqwest-async-support"))]
pub mod reqwest;
