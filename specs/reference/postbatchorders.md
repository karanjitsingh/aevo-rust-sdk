> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /batch-orders

Creates a deterministic batch of GTC limit orders. All legs must share account, instrument, side, post_only, and reduce_only.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "amount": {
        "title": "amount",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number.",
        "example": "1000000"
      },
      "close_position": {
        "title": "close_position",
        "type": "boolean",
        "description": "Is order a close position TPSL order.",
        "example": true
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
      "instrument": {
        "title": "instrument",
        "type": "integer",
        "description": "Instrument ID number.",
        "example": 12
      },
      "is_buy": {
        "title": "is_buy",
        "type": "boolean",
        "description": "True for long order, false for short order.",
        "example": true
      },
      "limit_price": {
        "title": "limit_price",
        "type": "string",
        "description": "Order limit price. In 6 decimals fixed number.",
        "example": 1000000
      },
      "maker": {
        "title": "maker",
        "type": "string",
        "description": "Account's Ethereum address.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "mmp": {
        "title": "mmp",
        "type": "boolean",
        "description": "Flag to include order into MMP.",
        "example": false
      },
      "parent_order_id": {
        "title": "parent_order_id",
        "type": "string",
        "description": "Order ID of the order which when cancelled, cancels this order as well",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
      },
      "partial_position": {
        "title": "partial_position",
        "type": "boolean",
        "description": "Is order a partial position TPSL order.",
        "example": false
      },
      "perps_plus": {
        "title": "perps_plus",
        "type": "boolean",
        "description": "Whether this options order is part of a perps plus position",
        "example": false
      },
      "post_only": {
        "title": "post_only",
        "type": "boolean",
        "description": "True for post-only maker orders, false for standard orders.",
        "example": false
      },
      "reduce_only": {
        "title": "reduce_only",
        "type": "boolean",
        "description": "True for reduce-only orders, false for standard orders.",
        "example": false
      },
      "salt": {
        "title": "salt",
        "type": "string",
        "description": "A randomly generated number to guarantee transaction uniqueness. In 6 decimals fixed number.",
        "example": "12345678"
      },
      "self_trade_prevention": {
        "title": "self_trade_prevention",
        "type": "string",
        "description": "Can be set to EXPIRE_NONE, EXPIRE_MAKER OR EXPIRE_TAKER (EXPIRE_NONE is set by default)",
        "enum": [
          "EXPIRE_NONE",
          "EXPIRE_TAKER",
          "EXPIRE_MAKER"
        ]
      },
      "signature": {
        "title": "signature",
        "type": "string",
        "description": "Hash of order payload signature signed by the account.",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "stop": {
        "title": "stop",
        "type": "string",
        "description": "Type of stop order.",
        "enum": [
          "STOP_LOSS",
          "TAKE_PROFIT"
        ]
      },
      "success_response": {
        "title": "success_response",
        "type": "boolean",
        "description": "Request successful if true. Eg. `true`",
        "example": true
      },
      "time_in_force": {
        "title": "time_in_force",
        "type": "string",
        "description": "Can be set to GTC or IOC (GTC is set by default)",
        "enum": [
          "GTC",
          "IOC"
        ]
      },
      "timestamp": {
        "title": "timestamp",
        "type": "string",
        "description": "Timestamp used in order signing in UNIX timestamp in seconds.",
        "example": "1680249600"
      },
      "trigger": {
        "title": "trigger",
        "type": "string",
        "description": "The price to trigger the stop order at. `stop` is required when `trigger` is specified.",
        "example": 1836.74
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
    "/batch-orders": {
      "post": {
        "summary": "POST /batch-orders",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostBatchOrders",
        "description": "Creates a deterministic batch of GTC limit orders. All legs must share account, instrument, side, post_only, and reduce_only.",
        "responses": {
          "200": {
            "description": "Batch order created.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostBatchOrders200Response",
                  "type": "object",
                  "properties": {
                    "success": {
                      "$ref": "#/components/schemas/success_response"
                    }
                  },
                  "required": [
                    "success"
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
                "title": "PostBatchOrdersPayload",
                "type": "object",
                "properties": {
                  "orders": {
                    "type": "array",
                    "items": {
                      "type": "object",
                      "properties": {
                        "instrument": {
                          "$ref": "#/components/schemas/instrument"
                        },
                        "maker": {
                          "$ref": "#/components/schemas/maker"
                        },
                        "is_buy": {
                          "$ref": "#/components/schemas/is_buy"
                        },
                        "amount": {
                          "$ref": "#/components/schemas/amount"
                        },
                        "limit_price": {
                          "$ref": "#/components/schemas/limit_price"
                        },
                        "salt": {
                          "$ref": "#/components/schemas/salt"
                        },
                        "signature": {
                          "$ref": "#/components/schemas/signature"
                        },
                        "timestamp": {
                          "$ref": "#/components/schemas/timestamp"
                        },
                        "post_only": {
                          "$ref": "#/components/schemas/post_only"
                        },
                        "reduce_only": {
                          "$ref": "#/components/schemas/reduce_only"
                        },
                        "time_in_force": {
                          "$ref": "#/components/schemas/time_in_force"
                        },
                        "mmp": {
                          "$ref": "#/components/schemas/mmp"
                        },
                        "stop": {
                          "$ref": "#/components/schemas/stop"
                        },
                        "trigger": {
                          "$ref": "#/components/schemas/trigger"
                        },
                        "close_position": {
                          "$ref": "#/components/schemas/close_position"
                        },
                        "partial_position": {
                          "$ref": "#/components/schemas/partial_position"
                        },
                        "parent_order_id": {
                          "$ref": "#/components/schemas/parent_order_id"
                        },
                        "perps_plus": {
                          "$ref": "#/components/schemas/perps_plus"
                        },
                        "self_trade_prevention": {
                          "$ref": "#/components/schemas/self_trade_prevention"
                        }
                      },
                      "required": [
                        "instrument",
                        "maker",
                        "is_buy",
                        "amount",
                        "limit_price",
                        "salt",
                        "signature",
                        "timestamp"
                      ]
                    }
                  }
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