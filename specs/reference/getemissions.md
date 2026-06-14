> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /emissions

Returns the farm + staking emissions

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
      "farming_emission_response": {
        "title": "farming_emission_response",
        "type": "string",
        "description": "Farming emission of $AEVO in an epoch, in $AEVO terms Eg. `569.2`",
        "example": "569.2"
      },
      "staking_emission_response": {
        "title": "staking_emission_response",
        "type": "string",
        "description": "Staking emission of $AEVO in an epoch, in $AEVO terms Eg. `593.4`",
        "example": "593.4"
      },
      "total_boosted_volume_response": {
        "title": "total_boosted_volume_response",
        "type": "string",
        "description": "Total boosted trading notional volume in an epoch, in USD terms. Eg. `123`",
        "example": "123"
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
    "/emissions": {
      "get": {
        "summary": "GET /emissions",
        "tags": [
          "Public API"
        ],
        "operationId": "GetEmissions",
        "description": "Returns the farm + staking emissions",
        "responses": {
          "200": {
            "description": "Emission schedule",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "GetEmissions200Response",
                    "type": "object",
                    "properties": {
                      "farming_emission": {
                        "$ref": "#/components/schemas/farming_emission_response"
                      },
                      "staking_emission": {
                        "$ref": "#/components/schemas/staking_emission_response"
                      },
                      "total_boosted_volume": {
                        "$ref": "#/components/schemas/total_boosted_volume_response"
                      }
                    },
                    "required": [
                      "farming_emission",
                      "staking_emission",
                      "total_boosted_volume"
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