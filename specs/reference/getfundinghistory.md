> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /funding-history

Returns the funding rate history for the instrument.

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
      "InstrumentNameQueryOptional": {
        "name": "instrument_name",
        "in": "query",
        "description": "Instrument name.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "ETH-PERP"
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
      "funding_history_response": {
        "title": "funding_history_response",
        "type": "array",
        "description": "List of [instrument name, timestamp, funding rate, mark price]. Timestamp is in UNIX nanoseconds. Funding rate is in decimals. Eg. `[ETH-PERP 1680249600000000000 0.000123 1892.82]`",
        "example": [
          "ETH-PERP",
          "1680249600000000000",
          "0.000123",
          "1892.82"
        ],
        "items": {
          "type": "string"
        }
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
    "/funding-history": {
      "get": {
        "summary": "GET /funding-history",
        "tags": [
          "Public API"
        ],
        "operationId": "GetFundingHistory",
        "description": "Returns the funding rate history for the instrument.",
        "responses": {
          "200": {
            "description": "Funding rate history of the instrument.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetFundingHistory200Response",
                  "type": "object",
                  "properties": {
                    "funding_history": {
                      "title": "funding_history_array",
                      "type": "array",
                      "description": "List of [instrument name, timestamp, funding rate, mark price]. Timestamp is in UNIX nanoseconds. Funding rate is in decimals.",
                      "example": [
                        [
                          "ETH-PERP",
                          "1680249600000000000",
                          "0.000123",
                          "1892.82"
                        ]
                      ],
                      "items": {
                        "$ref": "#/components/schemas/funding_history_response"
                      },
                      "required": [
                        "funding_history_array"
                      ]
                    }
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
            "$ref": "#/components/parameters/InstrumentNameQueryOptional"
          },
          {
            "$ref": "#/components/parameters/StartTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/EndTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/LimitQueryOptional"
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