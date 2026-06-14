> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /swap

Swaps collateral.

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
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "avg_price_response": {
        "title": "avg_price_response",
        "type": "string",
        "description": "Average entry price of the position. Eg. `12.23`",
        "example": "12.23"
      },
      "base_amount": {
        "title": "base_amount",
        "type": "string",
        "description": "The collateral amount. In 6 decimals fixed number for USDT, 18 decimals for WETH, and 8 decimals for WBTC.",
        "example": "1000000"
      },
      "close_position_response": {
        "title": "close_position_response",
        "type": "boolean",
        "description": "Is order a close position TPSL order. Eg. `true`",
        "example": true
      },
      "collateral_asset": {
        "title": "collateral_asset",
        "type": "string",
        "description": "Name of the collateral asset.",
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
      "created_timestamp_response": {
        "title": "created_timestamp_response",
        "type": "string",
        "description": "Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "error_400_response": {
        "title": "error_400_response",
        "type": "string",
        "description": "Error message. Eg. `ERR_MALFORMED_REQUEST`",
        "example": "ERR_MALFORMED_REQUEST"
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
      "filled_response": {
        "title": "filled_response",
        "type": "string",
        "description": "Amount filled. Eg. `12.23`",
        "example": "12.23"
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
      "is_buy": {
        "title": "is_buy",
        "type": "boolean",
        "description": "True for long order, false for short order.",
        "example": true
      },
      "is_twap_response": {
        "title": "is_twap_response",
        "type": "boolean",
        "description": "True if this is a TWAP order Eg. `true`",
        "example": true
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
      "order_status_response": {
        "title": "order_status_response",
        "type": "string",
        "description": "Order status. Eg. `filled`",
        "enum": [
          "filled",
          "partial",
          "opened",
          "cancelled",
          "expired",
          "rejected",
          "stop_order"
        ]
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
      "parent_order_id_response": {
        "title": "parent_order_id_response",
        "type": "string",
        "description": "Order ID of the order which when cancelled, cancels this order as well Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "partial_position_response": {
        "title": "partial_position_response",
        "type": "boolean",
        "description": "Is order a partial position TPSL order. Eg. `false`",
        "example": false
      },
      "post_only_response": {
        "title": "post_only_response",
        "type": "boolean",
        "description": "True for post-only maker orders, false for standard orders. Eg. `false`",
        "example": false
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "quote_amount": {
        "title": "quote_amount",
        "type": "string",
        "description": "Amount of USDC. In 6 decimals fixed number.",
        "example": "1000000"
      },
      "reduce_only_response": {
        "title": "reduce_only_response",
        "type": "boolean",
        "description": "True for reduce-only orders, false for standard orders. Eg. `false`",
        "example": false
      },
      "self_trade_prevention_response": {
        "title": "self_trade_prevention_response",
        "type": "string",
        "description": "Can be set to EXPIRE_NONE, EXPIRE_MAKER OR EXPIRE_TAKER (EXPIRE_NONE is set by default) Eg. `EXPIRE_NONE`",
        "enum": [
          "EXPIRE_NONE",
          "EXPIRE_TAKER",
          "EXPIRE_MAKER"
        ]
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
      "system_type_response": {
        "title": "system_type_response",
        "type": "string",
        "description": "Client that placed the order (API is set by default) Eg. `API`",
        "enum": [
          "API",
          "WEB",
          "MOBILE_WEB",
          "MOBILE_APP"
        ]
      },
      "time_in_force_response": {
        "title": "time_in_force_response",
        "type": "string",
        "description": "Can be set to GTC or IOC (GTC is set by default) Eg. `GTC`",
        "enum": [
          "GTC",
          "IOC"
        ]
      },
      "timestamp_response": {
        "title": "timestamp_response",
        "type": "string",
        "description": "Timestamp in UNIX in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "trigger_response": {
        "title": "trigger_response",
        "type": "string",
        "description": "The price to trigger the stop order at. `stop` is required when `trigger` is specified. Eg. `1836.74`",
        "example": "1836.74"
      },
      "twap_avg_price_response": {
        "title": "twap_avg_price_response",
        "type": "string",
        "description": "Volume-weighted average price of executed slices Eg. `3500000000`",
        "example": "3500000000"
      },
      "twap_duration_seconds_response": {
        "title": "twap_duration_seconds_response",
        "type": "string",
        "description": "Total duration of TWAP order in seconds Eg. `3600`",
        "example": "3600"
      },
      "twap_executed_response": {
        "title": "twap_executed_response",
        "type": "string",
        "description": "Amount executed so far for TWAP order (6 decimals) Eg. `5000000`",
        "example": "5000000"
      },
      "twap_interval_seconds_response": {
        "title": "twap_interval_seconds_response",
        "type": "string",
        "description": "Interval between slices in seconds Eg. `36`",
        "example": "36"
      },
      "twap_remaining_response": {
        "title": "twap_remaining_response",
        "type": "string",
        "description": "Remaining amount to execute for TWAP order (6 decimals) Eg. `5000000`",
        "example": "5000000"
      },
      "twap_slices_executed_response": {
        "title": "twap_slices_executed_response",
        "type": "string",
        "description": "Number of slices executed for TWAP order Eg. `5`",
        "example": "5"
      },
      "twap_slices_total_response": {
        "title": "twap_slices_total_response",
        "type": "string",
        "description": "Total number of slices for TWAP order Eg. `10`",
        "example": "10"
      },
      "twap_total_amount_response": {
        "title": "twap_total_amount_response",
        "type": "string",
        "description": "Total amount to execute for TWAP order (6 decimals) Eg. `10000000`",
        "example": "10000000"
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
    "/swap": {
      "post": {
        "summary": "POST /swap",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostSwap",
        "description": "Swaps collateral.",
        "responses": {
          "200": {
            "description": "Order created.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostSwap200Response",
                  "type": "object",
                  "properties": {
                    "order_id": {
                      "$ref": "#/components/schemas/order_id_response"
                    },
                    "account": {
                      "$ref": "#/components/schemas/account_response"
                    },
                    "instrument_id": {
                      "$ref": "#/components/schemas/instrument_id_response"
                    },
                    "instrument_name": {
                      "$ref": "#/components/schemas/instrument_name_response"
                    },
                    "instrument_type": {
                      "$ref": "#/components/schemas/instrument_type_response"
                    },
                    "order_type": {
                      "$ref": "#/components/schemas/order_type_response"
                    },
                    "side": {
                      "$ref": "#/components/schemas/side_response"
                    },
                    "amount": {
                      "$ref": "#/components/schemas/amount_response"
                    },
                    "price": {
                      "$ref": "#/components/schemas/price_response"
                    },
                    "avg_price": {
                      "$ref": "#/components/schemas/avg_price_response"
                    },
                    "filled": {
                      "$ref": "#/components/schemas/filled_response"
                    },
                    "order_status": {
                      "$ref": "#/components/schemas/order_status_response"
                    },
                    "post_only": {
                      "$ref": "#/components/schemas/post_only_response"
                    },
                    "reduce_only": {
                      "$ref": "#/components/schemas/reduce_only_response"
                    },
                    "initial_margin": {
                      "$ref": "#/components/schemas/initial_margin_response"
                    },
                    "option_type": {
                      "$ref": "#/components/schemas/option_type_response"
                    },
                    "iv": {
                      "$ref": "#/components/schemas/iv_response"
                    },
                    "expiry": {
                      "$ref": "#/components/schemas/expiry_response"
                    },
                    "strike": {
                      "$ref": "#/components/schemas/strike_response"
                    },
                    "created_timestamp": {
                      "$ref": "#/components/schemas/created_timestamp_response"
                    },
                    "timestamp": {
                      "$ref": "#/components/schemas/timestamp_response"
                    },
                    "system_type": {
                      "$ref": "#/components/schemas/system_type_response"
                    },
                    "time_in_force": {
                      "$ref": "#/components/schemas/time_in_force_response"
                    },
                    "stop": {
                      "$ref": "#/components/schemas/stop_response"
                    },
                    "trigger": {
                      "$ref": "#/components/schemas/trigger_response"
                    },
                    "close_position": {
                      "$ref": "#/components/schemas/close_position_response"
                    },
                    "partial_position": {
                      "$ref": "#/components/schemas/partial_position_response"
                    },
                    "isolated_margin": {
                      "$ref": "#/components/schemas/isolated_margin_response"
                    },
                    "parent_order_id": {
                      "$ref": "#/components/schemas/parent_order_id_response"
                    },
                    "self_trade_prevention": {
                      "$ref": "#/components/schemas/self_trade_prevention_response"
                    },
                    "is_twap": {
                      "$ref": "#/components/schemas/is_twap_response"
                    },
                    "twap_total_amount": {
                      "$ref": "#/components/schemas/twap_total_amount_response"
                    },
                    "twap_executed": {
                      "$ref": "#/components/schemas/twap_executed_response"
                    },
                    "twap_remaining": {
                      "$ref": "#/components/schemas/twap_remaining_response"
                    },
                    "twap_slices_total": {
                      "$ref": "#/components/schemas/twap_slices_total_response"
                    },
                    "twap_slices_executed": {
                      "$ref": "#/components/schemas/twap_slices_executed_response"
                    },
                    "twap_duration_seconds": {
                      "$ref": "#/components/schemas/twap_duration_seconds_response"
                    },
                    "twap_interval_seconds": {
                      "$ref": "#/components/schemas/twap_interval_seconds_response"
                    },
                    "twap_avg_price": {
                      "$ref": "#/components/schemas/twap_avg_price_response"
                    }
                  },
                  "required": [
                    "order_id",
                    "account",
                    "instrument_id",
                    "instrument_name",
                    "instrument_type",
                    "order_type",
                    "side",
                    "amount",
                    "price",
                    "avg_price",
                    "filled",
                    "order_status",
                    "timestamp",
                    "system_type"
                  ]
                }
              }
            }
          },
          "400": {
            "description": "Bad request.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GenericErrorResponse",
                  "type": "object",
                  "properties": {
                    "error": {
                      "$ref": "#/components/schemas/error_400_response"
                    }
                  },
                  "required": [
                    "error"
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
        },
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "title": "PostSwapPayload",
                "type": "object",
                "properties": {
                  "collateral_asset": {
                    "$ref": "#/components/schemas/collateral_asset"
                  },
                  "is_buy": {
                    "$ref": "#/components/schemas/is_buy"
                  },
                  "base_amount": {
                    "$ref": "#/components/schemas/base_amount"
                  },
                  "quote_amount": {
                    "$ref": "#/components/schemas/quote_amount"
                  }
                },
                "required": [
                  "collateral_asset",
                  "is_buy"
                ]
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