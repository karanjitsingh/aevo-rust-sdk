> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /index-history

Returns the historical index price for a given asset.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AssetQuery": {
        "name": "asset",
        "in": "query",
        "description": "Name of underlying asset.",
        "required": true,
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
      "ResolutionQueryOptional": {
        "name": "resolution",
        "in": "query",
        "description": "Interval between entries in seconds. Must be a multiple of 30. Defaults to `30`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 30
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
      "history_response": {
        "title": "history_response",
        "type": "array",
        "description": "List of [timestamp, price]. Timestamp is in UNIX nanoseconds. Eg. `[1680249600000000000 1323.45]`",
        "example": [
          "1680249600000000000",
          "1323.45"
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
    "/index-history": {
      "get": {
        "summary": "GET /index-history",
        "tags": [
          "Public API"
        ],
        "operationId": "GetIndexHistory",
        "description": "Returns the historical index price for a given asset.",
        "responses": {
          "200": {
            "description": "Array of historical index price of the asset.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetIndexHistory200Response",
                  "type": "object",
                  "properties": {
                    "history": {
                      "title": "history_array",
                      "type": "array",
                      "description": "List of [timestamp, price]. Timestamp is in UNIX nanoseconds.",
                      "example": [
                        [
                          "1680249600000000000",
                          "1323.45"
                        ]
                      ],
                      "items": {
                        "$ref": "#/components/schemas/history_response"
                      },
                      "required": [
                        "history_array"
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
            "$ref": "#/components/parameters/AssetQuery"
          },
          {
            "$ref": "#/components/parameters/ResolutionQueryOptional"
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