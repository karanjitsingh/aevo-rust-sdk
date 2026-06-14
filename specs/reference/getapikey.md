> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /api-key

Returns the API key information.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "ApiKeyQuery": {
        "name": "api_key",
        "in": "query",
        "description": "Account's API Key.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "URPtt6eNCXgL8ERuchphUretdaga2smF"
        }
      },
      "SignatureQuery": {
        "name": "signature",
        "in": "query",
        "description": "Hash of order payload signature signed by the account.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
        }
      },
      "TimestampQuery": {
        "name": "timestamp",
        "in": "query",
        "description": "Timestamp in UNIX in nanoseconds.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "1680249600000000000"
        }
      }
    },
    "schemas": {
      "api_key_response": {
        "title": "api_key_response",
        "type": "string",
        "description": "Account's API Key. Eg. `URPtt6eNCXgL8ERuchphUretdaga2smF`",
        "example": "URPtt6eNCXgL8ERuchphUretdaga2smF"
      },
      "api_secret_response": {
        "title": "api_secret_response",
        "type": "string",
        "description": "Client's API Secret Eg. `0140af7046a63530fc4bd319823d6eee98086ef0d446584b42f68b640b60c457`",
        "example": "0140af7046a63530fc4bd319823d6eee98086ef0d446584b42f68b640b60c457"
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
      "ip_addresses_response": {
        "title": "ip_addresses_response",
        "type": "array",
        "description": "Whitelisted client's IP address for API access. Eg. `[1.1.1.1 2.2.2.2]`",
        "example": [
          "1.1.1.1",
          "2.2.2.2"
        ],
        "items": {
          "type": "string"
        }
      },
      "name_response": {
        "title": "name_response",
        "type": "string",
        "description": "name Eg. `5732cc1f14e842f393e8cdecb49f02fa`",
        "example": "5732cc1f14e842f393e8cdecb49f02fa"
      },
      "read_only_response": {
        "title": "read_only_response",
        "type": "boolean",
        "description": "API can only access read-only endpoints if true. Eg. `true`",
        "example": true
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
    "/api-key": {
      "get": {
        "summary": "GET /api-key",
        "tags": [
          "Private API"
        ],
        "operationId": "GetApiKey",
        "description": "Returns the API key information.",
        "responses": {
          "200": {
            "description": "API key information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetApiKey200Response",
                  "type": "object",
                  "properties": {
                    "name": {
                      "$ref": "#/components/schemas/name_response"
                    },
                    "api_key": {
                      "$ref": "#/components/schemas/api_key_response"
                    },
                    "api_secret": {
                      "$ref": "#/components/schemas/api_secret_response"
                    },
                    "ip_addresses": {
                      "$ref": "#/components/schemas/ip_addresses_response"
                    },
                    "read_only": {
                      "$ref": "#/components/schemas/read_only_response"
                    },
                    "created_timestamp": {
                      "$ref": "#/components/schemas/created_timestamp_response"
                    }
                  },
                  "required": [
                    "api_key",
                    "api_secret",
                    "read_only",
                    "created_timestamp"
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
            "$ref": "#/components/parameters/ApiKeyQuery"
          },
          {
            "$ref": "#/components/parameters/TimestampQuery"
          },
          {
            "$ref": "#/components/parameters/SignatureQuery"
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