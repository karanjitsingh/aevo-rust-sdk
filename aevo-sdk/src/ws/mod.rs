//! Hand-written asynchronous WebSocket client runtime for Aevo.
//!
//! The strongly-typed per-channel message structs in [`messages`] are
//! **generated** by `aevo-codegen` from the documentation's response examples.
//! This module provides the connection, subscription and message-reading
//! plumbing that consumes them.
//!
//! ```no_run
//! # async fn run() -> Result<(), aevo_sdk::Error> {
//! use aevo_sdk::ws::{WsClient, Channel};
//!
//! let mut ws = WsClient::connect().await?;
//! ws.subscribe(&["fills".to_string()]).await?;
//! while let Some(msg) = ws.next_message().await {
//!     let value = msg?;
//!     // value["channel"], value["data"], ... or deserialize into a
//!     // generated `messages::*` struct.
//! }
//! # Ok(())
//! # }
//! ```

pub mod messages;

use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

use crate::error::Error;

pub use messages::Channel;

/// Default Aevo WebSocket endpoint (mainnet).
pub const DEFAULT_WS_URL: &str = "wss://ws.aevo.xyz";
/// Aevo WebSocket endpoint (testnet).
pub const TESTNET_WS_URL: &str = "wss://ws-testnet.aevo.xyz";

type WsStream = tokio_tungstenite::WebSocketStream<
    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
>;

/// An asynchronous Aevo WebSocket client.
pub struct WsClient {
    stream: WsStream,
}

impl WsClient {
    /// Connect to the default mainnet WebSocket endpoint.
    pub async fn connect() -> Result<Self, Error> {
        Self::connect_to(DEFAULT_WS_URL).await
    }

    /// Connect to a specific WebSocket URL (e.g. [`TESTNET_WS_URL`]).
    pub async fn connect_to(url: &str) -> Result<Self, Error> {
        let (stream, _resp) = tokio_tungstenite::connect_async(url)
            .await
            .map_err(|e| Error::Api {
                status: 0,
                body: format!("websocket connect failed: {e}"),
            })?;
        Ok(Self { stream })
    }

    /// Send a raw JSON value as a text frame.
    pub async fn send_json<T: Serialize>(&mut self, value: &T) -> Result<(), Error> {
        let text = serde_json::to_string(value)?;
        self.stream
            .send(Message::Text(text))
            .await
            .map_err(ws_err)
    }

    /// Subscribe to one or more channels.
    pub async fn subscribe(&mut self, channels: &[String]) -> Result<(), Error> {
        self.send_json(&json!({ "op": "subscribe", "data": channels }))
            .await
    }

    /// Unsubscribe from one or more channels.
    pub async fn unsubscribe(&mut self, channels: &[String]) -> Result<(), Error> {
        self.send_json(&json!({ "op": "unsubscribe", "data": channels }))
            .await
    }

    /// Authenticate the connection using an API key/secret pair.
    pub async fn auth(&mut self, key: &str, secret: &str) -> Result<(), Error> {
        self.send_json(&json!({
            "op": "auth",
            "data": { "key": key, "secret": secret }
        }))
        .await
    }

    /// Read the next message from the socket, parsed as a JSON value.
    ///
    /// Returns `None` once the stream is closed. Non-data frames (ping/pong,
    /// binary, close) are handled transparently and skipped; pings are
    /// answered with a pong automatically.
    pub async fn next_message(&mut self) -> Option<Result<serde_json::Value, Error>> {
        loop {
            match self.stream.next().await {
                Some(Ok(Message::Text(text))) => {
                    return Some(serde_json::from_str(&text).map_err(Error::from));
                }
                Some(Ok(Message::Binary(bin))) => {
                    return Some(serde_json::from_slice(&bin).map_err(Error::from));
                }
                Some(Ok(Message::Ping(payload))) => {
                    // Best-effort pong; ignore send errors here.
                    let _ = self.stream.send(Message::Pong(payload)).await;
                    continue;
                }
                Some(Ok(Message::Pong(_))) | Some(Ok(Message::Frame(_))) => continue,
                Some(Ok(Message::Close(_))) | None => return None,
                Some(Err(e)) => return Some(Err(ws_err(e))),
            }
        }
    }

    /// Read the next message and attempt to deserialize it into a typed
    /// generated message struct (or any `Deserialize` type).
    pub async fn next_typed<T: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Option<Result<T, Error>> {
        match self.next_message().await? {
            Ok(value) => Some(serde_json::from_value(value).map_err(Error::from)),
            Err(e) => Some(Err(e)),
        }
    }
}

fn ws_err(e: tokio_tungstenite::tungstenite::Error) -> Error {
    Error::Api {
        status: 0,
        body: format!("websocket error: {e}"),
    }
}
