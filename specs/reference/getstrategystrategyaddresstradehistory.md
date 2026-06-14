> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /strategy/{strategy_address}/trade-history

Return the account's trade history.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AssetQueryOptional": {
        "name": "asset",
        "in": "query",
        "description": "Name of underlying asset.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "ETH"
        }
      },
      "EndTimeQueryOptional": {
        "name": "end_time",
        "in": "query",
        "description": "Entries created after (>) end time are excluded in UNIX timestamp in nanoseconds. Defaults to current time.",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 1675036800000000000
        }
      },
      "InstrumentTypeQueryOptional": {
        "name": "instrument_type",
        "in": "query",
        "description": "Type of instrument.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "OPTION",
          "enum": [
            "OPTION",
            "PERPETUAL",
            "SPOT"
          ]
        }
      },
      "LimitQueryOptional": {
        "name": "limit",
        "in": "query",
        "description": "Limits the number of relevant entries in the response. Defaults to `10`. Maximum value is `50`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 10
        }
      },
      "OffsetQueryOptional": {
        "name": "offset",
        "in": "query",
        "description": "Offset.",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 10
        }
      },
      "OptionTypeQueryOptional": {
        "name": "option_type",
        "in": "query",
        "description": "Type of option contract.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "call",
          "enum": [
            "put",
            "call"
          ]
        }
      },
      "StartTimeQuery": {
        "name": "start_time",
        "in": "query",
        "description": "Entries created prior (<) to start time are excluded in UNIX timestamp in nanoseconds. Defaults to `0`",
        "required": true,
        "schema": {
          "type": "integer",
          "example": 1672531200000000000
        }
      },
      "StrategyAddressPath": {
        "name": "strategy_address",
        "in": "path",
        "description": "Address of strategy",
        "required": true,
        "schema": {
          "type": "string",
          "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
        }
      },
      "TradeTypesQueryOptional": {
        "name": "trade_types",
        "in": "query",
        "description": "Type of user trade.",
        "required": false,
        "schema": {
          "type": "array",
          "example": [
            "trade",
            "liquidation",
            "settlement",
            "funding"
          ],
          "items": {
            "type": "string",
            "enum": [
              "trade",
              "liquidation",
              "settlement",
              "funding"
            ]
          }
        }
      }
    },
    "schemas": {
      "account_response": {
        "title": "account_response",
        "type": "string",
        "description": "Account's Ethereum address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "agg_order_id_response": {
        "title": "agg_order_id_response",
        "type": "string",
        "description": "Aggressor Order ID. Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "avg_price_response": {
        "title": "avg_price_response",
        "type": "string",
        "description": "Average entry price of the position. Eg. `12.23`",
        "example": "12.23"
      },
      "count_response": {
        "title": "count_response",
        "type": "string",
        "description": "total number of rows in a query Eg. `5`",
        "example": "5"
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
      "fee_rate_response": {
        "title": "fee_rate_response",
        "type": "string",
        "description": "Funding fee rate in decimals. Only applies to funding trade type. Eg. `0.000065`",
        "example": "0.000065"
      },
      "fees_response": {
        "title": "fees_response",
        "type": "string",
        "description": "Fees paid for the trade. Eg. `12.23`",
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
      "iv_response": {
        "title": "iv_response",
        "type": "string",
        "description": "Option's implied volatility. Eg. `0.23`",
        "example": "0.23"
      },
      "liquidation_fee_response": {
        "title": "liquidation_fee_response",
        "type": "string",
        "description": "Liquidation fee. Eg. `86.23`",
        "example": "86.23"
      },
      "liquidity_response": {
        "title": "liquidity_response",
        "type": "string",
        "description": "Taker or maker order type Eg. `maker`",
        "enum": [
          "maker",
          "taker"
        ]
      },
      "mark_price_response": {
        "title": "mark_price_response",
        "type": "string",
        "description": "Mark price of the instrument. Eg. `12.23`",
        "example": "12.23"
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
      "payout_response": {
        "title": "payout_response",
        "type": "string",
        "description": "Settlement payout. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "pnl_response": {
        "title": "pnl_response",
        "type": "string",
        "description": "Profit and loss. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "settlement_price_response": {
        "title": "settlement_price_response",
        "type": "string",
        "description": "Settlement price at expiry. Eg. `1734.23`",
        "example": "1734.23"
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
      "spot_price_response": {
        "title": "spot_price_response",
        "type": "string",
        "description": "Spot price. Eg. `1802.30`",
        "example": "1802.30"
      },
      "strike_response": {
        "title": "strike_response",
        "type": "string",
        "description": "Option strike price. Eg. `2500`",
        "example": "2500"
      },
      "trade_id_response": {
        "title": "trade_id_response",
        "type": "string",
        "description": "Unique ID of the trade. Eg. `DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2`",
        "example": "DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2"
      },
      "trade_status_response": {
        "title": "trade_status_response",
        "type": "string",
        "description": "Trade status. Eg. `filled`",
        "enum": [
          "filled",
          "partial"
        ]
      },
      "trade_type_response": {
        "title": "trade_type_response",
        "type": "string",
        "description": "Type of user trade. Eg. `trade`",
        "enum": [
          "trade",
          "liquidation",
          "settlement",
          "funding"
        ]
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
    "/strategy/{strategy_address}/trade-history": {
      "get": {
        "summary": "GET /strategy/{strategy_address}/trade-history",
        "tags": [
          "Public API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetStrategyStrategyAddressTradeHistory",
        "description": "Return the account's trade history.",
        "responses": {
          "200": {
            "description": "Trade history.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStrategyStrategyAddressTradeHistory200Response",
                  "type": "object",
                  "properties": {
                    "count": {
                      "$ref": "#/components/schemas/count_response"
                    },
                    "trade_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "trade_id": {
                            "$ref": "#/components/schemas/trade_id_response"
                          },
                          "order_id": {
                            "$ref": "#/components/schemas/order_id_response"
                          },
                          "trade_type": {
                            "$ref": "#/components/schemas/trade_type_response"
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
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "spot_price": {
                            "$ref": "#/components/schemas/spot_price_response"
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
                          "mark_price": {
                            "$ref": "#/components/schemas/mark_price_response"
                          },
                          "side": {
                            "$ref": "#/components/schemas/side_response"
                          },
                          "fees": {
                            "$ref": "#/components/schemas/fees_response"
                          },
                          "liquidity": {
                            "$ref": "#/components/schemas/liquidity_response"
                          },
                          "iv": {
                            "$ref": "#/components/schemas/iv_response"
                          },
                          "fee_rate": {
                            "$ref": "#/components/schemas/fee_rate_response"
                          },
                          "pnl": {
                            "$ref": "#/components/schemas/pnl_response"
                          },
                          "payout": {
                            "$ref": "#/components/schemas/payout_response"
                          },
                          "strike": {
                            "$ref": "#/components/schemas/strike_response"
                          },
                          "option_type": {
                            "$ref": "#/components/schemas/option_type_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "order_type": {
                            "$ref": "#/components/schemas/order_type_response"
                          },
                          "agg_order_id": {
                            "$ref": "#/components/schemas/agg_order_id_response"
                          },
                          "trade_status": {
                            "$ref": "#/components/schemas/trade_status_response"
                          },
                          "settlement_price": {
                            "$ref": "#/components/schemas/settlement_price_response"
                          },
                          "liquidation_fee": {
                            "$ref": "#/components/schemas/liquidation_fee_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "trade_id",
                          "trade_type",
                          "account",
                          "instrument_id",
                          "instrument_name",
                          "instrument_type",
                          "asset",
                          "amount",
                          "side",
                          "fees",
                          "created_timestamp"
                        ]
                      }
                    }
                  },
                  "required": [
                    "count"
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
        "parameters": [
          {
            "$ref": "#/components/parameters/StrategyAddressPath"
          },
          {
            "$ref": "#/components/parameters/StartTimeQuery"
          },
          {
            "$ref": "#/components/parameters/AssetQueryOptional"
          },
          {
            "$ref": "#/components/parameters/EndTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/TradeTypesQueryOptional"
          },
          {
            "$ref": "#/components/parameters/InstrumentTypeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/OptionTypeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/LimitQueryOptional"
          },
          {
            "$ref": "#/components/parameters/OffsetQueryOptional"
          }
        ]
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```