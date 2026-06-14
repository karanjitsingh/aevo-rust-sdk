> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /twap-order-history

Returns TWAP child order execution history for the account.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
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
      "StartTimeQueryOptional": {
        "name": "start_time",
        "in": "query",
        "description": "Entries created prior (<) to start time are excluded in UNIX timestamp in nanoseconds. Defaults to `0`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 1672531200000000000
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
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "avg_price_response": {
        "title": "avg_price_response",
        "type": "string",
        "description": "Average entry price of the position. Eg. `12.23`",
        "example": "12.23"
      },
      "close_position_response": {
        "title": "close_position_response",
        "type": "boolean",
        "description": "Is order a close position TPSL order. Eg. `true`",
        "example": true
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
      "error_response": {
        "title": "error_response",
        "type": "string",
        "description": "The error code for a failed trade Eg. `IOC_ORDER_REJECTED`",
        "example": "IOC_ORDER_REJECTED"
      },
      "event_type_response": {
        "title": "event_type_response",
        "type": "string",
        "description": "Type of event that triggered this order history entry Eg. `MATCH_ORDER`",
        "example": "MATCH_ORDER"
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
      "order_status_response": {
        "title": "order_status_response",
        "type": "string",
        "description": "Order status. Eg. `filled`",
        "enum": [
          "filled",
          "partial",
          "opened",
          "cancelled",
          "expired",
          "rejected",
          "stop_order"
        ]
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
      "post_only_response": {
        "title": "post_only_response",
        "type": "boolean",
        "description": "True for post-only maker orders, false for standard orders. Eg. `false`",
        "example": false
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "reduce_only_response": {
        "title": "reduce_only_response",
        "type": "boolean",
        "description": "True for reduce-only orders, false for standard orders. Eg. `false`",
        "example": false
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
      "stop_type_response": {
        "title": "stop_type_response",
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
      "system_type_response": {
        "title": "system_type_response",
        "type": "string",
        "description": "Client that placed the order (API is set by default) Eg. `API`",
        "enum": [
          "API",
          "WEB",
          "MOBILE_WEB",
          "MOBILE_APP"
        ]
      },
      "timestamp_response": {
        "title": "timestamp_response",
        "type": "string",
        "description": "Timestamp in UNIX in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "trigger_response": {
        "title": "trigger_response",
        "type": "string",
        "description": "The price to trigger the stop order at. `stop` is required when `trigger` is specified. Eg. `1836.74`",
        "example": "1836.74"
      },
      "twap_parent_id_response": {
        "title": "twap_parent_id_response",
        "type": "string",
        "description": "Parent TWAP order ID for child orders Eg. `0x1234567890abcdef`",
        "example": "0x1234567890abcdef"
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
    "/twap-order-history": {
      "get": {
        "summary": "GET /twap-order-history",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetTwapOrderHistory",
        "description": "Returns TWAP child order execution history for the account.",
        "responses": {
          "200": {
            "description": "TWAP order history.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetTwapOrderHistory200Response",
                  "type": "object",
                  "properties": {
                    "count": {
                      "$ref": "#/components/schemas/count_response"
                    },
                    "twap_order_history": {
                      "type": "array",
                      "items": {
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
                          "instrument_type": {
                            "$ref": "#/components/schemas/instrument_type_response"
                          },
                          "order_type": {
                            "$ref": "#/components/schemas/order_type_response"
                          },
                          "side": {
                            "$ref": "#/components/schemas/side_response"
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
                          "filled": {
                            "$ref": "#/components/schemas/filled_response"
                          },
                          "error": {
                            "$ref": "#/components/schemas/error_response"
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
                          "order_status": {
                            "$ref": "#/components/schemas/order_status_response"
                          },
                          "event_type": {
                            "$ref": "#/components/schemas/event_type_response"
                          },
                          "stop_type": {
                            "$ref": "#/components/schemas/stop_type_response"
                          },
                          "mark_price": {
                            "$ref": "#/components/schemas/mark_price_response"
                          },
                          "trigger": {
                            "$ref": "#/components/schemas/trigger_response"
                          },
                          "post_only": {
                            "$ref": "#/components/schemas/post_only_response"
                          },
                          "reduce_only": {
                            "$ref": "#/components/schemas/reduce_only_response"
                          },
                          "close_position": {
                            "$ref": "#/components/schemas/close_position_response"
                          },
                          "twap_parent_id": {
                            "$ref": "#/components/schemas/twap_parent_id_response"
                          },
                          "system_type": {
                            "$ref": "#/components/schemas/system_type_response"
                          },
                          "timestamp": {
                            "$ref": "#/components/schemas/timestamp_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "order_id",
                          "account",
                          "instrument_id",
                          "instrument_name",
                          "instrument_type",
                          "order_type",
                          "side",
                          "amount",
                          "price",
                          "filled",
                          "order_status",
                          "event_type",
                          "twap_parent_id",
                          "timestamp",
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
            "$ref": "#/components/parameters/StartTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/EndTimeQueryOptional"
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
      "description": "Private APIs require authentication",
      "name": "Private API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```