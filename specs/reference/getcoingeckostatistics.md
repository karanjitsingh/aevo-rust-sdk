> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /coingecko-statistics

Returns the perpetual statistics of all assets specifically for https://www.coingecko.com/en/exchanges/aevo

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "base_currency_response": {
        "title": "base_currency_response",
        "type": "string",
        "description": "Symbol/currency code of base pair Eg. `BTC`",
        "example": "BTC"
      },
      "contract_price_currency_response": {
        "title": "contract_price_currency_response",
        "type": "string",
        "description": "Describes the currency which the contract is priced in. Eg. `USD`",
        "example": "USD"
      },
      "contract_type_response": {
        "title": "contract_type_response",
        "type": "string",
        "description": "Type of contract - Vanilla, Inverse or Quanto Eg. `Vanilla`",
        "example": "Vanilla"
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
      "funding_rate_response": {
        "title": "funding_rate_response",
        "type": "string",
        "description": "Funding rate in decimals. Eg. `0.00122`",
        "example": "0.00122"
      },
      "index_currency_response": {
        "title": "index_currency_response",
        "type": "string",
        "description": "Underlying currency for index Eg. `USD`",
        "example": "USD"
      },
      "index_price_response": {
        "title": "index_price_response",
        "type": "string",
        "description": "Current index price of the asset. Eg. `12.23`",
        "example": "12.23"
      },
      "next_funding_rate_timestamp_response": {
        "title": "next_funding_rate_timestamp_response",
        "type": "string",
        "description": "Timestamp of the next funding rate change Eg. `USD`",
        "example": "USD"
      },
      "open_interest_response": {
        "title": "open_interest_response",
        "type": "string",
        "description": "Open interest in USDC terms for a given expiry. Eg. `1234.56`",
        "example": "1234.56"
      },
      "product_volume_response": {
        "title": "product_volume_response",
        "type": "string",
        "description": "Type of product: Futures, Perpetual, Options Eg. `Perpetual`",
        "example": "Perpetual"
      },
      "target_currency_response": {
        "title": "target_currency_response",
        "type": "string",
        "description": "Symbol/currency code of target pair Eg. `ETH`",
        "example": "ETH"
      },
      "target_volume_response": {
        "title": "target_volume_response",
        "type": "string",
        "description": "24 hour trading volume in target pair volume Eg. `0`",
        "example": "0"
      },
      "ticker_id_response": {
        "title": "ticker_id_response",
        "type": "string",
        "description": "Identifier of a ticker with delimiter to separate base/target, eg. BTC-PERP Eg. `ETH-PERP`",
        "example": "ETH-PERP"
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
    "/coingecko-statistics": {
      "get": {
        "summary": "GET /coingecko-statistics",
        "tags": [
          "Public API"
        ],
        "operationId": "GetCoingeckoStatistics",
        "description": "Returns the perpetual statistics of all assets specifically for https://www.coingecko.com/en/exchanges/aevo",
        "responses": {
          "200": {
            "description": "Instrument trade information.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "GetCoingeckoStatistics200Response",
                    "type": "object",
                    "properties": {
                      "ticker_id": {
                        "$ref": "#/components/schemas/ticker_id_response"
                      },
                      "base_currency": {
                        "$ref": "#/components/schemas/base_currency_response"
                      },
                      "target_currency": {
                        "$ref": "#/components/schemas/target_currency_response"
                      },
                      "target_volume": {
                        "$ref": "#/components/schemas/target_volume_response"
                      },
                      "product_volume": {
                        "$ref": "#/components/schemas/product_volume_response"
                      },
                      "open_interest": {
                        "$ref": "#/components/schemas/open_interest_response"
                      },
                      "index_price": {
                        "$ref": "#/components/schemas/index_price_response"
                      },
                      "index_currency": {
                        "$ref": "#/components/schemas/index_currency_response"
                      },
                      "next_funding_rate_timestamp": {
                        "$ref": "#/components/schemas/next_funding_rate_timestamp_response"
                      },
                      "funding_rate": {
                        "$ref": "#/components/schemas/funding_rate_response"
                      },
                      "contract_type": {
                        "$ref": "#/components/schemas/contract_type_response"
                      },
                      "contract_price_currency": {
                        "$ref": "#/components/schemas/contract_price_currency_response"
                      }
                    },
                    "required": [
                      "ticker_id"
                    ]
                  }
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```