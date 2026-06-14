> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /twap-orders

Creates a new TWAP (Time-Weighted Average Price) order.

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
      "amount_float": {
        "title": "amount_float",
        "type": "string",
        "description": "Amount in unsigned float string",
        "example": "12.23"
      },
      "average_price_response": {
        "title": "average_price_response",
        "type": "string",
        "description": "Volume-weighted average price of executed slices Eg. `3500000000`",
        "example": "3500000000"
      },
      "created_time_response": {
        "title": "created_time_response",
        "type": "string",
        "description": "Timestamp when the TWAP order was created Eg. `2025-01-01T00:00:00Z`",
        "example": "2025-01-01T00:00:00Z"
      },
      "duration": {
        "title": "duration",
        "type": "integer",
        "description": "Duration of the RFQ block in seconds",
        "example": 30
      },
      "duration_seconds_response": {
        "title": "duration_seconds_response",
        "type": "string",
        "description": "Total duration of the TWAP order in seconds Eg. `3600`",
        "example": "3600"
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
      "executed_amount_response": {
        "title": "executed_amount_response",
        "type": "string",
        "description": "Amount executed so far in internal representation (6 decimals) Eg. `5000000`",
        "example": "5000000"
      },
      "instrument_id": {
        "title": "instrument_id",
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
      "interval_seconds_response": {
        "title": "interval_seconds_response",
        "type": "string",
        "description": "Interval between slices in seconds Eg. `36`",
        "example": "36"
      },
      "is_buy": {
        "title": "is_buy",
        "type": "boolean",
        "description": "True for long order, false for short order.",
        "example": true
      },
      "order_id_response": {
        "title": "order_id_response",
        "type": "string",
        "description": "Order ID is the hash of the order payload Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "reduce_only": {
        "title": "reduce_only",
        "type": "boolean",
        "description": "True for reduce-only orders, false for standard orders.",
        "example": false
      },
      "reduce_only_response": {
        "title": "reduce_only_response",
        "type": "boolean",
        "description": "True for reduce-only orders, false for standard orders. Eg. `false`",
        "example": false
      },
      "remaining_amount_response": {
        "title": "remaining_amount_response",
        "type": "string",
        "description": "Remaining amount to execute in internal representation (6 decimals) Eg. `5000000`",
        "example": "5000000"
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
      "slices_executed_response": {
        "title": "slices_executed_response",
        "type": "string",
        "description": "Number of slices executed so far Eg. `50`",
        "example": "50"
      },
      "slices_total_response": {
        "title": "slices_total_response",
        "type": "string",
        "description": "Total number of slices for the TWAP order Eg. `100`",
        "example": "100"
      },
      "status_response": {
        "title": "status_response",
        "type": "string",
        "description": "Status of the otc request Eg. `requested`",
        "example": "requested"
      },
      "total_amount_response": {
        "title": "total_amount_response",
        "type": "string",
        "description": "Total amount to execute in internal representation (6 decimals) Eg. `10000000`",
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
    "/twap-orders": {
      "post": {
        "summary": "POST /twap-orders",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostTwapOrders",
        "description": "Creates a new TWAP (Time-Weighted Average Price) order.",
        "responses": {
          "200": {
            "description": "TWAP order created.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostTwapOrders200Response",
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
                    "side": {
                      "$ref": "#/components/schemas/side_response"
                    },
                    "total_amount": {
                      "$ref": "#/components/schemas/total_amount_response"
                    },
                    "executed_amount": {
                      "$ref": "#/components/schemas/executed_amount_response"
                    },
                    "remaining_amount": {
                      "$ref": "#/components/schemas/remaining_amount_response"
                    },
                    "slices_total": {
                      "$ref": "#/components/schemas/slices_total_response"
                    },
                    "slices_executed": {
                      "$ref": "#/components/schemas/slices_executed_response"
                    },
                    "status": {
                      "$ref": "#/components/schemas/status_response"
                    },
                    "duration_seconds": {
                      "$ref": "#/components/schemas/duration_seconds_response"
                    },
                    "interval_seconds": {
                      "$ref": "#/components/schemas/interval_seconds_response"
                    },
                    "reduce_only": {
                      "$ref": "#/components/schemas/reduce_only_response"
                    },
                    "created_time": {
                      "$ref": "#/components/schemas/created_time_response"
                    },
                    "average_price": {
                      "$ref": "#/components/schemas/average_price_response"
                    }
                  },
                  "required": [
                    "order_id",
                    "account",
                    "instrument_id",
                    "instrument_name",
                    "side",
                    "total_amount",
                    "executed_amount",
                    "remaining_amount",
                    "slices_total",
                    "slices_executed",
                    "status",
                    "duration_seconds",
                    "interval_seconds",
                    "reduce_only"
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
                "title": "PostTwapOrdersPayload",
                "type": "object",
                "properties": {
                  "instrument_id": {
                    "$ref": "#/components/schemas/instrument_id"
                  },
                  "is_buy": {
                    "$ref": "#/components/schemas/is_buy"
                  },
                  "amount_float": {
                    "$ref": "#/components/schemas/amount_float"
                  },
                  "duration": {
                    "$ref": "#/components/schemas/duration"
                  },
                  "reduce_only": {
                    "$ref": "#/components/schemas/reduce_only"
                  }
                },
                "required": [
                  "instrument_id",
                  "is_buy",
                  "amount_float",
                  "duration"
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