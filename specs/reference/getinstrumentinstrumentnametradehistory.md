> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /instrument/{instrument_name}/trade-history

Returns the trade history for the given instrument.

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
      "InstrumentNamePath": {
        "name": "instrument_name",
        "in": "path",
        "description": "Instrument name.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "ETH-30JUN23-1600-C"
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
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
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
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
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
      "trade_id_response": {
        "title": "trade_id_response",
        "type": "string",
        "description": "Unique ID of the trade. Eg. `DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2`",
        "example": "DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2"
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
    "/instrument/{instrument_name}/trade-history": {
      "get": {
        "summary": "GET /instrument/{instrument_name}/trade-history",
        "tags": [
          "Public API"
        ],
        "operationId": "GetInstrumentInstrumentNameTradeHistory",
        "description": "Returns the trade history for the given instrument.",
        "responses": {
          "200": {
            "description": "Instrument trade information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetInstrumentInstrumentNameTradeHistory200Response",
                  "type": "object",
                  "properties": {
                    "count": {
                      "$ref": "#/components/schemas/count_response"
                    },
                    "trade_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "trade_id": {
                            "$ref": "#/components/schemas/trade_id_response"
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
                          "side": {
                            "$ref": "#/components/schemas/side_response"
                          },
                          "price": {
                            "$ref": "#/components/schemas/price_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "trade_id",
                          "instrument_id",
                          "instrument_name",
                          "instrument_type",
                          "side",
                          "price",
                          "amount",
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
          },
          {
            "$ref": "#/components/parameters/StartTimeQueryOptional"
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