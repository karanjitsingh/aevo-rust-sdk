> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /funding

Returns the current funding rate for the instrument.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "InstrumentNamePerpQuery": {
        "name": "instrument_name",
        "in": "query",
        "description": "Instrument name.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "ETH-PERP"
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
      "funding_rate_response": {
        "title": "funding_rate_response",
        "type": "string",
        "description": "Funding rate in decimals. Eg. `0.00122`",
        "example": "0.00122"
      },
      "next_epoch_response": {
        "title": "next_epoch_response",
        "type": "string",
        "description": "Next epoch in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
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
    "/funding": {
      "get": {
        "summary": "GET /funding",
        "tags": [
          "Public API"
        ],
        "operationId": "GetFunding",
        "description": "Returns the current funding rate for the instrument.",
        "responses": {
          "200": {
            "description": "Funding rate of the instrument.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetFunding200Response",
                  "type": "object",
                  "properties": {
                    "funding_rate": {
                      "$ref": "#/components/schemas/funding_rate_response"
                    },
                    "next_epoch": {
                      "$ref": "#/components/schemas/next_epoch_response"
                    }
                  },
                  "required": [
                    "funding_rate",
                    "next_epoch"
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
            "$ref": "#/components/parameters/InstrumentNamePerpQuery"
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