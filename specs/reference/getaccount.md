> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /account

Returns the account's information including API keys, signing keys and positions.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "account_response": {
        "title": "account_response",
        "type": "string",
        "description": "Account's Ethereum address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "account_type_response": {
        "title": "account_type_response",
        "type": "string",
        "description": "Account type. Eg. `STANDARD`",
        "enum": [
          "STANDARD",
          "MARKET_MAKER",
          "MANAGED_ACCOUNT"
        ]
      },
      "adl_rank_response": {
        "title": "adl_rank_response",
        "type": "string",
        "description": "ADL rank of an account Eg. `low`",
        "enum": [
          "na",
          "low",
          "medium",
          "high"
        ]
      },
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "api_key_response": {
        "title": "api_key_response",
        "type": "string",
        "description": "Account's API Key. Eg. `URPtt6eNCXgL8ERuchphUretdaga2smF`",
        "example": "URPtt6eNCXgL8ERuchphUretdaga2smF"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "available_balance_response": {
        "title": "available_balance_response",
        "type": "string",
        "description": "Available balance. Eg. `12.23`",
        "example": "12.23"
      },
      "available_margin_response": {
        "title": "available_margin_response",
        "type": "string",
        "description": "Available margin. Eg. `12.23`",
        "example": "12.23"
      },
      "avg_entry_price_response": {
        "title": "avg_entry_price_response",
        "type": "string",
        "description": "Average entry price. Eg. `12.23`",
        "example": "12.23"
      },
      "balance_response": {
        "title": "balance_response",
        "type": "string",
        "description": "Balance. Eg. `12.23`",
        "example": "12.23"
      },
      "base_isolated_margin_response": {
        "title": "base_isolated_margin_response",
        "type": "string",
        "description": "Base isolated margin allocated from fills only (excludes manual top-ups). Eg. `10000000`",
        "example": "10000000"
      },
      "campaign_sign_ups_response": {
        "title": "campaign_sign_ups_response",
        "type": "array",
        "description": "Array of campaigns that account has signed up for. Eg. `[campaign_1 campaign_2]`",
        "example": [
          "campaign_1",
          "campaign_2"
        ],
        "items": {
          "type": "string"
        }
      },
      "chain_id_response": {
        "title": "chain_id_response",
        "type": "string",
        "description": "The specified chain ID. Eg. `10`",
        "example": "10"
      },
      "collateral_asset_response": {
        "title": "collateral_asset_response",
        "type": "string",
        "description": "Name of the collateral asset. Eg. `USDT`",
        "enum": [
          "USDC",
          "USDT",
          "WETH",
          "WBTC",
          "aeUSD",
          "SDAI",
          "weETH"
        ]
      },
      "collateral_response": {
        "title": "collateral_response",
        "type": "string",
        "description": "Ethereum address of the collateral asset. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "collateral_value_response": {
        "title": "collateral_value_response",
        "type": "string",
        "description": "Value of the collateral in USD Eg. `10423`",
        "example": "10423"
      },
      "collateral_yield_bearing_response": {
        "title": "collateral_yield_bearing_response",
        "type": "boolean",
        "description": "If collateral asset is yield bearing Eg. `false`",
        "example": false
      },
      "created_timestamp_response": {
        "title": "created_timestamp_response",
        "type": "string",
        "description": "Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "credit_response": {
        "title": "credit_response",
        "type": "string",
        "description": "Credits in USDC Eg. `100.00`",
        "example": "100.00"
      },
      "credited_response": {
        "title": "credited_response",
        "type": "boolean",
        "description": "Whether account has been credited using the automatic credit system Eg. `true`",
        "example": true
      },
      "delta_response": {
        "title": "delta_response",
        "type": "string",
        "description": "Option's Delta. Eg. `0.23`",
        "example": "0.23"
      },
      "email_address_response": {
        "title": "email_address_response",
        "type": "string",
        "description": "Email address of an account. Eg. `timothy@ribbon.finance`",
        "example": "timothy@ribbon.finance"
      },
      "equity_response": {
        "title": "equity_response",
        "type": "string",
        "description": "Account's equity. Eg. `12.23`",
        "example": "12.23"
      },
      "error_401_response": {
        "title": "error_401_response",
        "type": "string",
        "description": "Error message. Eg. `UNAUTHORIZED`",
        "example": "UNAUTHORIZED"
      },
      "error_429_response": {
        "title": "error_429_response",
        "type": "string",
        "description": "Error message. Eg. `RATE_LIMIT_EXCEEDED`",
        "example": "RATE_LIMIT_EXCEEDED"
      },
      "error_500_response": {
        "title": "error_500_response",
        "type": "string",
        "description": "Error message. Eg. `INTERNAL_ERROR`",
        "example": "INTERNAL_ERROR"
      },
      "expiry_response": {
        "title": "expiry_response",
        "type": "string",
        "description": "Option expiry in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "gamma_response": {
        "title": "gamma_response",
        "type": "string",
        "description": "Option's Gamma. Eg. `0.23`",
        "example": "0.23"
      },
      "has_been_referred_response": {
        "title": "has_been_referred_response",
        "type": "boolean",
        "description": "Whether account has been referred by another account Eg. `true`",
        "example": true
      },
      "in_liquidation_response": {
        "title": "in_liquidation_response",
        "type": "boolean",
        "description": "True if an account is in liquidation mode. Eg. `false`",
        "example": false
      },
      "initial_margin_response": {
        "title": "initial_margin_response",
        "type": "string",
        "description": "Margin required to keep an open order. Eg. `12.23`",
        "example": "12.23"
      },
      "instrument_id_response": {
        "title": "instrument_id_response",
        "type": "string",
        "description": "Instrument ID number. Eg. `12`",
        "example": "12"
      },
      "instrument_name_response": {
        "title": "instrument_name_response",
        "type": "string",
        "description": "Instrument name. Eg. `ETH-30JUN23-1600-C`",
        "example": "ETH-30JUN23-1600-C"
      },
      "instrument_type_response": {
        "title": "instrument_type_response",
        "type": "string",
        "description": "Type of instrument. Eg. `OPTION`",
        "enum": [
          "OPTION",
          "PERPETUAL",
          "SPOT"
        ]
      },
      "intercom_hash_response": {
        "title": "intercom_hash_response",
        "type": "string",
        "description": "Intercom hash used to verify user identity for intercom messages Eg. `nwiaenainhiw`",
        "example": "nwiaenainhiw"
      },
      "ip_addresses_response": {
        "title": "ip_addresses_response",
        "type": "array",
        "description": "Whitelisted client's IP address for API access. Eg. `[1.1.1.1 2.2.2.2]`",
        "example": [
          "1.1.1.1",
          "2.2.2.2"
        ],
        "items": {
          "type": "string"
        }
      },
      "isolated_margin_response": {
        "title": "isolated_margin_response",
        "type": "string",
        "description": "The change in isolated margin assigned to a position. In 6 decimals fixed number. Negative allowed. Eg. `10000000`",
        "example": "10000000"
      },
      "iv_response": {
        "title": "iv_response",
        "type": "string",
        "description": "Option's implied volatility. Eg. `0.23`",
        "example": "0.23"
      },
      "label_response": {
        "title": "label_response",
        "type": "string",
        "description": "Transfer label. Eg. `Rewards`",
        "example": "Rewards"
      },
      "leverage_response": {
        "title": "leverage_response",
        "type": "string",
        "description": "The leverage of the position, will be shown if the position is isolated. Eg. `3.5`",
        "example": "3.5"
      },
      "limit_price_response": {
        "title": "limit_price_response",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "liquidation_price_response": {
        "title": "liquidation_price_response",
        "type": "string",
        "description": "Liquidation price of the order/position. Eg. `1288.23`",
        "example": "1288.23"
      },
      "maintenance_margin_response": {
        "title": "maintenance_margin_response",
        "type": "string",
        "description": "Maintenance margin. Eg. `12.23`",
        "example": "12.23"
      },
      "maker_fee_response": {
        "title": "maker_fee_response",
        "type": "string",
        "description": "Account's maker fee structure Eg. `0.0003`",
        "example": "0.0003"
      },
      "manual_mode_response": {
        "title": "manual_mode_response",
        "type": "boolean",
        "description": "Manual withdrawal flag Eg. `false`",
        "example": false
      },
      "margin_type_response": {
        "title": "margin_type_response",
        "type": "string",
        "description": "The margin type. Eg. `CROSS`",
        "enum": [
          "CROSS",
          "ISOLATED"
        ]
      },
      "margin_value_response": {
        "title": "margin_value_response",
        "type": "string",
        "description": "Margin value of the collateral in USD after collateral factor adjustments Eg. `10423`",
        "example": "10423"
      },
      "mark_price_response": {
        "title": "mark_price_response",
        "type": "string",
        "description": "Mark price of the instrument. Eg. `12.23`",
        "example": "12.23"
      },
      "name_response": {
        "title": "name_response",
        "type": "string",
        "description": "name Eg. `5732cc1f14e842f393e8cdecb49f02fa`",
        "example": "5732cc1f14e842f393e8cdecb49f02fa"
      },
      "network_address_response": {
        "title": "network_address_response",
        "type": "string",
        "description": "Account's non-eth network address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "network_response": {
        "title": "network_response",
        "type": "string",
        "description": "A non-eth network. Eg. `SUI`",
        "enum": [
          "SOL",
          "SUI"
        ]
      },
      "option_type_response": {
        "title": "option_type_response",
        "type": "string",
        "description": "Type of option contract. Eg. `call`",
        "enum": [
          "put",
          "call"
        ]
      },
      "order_id_response": {
        "title": "order_id_response",
        "type": "string",
        "description": "Order ID is the hash of the order payload Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "order_type_response": {
        "title": "order_type_response",
        "type": "string",
        "description": "Order type. Eg. `limit`",
        "enum": [
          "limit",
          "market"
        ]
      },
      "pending_withdrawals_response": {
        "title": "pending_withdrawals_response",
        "type": "string",
        "description": "Pending withdrawals for a yield vault or strategy Eg. `12`",
        "example": "12"
      },
      "portfolio_response": {
        "title": "portfolio_response",
        "type": "boolean",
        "description": "Portfolio margin. Eg. `false`",
        "example": "false"
      },
      "read_only_response": {
        "title": "read_only_response",
        "type": "boolean",
        "description": "API can only access read-only endpoints if true. Eg. `true`",
        "example": true
      },
      "referrer_response": {
        "title": "referrer_response",
        "type": "string",
        "description": "Referrer of a referee Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "rho_response": {
        "title": "rho_response",
        "type": "string",
        "description": "Option's Rho. Eg. `0.23`",
        "example": "0.23"
      },
      "side_response": {
        "title": "side_response",
        "type": "string",
        "description": "Trade side. Eg. `buy`",
        "enum": [
          "buy",
          "sell"
        ]
      },
      "signing_key_response": {
        "title": "signing_key_response",
        "type": "string",
        "description": "Ethereum address of the signing key. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "stop_response": {
        "title": "stop_response",
        "type": "string",
        "description": "Type of stop order. Eg. `STOP_LOSS`",
        "enum": [
          "STOP_LOSS",
          "TAKE_PROFIT"
        ]
      },
      "strike_response": {
        "title": "strike_response",
        "type": "string",
        "description": "Option strike price. Eg. `2500`",
        "example": "2500"
      },
      "taker_fee_response": {
        "title": "taker_fee_response",
        "type": "string",
        "description": "Account's taker fee structure Eg. `0.0005`",
        "example": "0.0005"
      },
      "theta_response": {
        "title": "theta_response",
        "type": "string",
        "description": "Option's Theta. Eg. `0.23`",
        "example": "0.23"
      },
      "to_response": {
        "title": "to_response",
        "type": "string",
        "description": "Ethereum address to send withdrawals to. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "trigger_response": {
        "title": "trigger_response",
        "type": "string",
        "description": "The price to trigger the stop order at. `stop` is required when `trigger` is specified. Eg. `1836.74`",
        "example": "1836.74"
      },
      "unrealized_pnl_response": {
        "title": "unrealized_pnl_response",
        "type": "string",
        "description": "Unrealized PNL. Eg. `12.23`",
        "example": "12.23"
      },
      "username_response": {
        "title": "username_response",
        "type": "string",
        "description": "Account's auto-generated username based on their Ethereum address. Eg. `officially-evolved-terrier`",
        "example": "officially-evolved-terrier"
      },
      "vega_response": {
        "title": "vega_response",
        "type": "string",
        "description": "Option's Vega. Eg. `0.23`",
        "example": "0.23"
      },
      "withdrawal_id_response": {
        "title": "withdrawal_id_response",
        "type": "string",
        "description": "Unique ID of the withdrawal Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      }
    },
    "securitySchemes": {
      "api_key": {
        "in": "header",
        "name": "AEVO-KEY",
        "type": "apiKey"
      },
      "api_secret": {
        "in": "header",
        "name": "AEVO-SECRET",
        "type": "apiKey"
      }
    }
  },
  "info": {
    "contact": {
      "email": "ken@ribbon.finance",
      "name": "Ribbon Finance",
      "url": "https://ribbon.finance"
    },
    "description": "Aevo",
    "license": {
      "name": "Apache 2.0",
      "url": "http://www.apache.org/licenses/LICENSE-2.0.html"
    },
    "title": "Aevo REST API",
    "version": "1.0.0"
  },
  "openapi": "3.0.0",
  "paths": {
    "/account": {
      "get": {
        "summary": "GET /account",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetAccount",
        "description": "Returns the account's information including API keys, signing keys and positions.",
        "responses": {
          "200": {
            "description": "Account information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetAccount200Response",
                  "type": "object",
                  "properties": {
                    "account": {
                      "$ref": "#/components/schemas/account_response"
                    },
                    "username": {
                      "$ref": "#/components/schemas/username_response"
                    },
                    "equity": {
                      "$ref": "#/components/schemas/equity_response"
                    },
                    "available_balance": {
                      "$ref": "#/components/schemas/available_balance_response"
                    },
                    "available_margin": {
                      "$ref": "#/components/schemas/available_margin_response"
                    },
                    "balance": {
                      "$ref": "#/components/schemas/balance_response"
                    },
                    "signing_keys": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "signing_key": {
                            "$ref": "#/components/schemas/signing_key_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "signing_key",
                          "expiry",
                          "created_timestamp"
                        ]
                      }
                    },
                    "collaterals": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collateral_asset": {
                            "$ref": "#/components/schemas/collateral_asset_response"
                          },
                          "collateral_value": {
                            "$ref": "#/components/schemas/collateral_value_response"
                          },
                          "margin_value": {
                            "$ref": "#/components/schemas/margin_value_response"
                          },
                          "balance": {
                            "$ref": "#/components/schemas/balance_response"
                          },
                          "available_balance": {
                            "$ref": "#/components/schemas/available_balance_response"
                          },
                          "collateral_yield_bearing": {
                            "$ref": "#/components/schemas/collateral_yield_bearing_response"
                          },
                          "pending_withdrawals": {
                            "$ref": "#/components/schemas/pending_withdrawals_response"
                          },
                          "unrealized_pnl": {
                            "$ref": "#/components/schemas/unrealized_pnl_response"
                          }
                        },
                        "required": [
                          "collateral_asset",
                          "collateral_value",
                          "margin_value",
                          "balance",
                          "available_balance",
                          "collateral_yield_bearing",
                          "pending_withdrawals",
                          "unrealized_pnl"
                        ]
                      }
                    },
                    "api_keys": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "name": {
                            "$ref": "#/components/schemas/name_response"
                          },
                          "api_key": {
                            "$ref": "#/components/schemas/api_key_response"
                          },
                          "ip_addresses": {
                            "$ref": "#/components/schemas/ip_addresses_response"
                          },
                          "read_only": {
                            "$ref": "#/components/schemas/read_only_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "api_key",
                          "read_only",
                          "created_timestamp"
                        ]
                      }
                    },
                    "positions": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "instrument_id": {
                            "$ref": "#/components/schemas/instrument_id_response"
                          },
                          "instrument_name": {
                            "$ref": "#/components/schemas/instrument_name_response"
                          },
                          "instrument_type": {
                            "$ref": "#/components/schemas/instrument_type_response"
                          },
                          "option": {
                            "type": "object",
                            "properties": {
                              "strike": {
                                "$ref": "#/components/schemas/strike_response"
                              },
                              "option_type": {
                                "$ref": "#/components/schemas/option_type_response"
                              },
                              "expiry": {
                                "$ref": "#/components/schemas/expiry_response"
                              },
                              "iv": {
                                "$ref": "#/components/schemas/iv_response"
                              },
                              "delta": {
                                "$ref": "#/components/schemas/delta_response"
                              },
                              "theta": {
                                "$ref": "#/components/schemas/theta_response"
                              },
                              "vega": {
                                "$ref": "#/components/schemas/vega_response"
                              },
                              "rho": {
                                "$ref": "#/components/schemas/rho_response"
                              },
                              "gamma": {
                                "$ref": "#/components/schemas/gamma_response"
                              }
                            },
                            "required": [
                              "strike",
                              "option_type",
                              "expiry",
                              "iv",
                              "delta",
                              "theta",
                              "vega",
                              "rho",
                              "gamma"
                            ]
                          },
                          "iv": {
                            "$ref": "#/components/schemas/iv_response"
                          },
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "side": {
                            "$ref": "#/components/schemas/side_response"
                          },
                          "mark_price": {
                            "$ref": "#/components/schemas/mark_price_response"
                          },
                          "avg_entry_price": {
                            "$ref": "#/components/schemas/avg_entry_price_response"
                          },
                          "unrealized_pnl": {
                            "$ref": "#/components/schemas/unrealized_pnl_response"
                          },
                          "initial_margin": {
                            "$ref": "#/components/schemas/initial_margin_response"
                          },
                          "maintenance_margin": {
                            "$ref": "#/components/schemas/maintenance_margin_response"
                          },
                          "margin_type": {
                            "$ref": "#/components/schemas/margin_type_response"
                          },
                          "liquidation_price": {
                            "$ref": "#/components/schemas/liquidation_price_response"
                          },
                          "isolated_margin": {
                            "$ref": "#/components/schemas/isolated_margin_response"
                          },
                          "base_isolated_margin": {
                            "$ref": "#/components/schemas/base_isolated_margin_response"
                          },
                          "leverage": {
                            "$ref": "#/components/schemas/leverage_response"
                          },
                          "adl_rank": {
                            "$ref": "#/components/schemas/adl_rank_response"
                          },
                          "closePositionTriggers": {
                            "type": "object",
                            "properties": {
                              "take_profit": {
                                "type": "object",
                                "properties": {
                                  "order_id": {
                                    "$ref": "#/components/schemas/order_id_response"
                                  },
                                  "trigger": {
                                    "$ref": "#/components/schemas/trigger_response"
                                  },
                                  "order_type": {
                                    "$ref": "#/components/schemas/order_type_response"
                                  }
                                },
                                "required": [
                                  "order_id",
                                  "trigger",
                                  "order_type"
                                ]
                              },
                              "stop_loss": {
                                "type": "object",
                                "properties": {
                                  "order_id": {
                                    "$ref": "#/components/schemas/order_id_response"
                                  },
                                  "trigger": {
                                    "$ref": "#/components/schemas/trigger_response"
                                  },
                                  "order_type": {
                                    "$ref": "#/components/schemas/order_type_response"
                                  }
                                },
                                "required": [
                                  "order_id",
                                  "trigger",
                                  "order_type"
                                ]
                              }
                            }
                          },
                          "partialPositionTriggers": {
                            "type": "object",
                            "properties": {
                              "take_profit": {
                                "type": "array",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "order_id": {
                                      "$ref": "#/components/schemas/order_id_response"
                                    },
                                    "trigger": {
                                      "$ref": "#/components/schemas/trigger_response"
                                    },
                                    "amount": {
                                      "$ref": "#/components/schemas/amount_response"
                                    },
                                    "limit_price": {
                                      "$ref": "#/components/schemas/limit_price_response"
                                    },
                                    "order_type": {
                                      "$ref": "#/components/schemas/order_type_response"
                                    },
                                    "stop": {
                                      "$ref": "#/components/schemas/stop_response"
                                    }
                                  },
                                  "required": [
                                    "order_id",
                                    "trigger",
                                    "amount",
                                    "limit_price",
                                    "order_type",
                                    "stop"
                                  ]
                                }
                              },
                              "stop_loss": {
                                "type": "array",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "order_id": {
                                      "$ref": "#/components/schemas/order_id_response"
                                    },
                                    "trigger": {
                                      "$ref": "#/components/schemas/trigger_response"
                                    },
                                    "amount": {
                                      "$ref": "#/components/schemas/amount_response"
                                    },
                                    "limit_price": {
                                      "$ref": "#/components/schemas/limit_price_response"
                                    },
                                    "order_type": {
                                      "$ref": "#/components/schemas/order_type_response"
                                    },
                                    "stop": {
                                      "$ref": "#/components/schemas/stop_response"
                                    }
                                  },
                                  "required": [
                                    "order_id",
                                    "trigger",
                                    "amount",
                                    "limit_price",
                                    "order_type",
                                    "stop"
                                  ]
                                }
                              }
                            }
                          }
                        },
                        "required": [
                          "instrument_id",
                          "instrument_name",
                          "instrument_type",
                          "asset",
                          "amount",
                          "side",
                          "mark_price",
                          "avg_entry_price",
                          "unrealized_pnl",
                          "initial_margin",
                          "maintenance_margin",
                          "adl_rank"
                        ]
                      }
                    },
                    "account_type": {
                      "$ref": "#/components/schemas/account_type_response"
                    },
                    "fee_structures": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "instrument_type": {
                            "$ref": "#/components/schemas/instrument_type_response"
                          },
                          "taker_fee": {
                            "$ref": "#/components/schemas/taker_fee_response"
                          },
                          "maker_fee": {
                            "$ref": "#/components/schemas/maker_fee_response"
                          }
                        },
                        "required": [
                          "asset",
                          "instrument_type",
                          "taker_fee",
                          "maker_fee"
                        ]
                      }
                    },
                    "campaign_sign_ups": {
                      "$ref": "#/components/schemas/campaign_sign_ups_response"
                    },
                    "portfolio": {
                      "$ref": "#/components/schemas/portfolio_response"
                    },
                    "in_liquidation": {
                      "$ref": "#/components/schemas/in_liquidation_response"
                    },
                    "initial_margin": {
                      "$ref": "#/components/schemas/initial_margin_response"
                    },
                    "maintenance_margin": {
                      "$ref": "#/components/schemas/maintenance_margin_response"
                    },
                    "email_address": {
                      "$ref": "#/components/schemas/email_address_response"
                    },
                    "intercom_hash": {
                      "$ref": "#/components/schemas/intercom_hash_response"
                    },
                    "credit": {
                      "$ref": "#/components/schemas/credit_response"
                    },
                    "credited": {
                      "$ref": "#/components/schemas/credited_response"
                    },
                    "has_been_referred": {
                      "$ref": "#/components/schemas/has_been_referred_response"
                    },
                    "referrer": {
                      "$ref": "#/components/schemas/referrer_response"
                    },
                    "leverages": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "instrument_id": {
                            "$ref": "#/components/schemas/instrument_id_response"
                          },
                          "leverage": {
                            "$ref": "#/components/schemas/leverage_response"
                          },
                          "margin_type": {
                            "$ref": "#/components/schemas/margin_type_response"
                          }
                        },
                        "required": [
                          "instrument_id",
                          "leverage",
                          "margin_type"
                        ]
                      }
                    },
                    "in_adl": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "instrument_name": {
                            "$ref": "#/components/schemas/instrument_name_response"
                          },
                          "instrument_id": {
                            "$ref": "#/components/schemas/instrument_id_response"
                          }
                        },
                        "required": [
                          "instrument_name",
                          "instrument_id"
                        ]
                      }
                    },
                    "manual_mode": {
                      "$ref": "#/components/schemas/manual_mode_response"
                    },
                    "manual_withdrawals": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "chain_id": {
                            "$ref": "#/components/schemas/chain_id_response"
                          },
                          "collateral": {
                            "$ref": "#/components/schemas/collateral_response"
                          },
                          "withdrawal_id": {
                            "$ref": "#/components/schemas/withdrawal_id_response"
                          },
                          "to": {
                            "$ref": "#/components/schemas/to_response"
                          },
                          "label": {
                            "$ref": "#/components/schemas/label_response"
                          }
                        },
                        "required": [
                          "account",
                          "amount",
                          "chain_id",
                          "collateral",
                          "withdrawal_id",
                          "to",
                          "label"
                        ]
                      }
                    },
                    "network_addresses": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "network": {
                            "$ref": "#/components/schemas/network_response"
                          },
                          "network_address": {
                            "$ref": "#/components/schemas/network_address_response"
                          }
                        },
                        "required": [
                          "network",
                          "network_address"
                        ]
                      }
                    }
                  },
                  "required": [
                    "account",
                    "username",
                    "equity",
                    "available_balance",
                    "available_margin",
                    "balance",
                    "account_type",
                    "campaign_sign_ups",
                    "portfolio",
                    "in_liquidation",
                    "initial_margin",
                    "maintenance_margin",
                    "email_address",
                    "intercom_hash",
                    "credit",
                    "credited",
                    "has_been_referred",
                    "referrer",
                    "manual_mode"
                  ]
                }
              }
            }
          },
          "401": {
            "description": "Unauthorized.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GenericErrorResponse",
                  "type": "object",
                  "properties": {
                    "error": {
                      "$ref": "#/components/schemas/error_401_response"
                    }
                  },
                  "required": [
                    "error"
                  ]
                }
              }
            }
          },
          "429": {
            "description": "Rate limit exceeded.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GenericErrorResponse",
                  "type": "object",
                  "properties": {
                    "error": {
                      "$ref": "#/components/schemas/error_429_response"
                    }
                  },
                  "required": [
                    "error"
                  ]
                }
              }
            }
          },
          "500": {
            "description": "Internal server error.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GenericErrorResponse",
                  "type": "object",
                  "properties": {
                    "error": {
                      "$ref": "#/components/schemas/error_500_response"
                    }
                  },
                  "required": [
                    "error"
                  ]
                }
              }
            }
          }
        }
      }
    }
  },
  "servers": [
    {
      "description": "Aevo REST API",
      "url": "https://api.aevo.xyz"
    }
  ],
  "tags": [
    {
      "description": "Private APIs require authentication",
      "name": "Private API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```