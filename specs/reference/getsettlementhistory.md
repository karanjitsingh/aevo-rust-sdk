> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /settlement-history

Returns the historical settlement prices for a given asset.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AssetQueryOptional": {
        "name": "asset",
        "in": "query",
        "description": "Name of underlying asset.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "ETH"
        }
      },
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
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
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
      "expiry_response": {
        "title": "expiry_response",
        "type": "string",
        "description": "Option expiry in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "settlement_price_response": {
        "title": "settlement_price_response",
        "type": "string",
        "description": "Settlement price at expiry. Eg. `1734.23`",
        "example": "1734.23"
      },
      "settlement_timestamp_response": {
        "title": "settlement_timestamp_response",
        "type": "string",
        "description": "Timestamp at which the settlement price is set Eg. `1680249600000000000`",
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
    "/settlement-history": {
      "get": {
        "summary": "GET /settlement-history",
        "tags": [
          "Public API"
        ],
        "operationId": "GetSettlementHistory",
        "description": "Returns the historical settlement prices for a given asset.",
        "responses": {
          "200": {
            "description": "Array of historical settlement price of the asset.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "GetSettlementHistory200Response",
                    "type": "object",
                    "properties": {
                      "asset": {
                        "$ref": "#/components/schemas/asset_response"
                      },
                      "expiry": {
                        "$ref": "#/components/schemas/expiry_response"
                      },
                      "settlement_price": {
                        "$ref": "#/components/schemas/settlement_price_response"
                      },
                      "settlement_timestamp": {
                        "$ref": "#/components/schemas/settlement_timestamp_response"
                      }
                    },
                    "required": [
                      "asset",
                      "expiry",
                      "settlement_price",
                      "settlement_timestamp"
                    ]
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
            "$ref": "#/components/parameters/AssetQueryOptional"
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