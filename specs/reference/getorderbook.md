> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /orderbook

Returns the orderbook for a given symbol.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "InstrumentNameQuery": {
        "name": "instrument_name",
        "in": "query",
        "description": "Instrument name.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "ETH-30JUN23-1600-C"
        }
      }
    },
    "schemas": {
      "asks_response": {
        "title": "asks_response",
        "type": "array",
        "description": "Array of 3 elements, price in USD, contract amount, and IV respectively. Eg. `[1 100 12]`",
        "example": [
          "1",
          "100",
          "12"
        ],
        "items": {
          "type": "string"
        }
      },
      "bids_response": {
        "title": "bids_response",
        "type": "array",
        "description": "Array of 3 elements, price in USD, contract amount, and IV respectively. Eg. `[1 100 12]`",
        "example": [
          "1",
          "100",
          "12"
        ],
        "items": {
          "type": "string"
        }
      },
      "checksum_response": {
        "title": "checksum_response",
        "type": "string",
        "description": "Payload checksum. Eg. `8479283742`",
        "example": "8479283742"
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
      "last_updated_response": {
        "title": "last_updated_response",
        "type": "string",
        "description": "Last updated timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "type_response": {
        "title": "type_response",
        "type": "string",
        "description": "For REST API, the value is always set to `snapshot`. Eg. `snapshot`",
        "example": "snapshot"
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
    "/orderbook": {
      "get": {
        "summary": "GET /orderbook",
        "tags": [
          "Public API"
        ],
        "operationId": "GetOrderbook",
        "description": "Returns the orderbook for a given symbol.",
        "responses": {
          "200": {
            "description": "Instrument orderbook.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetOrderbook200Response",
                  "type": "object",
                  "properties": {
                    "type": {
                      "$ref": "#/components/schemas/type_response"
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
                    "bids": {
                      "title": "bids_array",
                      "type": "array",
                      "description": "Array of 3 elements, price in USD, contract amount, and IV respectively.",
                      "example": [
                        [
                          "1",
                          "100",
                          "12"
                        ]
                      ],
                      "items": {
                        "$ref": "#/components/schemas/bids_response"
                      },
                      "required": [
                        "bids_array"
                      ]
                    },
                    "asks": {
                      "title": "asks_array",
                      "type": "array",
                      "description": "Array of 3 elements, price in USD, contract amount, and IV respectively.",
                      "example": [
                        [
                          "1",
                          "100",
                          "12"
                        ]
                      ],
                      "items": {
                        "$ref": "#/components/schemas/asks_response"
                      },
                      "required": [
                        "asks_array"
                      ]
                    },
                    "last_updated": {
                      "$ref": "#/components/schemas/last_updated_response"
                    },
                    "checksum": {
                      "$ref": "#/components/schemas/checksum_response"
                    }
                  },
                  "required": [
                    "type",
                    "instrument_id",
                    "instrument_name",
                    "instrument_type",
                    "last_updated",
                    "checksum"
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
            "$ref": "#/components/parameters/InstrumentNameQuery"
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