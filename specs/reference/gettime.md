> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /time

Returns the server time

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "block_response": {
        "title": "block_response",
        "type": "string",
        "description": "1366770 Eg. `2023-08-24 07:15:51.448450894 +0000 UTC`",
        "example": "2023-08-24 07:15:51.448450894 +0000 UTC"
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
      "name_response": {
        "title": "name_response",
        "type": "string",
        "description": "name Eg. `5732cc1f14e842f393e8cdecb49f02fa`",
        "example": "5732cc1f14e842f393e8cdecb49f02fa"
      },
      "sequence_response": {
        "title": "sequence_response",
        "type": "string",
        "description": "Sequence of the current time Eg. `1602711444`",
        "example": "1602711444"
      },
      "time_response": {
        "title": "time_response",
        "type": "string",
        "description": "ISO string of the current time Eg. `2023-08-24 07:15:51.448450894 +0000 UTC`",
        "example": "2023-08-24 07:15:51.448450894 +0000 UTC"
      },
      "timestamp_response": {
        "title": "timestamp_response",
        "type": "string",
        "description": "Timestamp in UNIX in nanoseconds. Eg. `1680249600000000000`",
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
    "/time": {
      "get": {
        "summary": "GET /time",
        "tags": [
          "Public API"
        ],
        "operationId": "GetTime",
        "description": "Returns the server time",
        "responses": {
          "200": {
            "description": "Server time.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetTime200Response",
                  "type": "object",
                  "properties": {
                    "name": {
                      "$ref": "#/components/schemas/name_response"
                    },
                    "timestamp": {
                      "$ref": "#/components/schemas/timestamp_response"
                    },
                    "time": {
                      "$ref": "#/components/schemas/time_response"
                    },
                    "sequence": {
                      "$ref": "#/components/schemas/sequence_response"
                    },
                    "block": {
                      "$ref": "#/components/schemas/block_response"
                    }
                  },
                  "required": [
                    "name",
                    "timestamp",
                    "time",
                    "sequence",
                    "block"
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```