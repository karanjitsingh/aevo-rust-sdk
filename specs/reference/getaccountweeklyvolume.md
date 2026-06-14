> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /account/weekly-volume

Returns the account's weekly trading volume and multiplier tier for LP NFT distribution.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
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
      "multiplier_response": {
        "title": "multiplier_response",
        "type": "string",
        "description": "Volume multiplier for LP NFT distribution Eg. `2`",
        "example": "2"
      },
      "tier_response": {
        "title": "tier_response",
        "type": "string",
        "description": "Volume tier for LP NFT distribution (1-4) Eg. `2`",
        "example": "2"
      },
      "week_end_response": {
        "title": "week_end_response",
        "type": "string",
        "description": "End of the week Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "week_start_response": {
        "title": "week_start_response",
        "type": "string",
        "description": "Start of the week Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "weekly_volume_response": {
        "title": "weekly_volume_response",
        "type": "string",
        "description": "Weekly trading volume in USD Eg. `1234567.89`",
        "example": "1234567.89"
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
    "/account/weekly-volume": {
      "get": {
        "summary": "GET /account/weekly-volume",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetAccountWeeklyVolume",
        "description": "Returns the account's weekly trading volume and multiplier tier for LP NFT distribution.",
        "responses": {
          "200": {
            "description": "Weekly volume and multiplier tier.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetAccountWeeklyVolume200Response",
                  "type": "object",
                  "properties": {
                    "weekly_volume": {
                      "$ref": "#/components/schemas/weekly_volume_response"
                    },
                    "multiplier": {
                      "$ref": "#/components/schemas/multiplier_response"
                    },
                    "tier": {
                      "$ref": "#/components/schemas/tier_response"
                    },
                    "week_start": {
                      "$ref": "#/components/schemas/week_start_response"
                    },
                    "week_end": {
                      "$ref": "#/components/schemas/week_end_response"
                    }
                  },
                  "required": [
                    "weekly_volume",
                    "multiplier",
                    "tier",
                    "week_start",
                    "week_end"
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