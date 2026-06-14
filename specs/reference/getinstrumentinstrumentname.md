> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /instrument/{instrument_name}

Returns the instrument information for the given instrument.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "InstrumentNamePath": {
        "name": "instrument_name",
        "in": "path",
        "description": "Instrument name.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "ETH-30JUN23-1600-C"
        }
      }
    },
    "schemas": {
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
      "daily_volume_contracts_response": {
        "title": "daily_volume_contracts_response",
        "type": "string",
        "description": "24-hour traded volume in contract terms. Eg. `1234.5`",
        "example": "1234.5"
      },
      "daily_volume_response": {
        "title": "daily_volume_response",
        "type": "string",
        "description": "24-hour traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
      "initial_margin_fraction_response": {
        "title": "initial_margin_fraction_response",
        "type": "string",
        "description": "Initial margin fraction/rate for the instrument (e.g., 0.1 means 10%). Eg. `0.1`",
        "example": "0.1"
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
      "maintenance_margin_fraction_response": {
        "title": "maintenance_margin_fraction_response",
        "type": "string",
        "description": "Maintenance margin fraction/rate for the instrument (e.g., 0.05 means 5%). Eg. `0.05`",
        "example": "0.05"
      },
      "mark_price_response": {
        "title": "mark_price_response",
        "type": "string",
        "description": "Mark price of the instrument. Eg. `12.23`",
        "example": "12.23"
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "rho_response": {
        "title": "rho_response",
        "type": "string",
        "description": "Option's Rho. Eg. `0.23`",
        "example": "0.23"
      },
      "theta_response": {
        "title": "theta_response",
        "type": "string",
        "description": "Option's Theta. Eg. `0.23`",
        "example": "0.23"
      },
      "total_oi_response": {
        "title": "total_oi_response",
        "type": "string",
        "description": "Total open interest in number of contracts. Eg. `1234.56`",
        "example": "1234.56"
      },
      "total_volume_contracts_response": {
        "title": "total_volume_contracts_response",
        "type": "string",
        "description": "Total traded volume in contract terms. Eg. `1234.5`",
        "example": "1234.5"
      },
      "total_volume_response": {
        "title": "total_volume_response",
        "type": "string",
        "description": "Total traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
    "/instrument/{instrument_name}": {
      "get": {
        "summary": "GET /instrument/{instrument_name}",
        "tags": [
          "Public API"
        ],
        "operationId": "GetInstrumentInstrumentName",
        "description": "Returns the instrument information for the given instrument.",
        "responses": {
          "200": {
            "description": "Instrument information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetInstrumentInstrumentName200Response",
                  "type": "object",
                  "properties": {
                    "asset": {
                      "$ref": "#/components/schemas/asset_response"
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
                    "mark_price": {
                      "$ref": "#/components/schemas/mark_price_response"
                    },
                    "index_price": {
                      "$ref": "#/components/schemas/index_price_response"
                    },
                    "forward_price": {
                      "$ref": "#/components/schemas/forward_price_response"
                    },
                    "initial_margin_fraction": {
                      "$ref": "#/components/schemas/initial_margin_fraction_response"
                    },
                    "maintenance_margin_fraction": {
                      "$ref": "#/components/schemas/maintenance_margin_fraction_response"
                    },
                    "best_bid": {
                      "type": "object",
                      "properties": {
                        "price": {
                          "$ref": "#/components/schemas/price_response"
                        },
                        "amount": {
                          "$ref": "#/components/schemas/amount_response"
                        },
                        "iv": {
                          "$ref": "#/components/schemas/iv_response"
                        }
                      },
                      "required": [
                        "price",
                        "amount",
                        "iv"
                      ]
                    },
                    "best_ask": {
                      "type": "object",
                      "properties": {
                        "price": {
                          "$ref": "#/components/schemas/price_response"
                        },
                        "amount": {
                          "$ref": "#/components/schemas/amount_response"
                        },
                        "iv": {
                          "$ref": "#/components/schemas/iv_response"
                        }
                      },
                      "required": [
                        "price",
                        "amount",
                        "iv"
                      ]
                    },
                    "markets": {
                      "type": "object",
                      "properties": {
                        "daily_volume": {
                          "$ref": "#/components/schemas/daily_volume_response"
                        },
                        "daily_volume_contracts": {
                          "$ref": "#/components/schemas/daily_volume_contracts_response"
                        },
                        "total_volume": {
                          "$ref": "#/components/schemas/total_volume_response"
                        },
                        "total_volume_contracts": {
                          "$ref": "#/components/schemas/total_volume_contracts_response"
                        },
                        "total_oi": {
                          "$ref": "#/components/schemas/total_oi_response"
                        }
                      },
                      "required": [
                        "daily_volume",
                        "daily_volume_contracts",
                        "total_volume",
                        "total_volume_contracts",
                        "total_oi"
                      ]
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
                    }
                  },
                  "required": [
                    "asset",
                    "instrument_id",
                    "instrument_name",
                    "instrument_type",
                    "mark_price",
                    "index_price",
                    "forward_price",
                    "initial_margin_fraction",
                    "maintenance_margin_fraction"
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
            "$ref": "#/components/parameters/InstrumentNamePath"
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