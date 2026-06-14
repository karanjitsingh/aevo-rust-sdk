> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /otc/create-request

Create a new otc request

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "amount": {
        "title": "amount",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number.",
        "example": "1000000"
      },
      "asset": {
        "title": "asset",
        "type": "string",
        "description": "Name of underlying asset.",
        "example": "ETH"
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
      "expiry": {
        "title": "expiry",
        "type": "string",
        "description": "Signing key expiry in UNIX timestamp in seconds.",
        "example": 1685520000
      },
      "indicative_price": {
        "title": "indicative_price",
        "type": "string",
        "description": "Indicative price of the otc request",
        "example": "1000"
      },
      "option_type": {
        "title": "option_type",
        "type": "string",
        "description": "Type of option contract.",
        "enum": [
          "put",
          "call"
        ]
      },
      "strike": {
        "title": "strike",
        "type": "string",
        "description": "Option strike price.",
        "example": "2500"
      },
      "success_response": {
        "title": "success_response",
        "type": "boolean",
        "description": "Request successful if true. Eg. `true`",
        "example": true
      },
      "unwind_id": {
        "title": "unwind_id",
        "type": "integer",
        "description": "Otc id of position to unwind.",
        "example": 12
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
    "/otc/create-request": {
      "post": {
        "summary": "POST /otc/create-request",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostOtcCreateRequest",
        "description": "Create a new otc request",
        "responses": {
          "200": {
            "description": "Whether or not request was created successfully",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostOtcCreateRequest200Response",
                  "type": "object",
                  "properties": {
                    "success": {
                      "$ref": "#/components/schemas/success_response"
                    }
                  },
                  "required": [
                    "success"
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
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "title": "PostOtcCreateRequestPayload",
                "type": "object",
                "properties": {
                  "asset": {
                    "$ref": "#/components/schemas/asset"
                  },
                  "strike": {
                    "$ref": "#/components/schemas/strike"
                  },
                  "expiry": {
                    "$ref": "#/components/schemas/expiry"
                  },
                  "option_type": {
                    "$ref": "#/components/schemas/option_type"
                  },
                  "amount": {
                    "$ref": "#/components/schemas/amount"
                  },
                  "indicative_price": {
                    "$ref": "#/components/schemas/indicative_price"
                  },
                  "unwind_id": {
                    "$ref": "#/components/schemas/unwind_id"
                  }
                },
                "required": [
                  "amount",
                  "indicative_price"
                ]
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