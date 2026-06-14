//! Hand-written runtime support for the generated Aevo client.
//!
//! The generated endpoint methods (in `endpoints.rs`) are implemented as an
//! `impl AevoClient` block and dispatch through [`AevoClient::send`].

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Error;

/// Default Aevo REST API base URL (mainnet).
pub const DEFAULT_BASE_URL: &str = "https://api.aevo.xyz";

/// Default Aevo REST API base URL (testnet).
pub const TESTNET_BASE_URL: &str = "https://api-testnet.aevo.xyz";

/// Asynchronous client for the Aevo REST API.
///
/// Construct one with [`AevoClient::new`] for the default mainnet endpoint, or
/// use [`AevoClient::builder`] to customise the base URL and authentication
/// headers.
#[derive(Debug, Clone)]
pub struct AevoClient {
    http: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl AevoClient {
    /// Create a new client pointed at the default mainnet base URL.
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("default base url is always valid")
    }

    /// Start building a customised client.
    pub fn builder() -> AevoClientBuilder {
        AevoClientBuilder::default()
    }

    /// The base URL this client targets.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Send a request and deserialize the JSON response.
    ///
    /// This is the single dispatch point used by every generated endpoint
    /// method. `query` is serialized into the URL query string and `body` is
    /// serialized as a JSON request body; pass `None` (with a concrete type,
    /// e.g. `None::<&()>`) to omit either.
    pub async fn send<Q, B, R>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        auth: bool,
    ) -> Result<R, Error>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, url);

        if let Some(q) = query {
            req = req.query(q);
        }
        if let Some(b) = body {
            req = req.json(b);
        }
        if auth {
            if let Some(key) = &self.api_key {
                req = req.header("AEVO-KEY", key);
            }
            if let Some(secret) = &self.api_secret {
                req = req.header("AEVO-SECRET", secret);
            }
        }

        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                body: text,
            });
        }

        // An empty body deserializes to `()` or `Option::None`; guard against it.
        if text.is_empty() {
            // `serde_json` can deserialize `null` into unit/Option types.
            return Ok(serde_json::from_str("null")?);
        }
        Ok(serde_json::from_str(&text)?)
    }
}

impl Default for AevoClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for [`AevoClient`].
#[derive(Debug, Default, Clone)]
pub struct AevoClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
    http: Option<reqwest::Client>,
}

impl AevoClientBuilder {
    /// Override the base URL (defaults to [`DEFAULT_BASE_URL`]).
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the `AEVO-KEY` header used for authenticated (private) endpoints.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the `AEVO-SECRET` header used for authenticated (private) endpoints.
    pub fn api_secret(mut self, api_secret: impl Into<String>) -> Self {
        self.api_secret = Some(api_secret.into());
        self
    }

    /// Provide a pre-configured [`reqwest::Client`] (timeouts, proxies, ...).
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    /// Build the [`AevoClient`].
    pub fn build(self) -> Result<AevoClient, Error> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        // Normalise: strip a single trailing slash so `path` joins cleanly.
        let base_url = base_url.strip_suffix('/').map(str::to_string).unwrap_or(base_url);
        if base_url.is_empty() {
            return Err(Error::InvalidBaseUrl(base_url));
        }
        Ok(AevoClient {
            http: self.http.unwrap_or_default(),
            base_url,
            api_key: self.api_key,
            api_secret: self.api_secret,
        })
    }
}
