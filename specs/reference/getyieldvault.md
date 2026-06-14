> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /yield-vault

Returns the yield vault information for the given yield vault.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "apy_response": {
        "title": "apy_response",
        "type": "string",
        "description": "Annualized Percentage Return of a yield vault or strategy Eg. `12`",
        "example": "12"
      },
      "aum_liquid_response": {
        "title": "aum_liquid_response",
        "type": "string",
        "description": "Liquid AUM of a strategy Eg. `12`",
        "example": "12"
      },
      "aum_response": {
        "title": "aum_response",
        "type": "string",
        "description": "AUM of a strategy Eg. `12`",
        "example": "12"
      },
      "cap_response": {
        "title": "cap_response",
        "type": "string",
        "description": "Capacity of a strategy Eg. `12`",
        "example": "12"
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
      "is_paused_response": {
        "title": "is_paused_response",
        "type": "boolean",
        "description": "Pause status of a yield vault or strategy Eg. `false`",
        "example": false
      },
      "pending_withdrawals_response": {
        "title": "pending_withdrawals_response",
        "type": "string",
        "description": "Pending withdrawals for a yield vault or strategy Eg. `12`",
        "example": "12"
      },
      "pps_response": {
        "title": "pps_response",
        "type": "string",
        "description": "Price per share of a yield vault or strategy Eg. `1.04`",
        "example": "1.04"
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
    "/yield-vault": {
      "get": {
        "summary": "GET /yield-vault",
        "tags": [
          "Public API"
        ],
        "operationId": "GetYieldVault",
        "description": "Returns the yield vault information for the given yield vault.",
        "responses": {
          "200": {
            "description": "Yield vault information.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetYieldVault200Response",
                  "type": "object",
                  "properties": {
                    "aum": {
                      "$ref": "#/components/schemas/aum_response"
                    },
                    "aum_liquid": {
                      "$ref": "#/components/schemas/aum_liquid_response"
                    },
                    "cap": {
                      "$ref": "#/components/schemas/cap_response"
                    },
                    "pps": {
                      "$ref": "#/components/schemas/pps_response"
                    },
                    "apy": {
                      "$ref": "#/components/schemas/apy_response"
                    },
                    "pending_withdrawals": {
                      "$ref": "#/components/schemas/pending_withdrawals_response"
                    },
                    "is_paused": {
                      "$ref": "#/components/schemas/is_paused_response"
                    }
                  },
                  "required": [
                    "aum",
                    "aum_liquid",
                    "cap",
                    "pps",
                    "apy",
                    "pending_withdrawals",
                    "is_paused"
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