> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /rfqs/{block_id}/quotes

Get the quotes for a given RFQ block.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "BlockIdPath": {
        "name": "block_id",
        "in": "path",
        "description": "Block ID is the unique identifier of the block",
        "required": true,
        "schema": {
          "type": "string",
          "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
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
      "is_buy_response": {
        "title": "is_buy_response",
        "type": "boolean",
        "description": "True for long order, false for short order. Eg. `true`",
        "example": true
      },
      "iv_response": {
        "title": "iv_response",
        "type": "string",
        "description": "Option's implied volatility. Eg. `0.23`",
        "example": "0.23"
      },
      "limit_price_response": {
        "title": "limit_price_response",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
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
      "side_response": {
        "title": "side_response",
        "type": "string",
        "description": "Trade side. Eg. `buy`",
        "enum": [
          "buy",
          "sell"
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
    "/rfqs/{block_id}/quotes": {
      "get": {
        "summary": "GET /rfqs/{block_id}/quotes",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetRfqsBlockIdQuotes",
        "description": "Get the quotes for a given RFQ block.",
        "responses": {
          "200": {
            "description": "List of RFQ quotes.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetRfqsBlockIdQuotes200Response",
                  "type": "object",
                  "properties": {
                    "block_id": {
                      "$ref": "#/components/schemas/block_id_response"
                    },
                    "asks": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "quote_id": {
                            "$ref": "#/components/schemas/quote_id_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "is_buy": {
                            "$ref": "#/components/schemas/is_buy_response"
                          },
                          "limit_price": {
                            "$ref": "#/components/schemas/limit_price_response"
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
                                "side": {
                                  "$ref": "#/components/schemas/side_response"
                                },
                                "price": {
                                  "$ref": "#/components/schemas/price_response"
                                },
                                "iv": {
                                  "$ref": "#/components/schemas/iv_response"
                                }
                              },
                              "required": [
                                "instrument_id",
                                "instrument_name",
                                "side",
                                "price"
                              ]
                            }
                          },
                          "quote_status": {
                            "$ref": "#/components/schemas/quote_status_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "quote_id",
                          "amount",
                          "is_buy",
                          "limit_price",
                          "quote_status",
                          "created_timestamp"
                        ]
                      }
                    },
                    "bids": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "quote_id": {
                            "$ref": "#/components/schemas/quote_id_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "is_buy": {
                            "$ref": "#/components/schemas/is_buy_response"
                          },
                          "limit_price": {
                            "$ref": "#/components/schemas/limit_price_response"
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
                                "side": {
                                  "$ref": "#/components/schemas/side_response"
                                },
                                "price": {
                                  "$ref": "#/components/schemas/price_response"
                                },
                                "iv": {
                                  "$ref": "#/components/schemas/iv_response"
                                }
                              },
                              "required": [
                                "instrument_id",
                                "instrument_name",
                                "side",
                                "price"
                              ]
                            }
                          },
                          "quote_status": {
                            "$ref": "#/components/schemas/quote_status_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "quote_id",
                          "amount",
                          "is_buy",
                          "limit_price",
                          "quote_status",
                          "created_timestamp"
                        ]
                      }
                    }
                  },
                  "required": [
                    "block_id"
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
        "parameters": [
          {
            "$ref": "#/components/parameters/BlockIdPath"
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
      "description": "Private APIs require authentication",
      "name": "Private API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```