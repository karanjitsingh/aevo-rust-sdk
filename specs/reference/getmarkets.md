> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /markets

Returns a list of instruments. If `asset` is not specified, the response will include all listed instruments.

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
      "amount_step_response": {
        "title": "amount_step_response",
        "type": "string",
        "description": "Allowed increments in contract amount. Eg. `0.1`",
        "example": "0.1"
      },
      "delta_response": {
        "title": "delta_response",
        "type": "string",
        "description": "Option's Delta. Eg. `0.23`",
        "example": "0.23"
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
      "forward_price_response": {
        "title": "forward_price_response",
        "type": "string",
        "description": "Current forward price of the asset. Eg. `12.23`",
        "example": "12.23"
      },
      "gamma_response": {
        "title": "gamma_response",
        "type": "string",
        "description": "Option's Gamma. Eg. `0.23`",
        "example": "0.23"
      },
      "index_price_response": {
        "title": "index_price_response",
        "type": "string",
        "description": "Current index price of the asset. Eg. `12.23`",
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
      "is_active_response": {
        "title": "is_active_response",
        "type": "boolean",
        "description": "True if instrument is active and tradable. Eg. `true`",
        "example": true
      },
      "iv_response": {
        "title": "iv_response",
        "type": "string",
        "description": "Option's implied volatility. Eg. `0.23`",
        "example": "0.23"
      },
      "mark_price_response": {
        "title": "mark_price_response",
        "type": "string",
        "description": "Mark price of the instrument. Eg. `12.23`",
        "example": "12.23"
      },
      "max_leverage_response": {
        "title": "max_leverage_response",
        "type": "string",
        "description": "The maximum leverage multiplier for an instrument. Eg. `12`",
        "example": "12"
      },
      "max_notional_value_response": {
        "title": "max_notional_value_response",
        "type": "string",
        "description": "Maximum allowed notional value Eg. `0.1`",
        "example": "0.1"
      },
      "max_order_value_response": {
        "title": "max_order_value_response",
        "type": "string",
        "description": "Maximum allowed order value Eg. `0.1`",
        "example": "0.1"
      },
      "min_order_value_response": {
        "title": "min_order_value_response",
        "type": "string",
        "description": "Minimum allowed order value Eg. `0.1`",
        "example": "0.1"
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
      "price_step_response": {
        "title": "price_step_response",
        "type": "string",
        "description": "Allowed increments in price. Eg. `0.05`",
        "example": "0.05"
      },
      "quote_asset_response": {
        "title": "quote_asset_response",
        "type": "string",
        "description": "Quote asset symbol. Eg. `USDC`",
        "example": "USDC"
      },
      "rho_response": {
        "title": "rho_response",
        "type": "string",
        "description": "Option's Rho. Eg. `0.23`",
        "example": "0.23"
      },
      "strike_response": {
        "title": "strike_response",
        "type": "string",
        "description": "Option strike price. Eg. `2500`",
        "example": "2500"
      },
      "theta_response": {
        "title": "theta_response",
        "type": "string",
        "description": "Option's Theta. Eg. `0.23`",
        "example": "0.23"
      },
      "underlying_asset_response": {
        "title": "underlying_asset_response",
        "type": "string",
        "description": "Underlying asset symbol. Eg. `ETH`",
        "example": "ETH"
      },
      "vega_response": {
        "title": "vega_response",
        "type": "string",
        "description": "Option's Vega. Eg. `0.23`",
        "example": "0.23"
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
    "/markets": {
      "get": {
        "summary": "GET /markets",
        "tags": [
          "Public API"
        ],
        "operationId": "GetMarkets",
        "description": "Returns a list of instruments. If `asset` is not specified, the response will include all listed instruments.",
        "responses": {
          "200": {
            "description": "List of instruments.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "GetMarkets200Response",
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
                      "underlying_asset": {
                        "$ref": "#/components/schemas/underlying_asset_response"
                      },
                      "quote_asset": {
                        "$ref": "#/components/schemas/quote_asset_response"
                      },
                      "price_step": {
                        "$ref": "#/components/schemas/price_step_response"
                      },
                      "amount_step": {
                        "$ref": "#/components/schemas/amount_step_response"
                      },
                      "min_order_value": {
                        "$ref": "#/components/schemas/min_order_value_response"
                      },
                      "max_order_value": {
                        "$ref": "#/components/schemas/max_order_value_response"
                      },
                      "max_notional_value": {
                        "$ref": "#/components/schemas/max_notional_value_response"
                      },
                      "mark_price": {
                        "$ref": "#/components/schemas/mark_price_response"
                      },
                      "forward_price": {
                        "$ref": "#/components/schemas/forward_price_response"
                      },
                      "index_price": {
                        "$ref": "#/components/schemas/index_price_response"
                      },
                      "is_active": {
                        "$ref": "#/components/schemas/is_active_response"
                      },
                      "option_type": {
                        "$ref": "#/components/schemas/option_type_response"
                      },
                      "expiry": {
                        "$ref": "#/components/schemas/expiry_response"
                      },
                      "strike": {
                        "$ref": "#/components/schemas/strike_response"
                      },
                      "greeks": {
                        "type": "object",
                        "properties": {
                          "delta": {
                            "$ref": "#/components/schemas/delta_response"
                          },
                          "gamma": {
                            "$ref": "#/components/schemas/gamma_response"
                          },
                          "rho": {
                            "$ref": "#/components/schemas/rho_response"
                          },
                          "theta": {
                            "$ref": "#/components/schemas/theta_response"
                          },
                          "vega": {
                            "$ref": "#/components/schemas/vega_response"
                          },
                          "iv": {
                            "$ref": "#/components/schemas/iv_response"
                          }
                        },
                        "required": [
                          "delta",
                          "gamma",
                          "rho",
                          "theta",
                          "vega",
                          "iv"
                        ]
                      },
                      "max_leverage": {
                        "$ref": "#/components/schemas/max_leverage_response"
                      }
                    },
                    "required": [
                      "instrument_id",
                      "instrument_name",
                      "instrument_type",
                      "underlying_asset",
                      "quote_asset",
                      "price_step",
                      "amount_step",
                      "min_order_value",
                      "max_order_value",
                      "max_notional_value",
                      "mark_price",
                      "index_price",
                      "is_active"
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
        },
        "parameters": [
          {
            "$ref": "#/components/parameters/AssetQueryOptional"
          },
          {
            "$ref": "#/components/parameters/InstrumentTypeQueryOptional"
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