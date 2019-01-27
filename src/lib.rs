//! [![travis-badge][]][travis] [![license-badge][]][license] [![docs-badge][]][docs] [![rust badge]][rust link]
//!
//! An unofficial Rust library acting as a wrapper around the [Discord Bot List]
//! API, offering implementations for both sync and async reqwest (v0.9).
//!
//! ### Compile features
//!
//! - **reqwest-sync-support**: Compliles with sync `reqwest` support (*default*)
//! - **reqwest-async-support**: Compiles with async `reqwest` support
//!
//! Note that `reqwest-async-support` requires nightly for the unstable
//! `core::future` API.
//!
//! ### Installation
//!
//! This library requires at least Rust 1.31.0.
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! discord-bots-org = "0.1"
//! ```
//!
//! To enable both async `reqwest-async` and `reqwest-sync` support:
//!
//! ```toml
//! [dependencies.discord-bots-org]
//! version = "0.1"
//! features = ["reqwest-async-support", "reqwest-sync-support"]
//! ```
//!
//! To enable `reqwest-async-support` but not `reqwest-sync-support`:
//!
//! ```toml
//! [dependencies.discord-bots-org]
//! version = "0.1"
//! default-features = false
//! features = ["reqwest-async-support"]
//! ```
//!
//! ### Examples
//!
//! Using reqwest synchronously, request a bot by ID:
//!
//! ```rust,no_run
//! extern crate discord_bots_org;
//! extern crate reqwest;
//!
//! use discord_bots_org::ReqwestSyncClient as ApiClient;
//! use reqwest::Client as ReqwestClient;
//! use std::{
//!     error::Error,
//!     sync::Arc,
//! };
//!
//! fn main() -> Result<(), Box<Error>> {
//!     // Create the Reqwest Client.
//!     let reqwest_client = Arc::new(ReqwestClient::new());
//!
//!     // Create the API Client.
//!     let client = ApiClient::new(Arc::clone(&reqwest_client));
//!
//!     // Request the bot information.
//!     let bot = client.get_bot(270_198_738_570_444_801)?;
//!
//!     println!("The bot's name is: {}", bot.username);
//!
//!     Ok(())
//! }
//! ```
//!
//! Examples are sparse for asynchronous reqwest. It is assumed that if you're using
//! unstable asynchronous Rust APIs that you're already proficient in them.
//!
//! For more examples, refer to the [examples] folder.
//!
//! ### License
//!
//! ISC. View the full license [here][license].
//!
//! [LICENSE.md]: https://github.com/dabbotorg/discord-bots-org.rs/blob/master/LICENSE.md
//! [Discord Bot List]: https://discordbots.org
//! [crates.io]: https://crates.io/crates/discord-bots-org
//! [docs]: https://docs.rs/discord-bots-org
//! [docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
//! [examples]: https://github.com/dabbotorg/discord-bots-org.rs/tree/master/examples
//! [license]: https://github.com/dabbotorg/discord-bots-org.rs/blob/master/LICENSE.md
//! [license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [rust badge]: https://img.shields.io/badge/rust-1.31.0+-93450a.svg?style=flat-square
//! [rust link]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
//! [travis]: https://travis-ci.org/dabbotorg/discord-bots-org.rs
//! [travis-badge]: https://img.shields.io/travis/dabbotorg/discord-bots-org.rs.svg?style=flat-square
#![deny(missing_docs)]
#![cfg_attr(
    feature = "futures-preview",
    feature(async_await, await_macro, futures_api)
)]

pub mod bridge;
pub mod builder;
pub mod model;

mod endpoints;
mod error;

pub use self::error::{Error, Result};

#[cfg(feature = "reqwest-async-support")]
pub use self::bridge::reqwest::r#async::Client as ReqwestAsyncClient;

#[cfg(feature = "reqwest-sync-support")]
pub use self::bridge::reqwest::sync::Client as ReqwestSyncClient;
