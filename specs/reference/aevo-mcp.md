> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Aevo MCP

Connect AI agents to the Aevo trading platform. Retrieve market data, manage accounts, and execute trades easily

## Authentication

The AEVO MCP server uses session-based authentication. Each MCP client session stores its own AEVO credentials in server memory via the `aevo_authenticate` tool.

### Credentials

All credentials are available at [app.aevo.xyz/settings](https://app.aevo.xyz/settings/api-keys).

**To read account data** (balances, positions, history):

* `api_key` + `api_secret`

**To trade** (create/cancel orders):

* `api_key` + `api_secret` + `wallet_address` + `signing_key_private_key`

### Session Flow

```
1. Connect to the MCP server
2. Call aevo_onboard() — checks if credentials exist
3. Call aevo_authenticate(api_key, api_secret, wallet_address, signing_key_private_key)
4. Use tools normally
5. Session expires automatically after 12 hours
```

Credentials are stored in memory only — never written to disk or logs. Each session is isolated; credentials from one session are never visible to another.

## Hosted Server

### Endpoints

| Environment | MCP Endpoint                       |
| ----------- | ---------------------------------- |
| Mainnet     | `https://mcp.aevo.xyz/mcp`         |
| Testnet     | `https://mcp-testnet.aevo.xyz/mcp` |

### Connecting

**Claude Code:**

```bash
claude mcp add --transport http aevo-trading https://mcp.aevo.xyz/mcp
```

**Claude Desktop** (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "aevo-trading": {
      "type": "http",
      "url": "https://mcp.aevo.xyz/mcp"
    }
  }
}
```

### Self-Hosting

```bash
git clone https://github.com/ribbon-finance/aevo-mcp.git
cd aevo-mcp
docker compose up --build -d
```

Then connect to `http://localhost:8080/mcp`.

For multi-user hosting, do not set `AEVO_API_KEY`/`AEVO_API_SECRET` in the server environment — let each session authenticate independently.

## Tool Reference

| Tool                     |      Auth     | Description                                         |
| ------------------------ | :-----------: | --------------------------------------------------- |
| `aevo_onboard`           |       -       | Check session state and what credentials are needed |
| `aevo_authenticate`      |       -       | Store credentials for the current session           |
| `aevo_clear_auth`        |       -       | Clear session credentials                           |
| `aevo_get_status`        |       -       | Server identity and credential status               |
| `aevo_ping`              |       -       | Server liveness check                               |
| `aevo_healthcheck`       |       -       | API connectivity check                              |
| `aevo_list_assets`       |       -       | List supported assets                               |
| `aevo_list_markets`      |       -       | List available markets                              |
| `aevo_get_orderbook`     |       -       | Orderbook for an instrument                         |
| `aevo_get_instrument`    |       -       | Instrument metadata                                 |
| `aevo_get_funding_rate`  |       -       | Current funding rate                                |
| `aevo_get_statistics`    |       -       | Exchange-wide statistics                            |
| `aevo_get_index_price`   |       -       | Spot reference price                                |
| `aevo_get_server_time`   |       -       | Server timestamp                                    |
| `aevo_get_account`       |      API      | Account balances and margin                         |
| `aevo_get_portfolio`     |      API      | Portfolio with Greeks                               |
| `aevo_get_positions`     |      API      | Open positions                                      |
| `aevo_get_trade_fills`   |      API      | Trade fill history                                  |
| `aevo_get_order_history` |      API      | Order history                                       |
| `aevo_list_orders`       |      API      | Current open orders                                 |
| `aevo_get_order`         |      API      | Single order by ID                                  |
| `aevo_update_leverage`   |      API      | Change leverage for an instrument                   |
| `aevo_build_order`       |    Signing    | Build and sign an order (without submitting)        |
| `aevo_create_order`      | API + Signing | Submit a signed order                               |
| `aevo_cancel_order`      |      API      | Cancel an order                                     |
| `aevo_cancel_orders`     |      API      | Cancel multiple orders                              |
| `aevo_cancel_all_orders` |      API      | Cancel all open orders                              |
| `aevo_register_account`  |    Signing    | Register signing key on-chain                       |

**Auth legend:**

* `-` = no credentials needed (public)
* `API` = requires `api_key` + `api_secret`
* `Signing` = requires `wallet_address` + `signing_key_private_key`
* `API + Signing` = requires all four