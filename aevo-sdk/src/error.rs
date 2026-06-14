//! Error types for the Aevo SDK.

/// Errors that can occur when calling the Aevo API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The underlying HTTP transport failed (connection, timeout, TLS, ...).
    #[error("http transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// The API returned a non-success status code.
    #[error("api error (status {status}): {body}")]
    Api {
        /// HTTP status code returned by the server.
        status: u16,
        /// Raw response body returned by the server.
        body: String,
    },

    /// Failed to (de)serialize a request or response body.
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// An invalid base URL was supplied to the client builder.
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(String),
}
