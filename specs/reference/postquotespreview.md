> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /quotes/preview

Simulate a new quote.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "account": {
        "title": "account",
        "type": "string",
        "description": "Account's Ethereum address.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "amount": {
        "title": "amount",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number.",
        "example": "1000000"
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
      "block_id": {
        "title": "block_id",
        "type": "string",
        "description": "Block ID is the unique identifier of the block",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "block_id_response": {
        "title": "block_id_response",
        "type": "string",
        "description": "Block ID is the unique identifier of the block Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
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
      "index_price_response": {
        "title": "index_price_response",
        "type": "string",
        "description": "Current index price of the asset. Eg. `12.23`",
        "example": "12.23"
      },
      "initial_margin_response": {
        "title": "initial_margin_response",
        "type": "string",
        "description": "Margin required to keep an open order. Eg. `12.23`",
        "example": "12.23"
      },
      "instrument": {
        "title": "instrument",
        "type": "integer",
        "description": "Instrument ID number.",
        "example": 12
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
      "iv_response": {
        "title": "iv_response",
        "type": "string",
        "description": "Option's implied volatility. Eg. `0.23`",
        "example": "0.23"
      },
      "limit_price": {
        "title": "limit_price",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number.",
        "example": 1000000
      },
      "limit_price_response": {
        "title": "limit_price_response",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
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
      "price": {
        "title": "price",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number.",
        "example": "2000000"
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "quote_id_response": {
        "title": "quote_id_response",
        "type": "string",
        "description": "Quote ID is the hash of the quote payload Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "quote_status_response": {
        "title": "quote_status_response",
        "type": "string",
        "description": "Quote status. Eg. `filled`",
        "enum": [
          "filled",
          "partial",
          "opened",
          "cancelled",
          "rejected"
        ]
      },
      "ratio_response": {
        "title": "ratio_response",
        "type": "string",
        "description": "Ratio of the instrument leg in decimals. Eg. `1`",
        "example": "1"
      },
      "salt": {
        "title": "salt",
        "type": "string",
        "description": "A randomly generated number to guarantee transaction uniqueness. In 6 decimals fixed number.",
        "example": "12345678"
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
      "signature": {
        "title": "signature",
        "type": "string",
        "description": "Hash of order payload signature signed by the account.",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "strike_response": {
        "title": "strike_response",
        "type": "string",
        "description": "Option strike price. Eg. `2500`",
        "example": "2500"
      },
      "system_type": {
        "title": "system_type",
        "type": "string",
        "description": "Client that placed the order (API is set by default)",
        "enum": [
          "API",
          "WEB",
          "MOBILE_WEB",
          "MOBILE_APP"
        ]
      },
      "timestamp": {
        "title": "timestamp",
        "type": "string",
        "description": "Timestamp used in order signing in UNIX timestamp in seconds.",
        "example": "1680249600"
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
    "/quotes/preview": {
      "post": {
        "summary": "POST /quotes/preview",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostQuotesPreview",
        "description": "Simulate a new quote.",
        "responses": {
          "200": {
            "description": "Quote created.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostQuotesPreview200Response",
                  "type": "object",
                  "properties": {
                    "block_id": {
                      "$ref": "#/components/schemas/block_id_response"
                    },
                    "quote_id": {
                      "$ref": "#/components/schemas/quote_id_response"
                    },
                    "amount": {
                      "$ref": "#/components/schemas/amount_response"
                    },
                    "initial_margin": {
                      "$ref": "#/components/schemas/initial_margin_response"
                    },
                    "filled": {
                      "$ref": "#/components/schemas/filled_response"
                    },
                    "limit_price": {
                      "$ref": "#/components/schemas/limit_price_response"
                    },
                    "quote_status": {
                      "$ref": "#/components/schemas/quote_status_response"
                    },
                    "legs": {
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
                          "side": {
                            "$ref": "#/components/schemas/side_response"
                          },
                          "ratio": {
                            "$ref": "#/components/schemas/ratio_response"
                          },
                          "price": {
                            "$ref": "#/components/schemas/price_response"
                          },
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "index_price": {
                            "$ref": "#/components/schemas/index_price_response"
                          },
                          "mark_price": {
                            "$ref": "#/components/schemas/mark_price_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "strike": {
                            "$ref": "#/components/schemas/strike_response"
                          },
                          "option_type": {
                            "$ref": "#/components/schemas/option_type_response"
                          },
                          "iv": {
                            "$ref": "#/components/schemas/iv_response"
                          }
                        },
                        "required": [
                          "instrument_id",
                          "instrument_name",
                          "instrument_type",
                          "side",
                          "ratio",
                          "price",
                          "asset",
                          "index_price",
                          "mark_price"
                        ]
                      }
                    },
                    "avg_price": {
                      "$ref": "#/components/schemas/avg_price_response"
                    },
                    "created_timestamp": {
                      "$ref": "#/components/schemas/created_timestamp_response"
                    }
                  },
                  "required": [
                    "block_id",
                    "quote_id",
                    "amount",
                    "initial_margin",
                    "filled",
                    "limit_price",
                    "quote_status",
                    "created_timestamp"
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
                "title": "PostQuotesPreviewPayload",
                "type": "object",
                "properties": {
                  "block_id": {
                    "$ref": "#/components/schemas/block_id"
                  },
                  "account": {
                    "$ref": "#/components/schemas/account"
                  },
                  "amount": {
                    "$ref": "#/components/schemas/amount"
                  },
                  "is_buy": {
                    "$ref": "#/components/schemas/is_buy"
                  },
                  "salt": {
                    "$ref": "#/components/schemas/salt"
                  },
                  "timestamp": {
                    "$ref": "#/components/schemas/timestamp"
                  },
                  "signature": {
                    "$ref": "#/components/schemas/signature"
                  },
                  "legs": {
                    "type": "array",
                    "items": {
                      "type": "object",
                      "properties": {
                        "instrument": {
                          "$ref": "#/components/schemas/instrument"
                        },
                        "price": {
                          "$ref": "#/components/schemas/price"
                        }
                      },
                      "required": [
                        "instrument",
                        "price"
                      ]
                    }
                  },
                  "limit_price": {
                    "$ref": "#/components/schemas/limit_price"
                  },
                  "system_type": {
                    "$ref": "#/components/schemas/system_type"
                  }
                },
                "required": [
                  "block_id",
                  "account",
                  "amount",
                  "is_buy",
                  "salt",
                  "timestamp",
                  "signature"
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