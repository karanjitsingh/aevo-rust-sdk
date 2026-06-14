> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /statistics

Returns the market statistics for the given asset.

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
      }
    },
    "schemas": {
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "calls_response": {
        "title": "calls_response",
        "type": "string",
        "description": "Call options open interest in number of contracts. Eg. `1234.56`",
        "example": "1234.56"
      },
      "daily_buy_volume_response": {
        "title": "daily_buy_volume_response",
        "type": "string",
        "description": "24-hour buy notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "daily_sell_volume_response": {
        "title": "daily_sell_volume_response",
        "type": "string",
        "description": "24-hour sell notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "daily_volume_contracts_response": {
        "title": "daily_volume_contracts_response",
        "type": "string",
        "description": "24-hour traded volume in contract terms. Eg. `1234.5`",
        "example": "1234.5"
      },
      "daily_volume_premium_response": {
        "title": "daily_volume_premium_response",
        "type": "string",
        "description": "24-hour traded premium volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "daily_volume_response": {
        "title": "daily_volume_response",
        "type": "string",
        "description": "24-hour traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
      "funding_daily_avg_response": {
        "title": "funding_daily_avg_response",
        "type": "string",
        "description": "24-hour average of the funding rate of the instrument. Eg. `12.52`",
        "example": "12.52"
      },
      "index_daily_change_response": {
        "title": "index_daily_change_response",
        "type": "string",
        "description": "24-hour change in index price of the underlying asset. Eg. `12.52`",
        "example": "12.52"
      },
      "index_price_response": {
        "title": "index_price_response",
        "type": "string",
        "description": "Current index price of the asset. Eg. `12.23`",
        "example": "12.23"
      },
      "mark_daily_change_response": {
        "title": "mark_daily_change_response",
        "type": "string",
        "description": "24-hour change in mark price of the instrument. Eg. `12.52`",
        "example": "12.52"
      },
      "mark_price_24h_ago_response": {
        "title": "mark_price_24h_ago_response",
        "type": "string",
        "description": "Mark price of the instrument from 24-hour ago. Eg. `12.52`",
        "example": "12.52"
      },
      "mark_price_response": {
        "title": "mark_price_response",
        "type": "string",
        "description": "Mark price of the instrument. Eg. `12.23`",
        "example": "12.23"
      },
      "put_call_ratio_response": {
        "title": "put_call_ratio_response",
        "type": "string",
        "description": "Put call ratio. Eg. `0.23`",
        "example": "0.23"
      },
      "puts_response": {
        "title": "puts_response",
        "type": "string",
        "description": "Put options open interest in number of contracts. Eg. `1234.56`",
        "example": "1234.56"
      },
      "total_response": {
        "title": "total_response",
        "type": "string",
        "description": "Total open interest in number of contracts. Eg. `1234.56`",
        "example": "1234.56"
      },
      "total_volume_premium_response": {
        "title": "total_volume_premium_response",
        "type": "string",
        "description": "Total traded premium volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "total_volume_response": {
        "title": "total_volume_response",
        "type": "string",
        "description": "Total traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
    "/statistics": {
      "get": {
        "summary": "GET /statistics",
        "tags": [
          "Public API"
        ],
        "operationId": "GetStatistics",
        "description": "Returns the market statistics for the given asset.",
        "responses": {
          "200": {
            "description": "Instrument trade information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStatistics200Response",
                  "type": "object",
                  "properties": {
                    "asset": {
                      "$ref": "#/components/schemas/asset_response"
                    },
                    "open_interest": {
                      "type": "object",
                      "properties": {
                        "calls": {
                          "$ref": "#/components/schemas/calls_response"
                        },
                        "puts": {
                          "$ref": "#/components/schemas/puts_response"
                        },
                        "total": {
                          "$ref": "#/components/schemas/total_response"
                        }
                      },
                      "required": [
                        "total"
                      ]
                    },
                    "daily_volume": {
                      "$ref": "#/components/schemas/daily_volume_response"
                    },
                    "daily_buy_volume": {
                      "$ref": "#/components/schemas/daily_buy_volume_response"
                    },
                    "daily_sell_volume": {
                      "$ref": "#/components/schemas/daily_sell_volume_response"
                    },
                    "daily_volume_premium": {
                      "$ref": "#/components/schemas/daily_volume_premium_response"
                    },
                    "total_volume": {
                      "$ref": "#/components/schemas/total_volume_response"
                    },
                    "total_volume_premium": {
                      "$ref": "#/components/schemas/total_volume_premium_response"
                    },
                    "daily_volume_contracts": {
                      "$ref": "#/components/schemas/daily_volume_contracts_response"
                    },
                    "index_price": {
                      "$ref": "#/components/schemas/index_price_response"
                    },
                    "index_daily_change": {
                      "$ref": "#/components/schemas/index_daily_change_response"
                    },
                    "mark_price": {
                      "$ref": "#/components/schemas/mark_price_response"
                    },
                    "mark_price_24h_ago": {
                      "$ref": "#/components/schemas/mark_price_24h_ago_response"
                    },
                    "mark_daily_change": {
                      "$ref": "#/components/schemas/mark_daily_change_response"
                    },
                    "funding_daily_avg": {
                      "$ref": "#/components/schemas/funding_daily_avg_response"
                    },
                    "put_call_ratio": {
                      "$ref": "#/components/schemas/put_call_ratio_response"
                    }
                  },
                  "required": [
                    "daily_volume",
                    "daily_buy_volume",
                    "daily_sell_volume"
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
            "$ref": "#/components/parameters/AssetQueryOptional"
          },
          {
            "$ref": "#/components/parameters/InstrumentTypeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/EndTimeQueryOptional"
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