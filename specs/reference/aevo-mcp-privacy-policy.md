> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Aevo MCP Privacy Policy

This Privacy Policy describes how the AEVO MCP Server handles data when you use it to interact with the [AEVO](https://app.aevo.xyz) decentralized derivatives exchange via the Model Context Protocol (MCP).

## 1. Introduction

This Privacy Policy describes how the AEVO MCP Server handles data when you use it to interact with the [AEVO](https://app.aevo.xyz) decentralized derivatives exchange via the Model Context Protocol (MCP).

The Server is open source. You can review the complete source code at [github.com/ribbon-finance/aevo-mcp](https://github.com/ribbon-finance/aevo-mcp).

### Tool call parameters

Each tool call may include parameters such as instrument names, order details (price, amount, side), and query filters (time ranges, asset names). These are used solely to construct the corresponding AEVO API request.

### Session metadata

The MCP protocol provides session identifiers (session ID, client ID) used to isolate credentials between concurrent sessions. No additional metadata is collected.

## 2. Security

* **Session isolation** — Each MCP session uses a unique key derived from the session or client ID; credentials are never shared across sessions.
* **Credential masking** — Internal representations of credentials are masked (e.g., `ab***yz`) to prevent accidental exposure in logs or debug output.
* **Error sanitization** — API error messages are truncated and hex patterns (potential key material) are redacted before being returned to clients.
* **Input validation** — Tool inputs are validated and sanitized to prevent prompt injection and parameter manipulation.
* **HMAC request signing** — All authenticated API requests use HMAC-SHA256 signatures rather than transmitting secrets directly.
* **Rate limiting** — Per-session and global rate limits protect against abuse (configurable read/write/destructive rates).

## 3. What We Do NOT Collect

* No cookies, local storage, or browser state
* No IP addresses (the Server itself does not log them)
* No analytics or telemetry
* No usage tracking or behavioral profiling
* No personal information beyond what you explicitly provide via tool calls

## 4. Your Rights

* **Clear credentials** — Call `aevo_clear_auth` to immediately remove your session credentials.
* **Session expiry** — Credentials are automatically deleted after 12 hours of inactivity, even without explicit action.
* **Self-host** — The Server is open source. You can run it locally to maintain full control over your data.
* **Inspect the code** — The complete source code is available for audit at [github.com/ribbon-finance/aevo-mcp](https://github.com/ribbon-finance/aevo-mcp).

## 5. Open Source Transparency

This Server is open source under the MIT License. Every claim in this Privacy Policy can be verified by reviewing the source code. We encourage you to do so.

## 6. Contact

For questions about this Privacy Policy or the Server's data handling:

* **GitHub Issues:** [github.com/ribbon-finance/aevo-mcp/issues](https://github.com/ribbon-finance/aevo-mcp/issues)
* **AEVO:** [app.aevo.xyz](https://app.aevo.xyz)

## 7. Changes to This Policy

We may update this Privacy Policy to reflect changes in the Server's functionality. Material changes will be noted in the repository's commit history. The "Last updated" date at the top of this document indicates when it was last revised.