//! # aevo-sdk
//!
//! Asynchronous Rust client for the [Aevo](https://aevo.xyz) exchange REST API.
//!
//! The data models in [`models`] and the endpoint methods on [`AevoClient`]
//! (in the `endpoints` module) are **generated** from Aevo's official OpenAPI
//! documentation by the `aevo-codegen` crate. Do not edit those files by hand;
//! re-run the generator instead.
//!
//! ```no_run
//! # async fn run() -> Result<(), aevo_sdk::Error> {
//! use aevo_sdk::AevoClient;
//!
//! let client = AevoClient::new();
//! // Public endpoint example (generated):
//! // let time = client.get_time().await?;
//! # Ok(())
//! # }
//! ```

pub mod error;
mod http;

pub mod models;
mod endpoints;

pub use error::Error;
pub use http::{AevoClient, AevoClientBuilder, DEFAULT_BASE_URL, TESTNET_BASE_URL};
