> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /positions

Returns the account's positions

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
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "avg_entry_price_response": {
        "title": "avg_entry_price_response",
        "type": "string",
        "description": "Average entry price. Eg. `12.23`",
        "example": "12.23"
      },
      "base_isolated_margin_response": {
        "title": "base_isolated_margin_response",
        "type": "string",
        "description": "Base isolated margin allocated from fills only (excludes manual top-ups). Eg. `10000000`",
        "example": "10000000"
      },
      "delta_response": {
        "title": "delta_response",
        "type": "string",
        "description": "Option's Delta. Eg. `0.23`",
        "example": "0.23"
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
      "margin_type_response": {
        "title": "margin_type_response",
        "type": "string",
        "description": "The margin type. Eg. `CROSS`",
        "enum": [
          "CROSS",
          "ISOLATED"
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
      "theta_response": {
        "title": "theta_response",
        "type": "string",
        "description": "Option's Theta. Eg. `0.23`",
        "example": "0.23"
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
      "vega_response": {
        "title": "vega_response",
        "type": "string",
        "description": "Option's Vega. Eg. `0.23`",
        "example": "0.23"
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
    "/positions": {
      "get": {
        "summary": "GET /positions",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetPositions",
        "description": "Returns the account's positions",
        "responses": {
          "200": {
            "description": "Account position information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetPositions200Response",
                  "type": "object",
                  "properties": {
                    "account": {
                      "$ref": "#/components/schemas/account_response"
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
                    }
                  },
                  "required": [
                    "account"
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