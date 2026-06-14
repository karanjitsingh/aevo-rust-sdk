> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /mmp

Sets market maker protection (MMP) settings.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "amount_limit": {
        "title": "amount_limit",
        "type": "string",
        "description": "Amount limit setting for market maker protection (MMP). In 6 decimals fixed number.",
        "example": 1000000
      },
      "asset": {
        "title": "asset",
        "type": "string",
        "description": "Name of underlying asset.",
        "example": "ETH"
      },
      "delta_limit": {
        "title": "delta_limit",
        "type": "number",
        "description": "Delta limit setting for market maker protection (MMP). In unsigned float.",
        "example": 0.1
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
      "frozen": {
        "title": "frozen",
        "type": "integer",
        "description": "Duration in seconds during which the account will be frozen. If set to 0, manual reset is required.",
        "example": 30
      },
      "interval": {
        "title": "interval",
        "type": "integer",
        "description": "Interval in seconds. A setting for market maker protection (MMP). If set to 0, MMP is disabled.",
        "example": 30
      },
      "success_response": {
        "title": "success_response",
        "type": "boolean",
        "description": "Request successful if true. Eg. `true`",
        "example": true
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
    "/mmp": {
      "post": {
        "summary": "POST /mmp",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostMmp",
        "description": "Sets market maker protection (MMP) settings.",
        "responses": {
          "200": {
            "description": "Setting successful.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostMmp200Response",
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
                "title": "PostMmpPayload",
                "type": "object",
                "properties": {
                  "interval": {
                    "$ref": "#/components/schemas/interval"
                  },
                  "frozen": {
                    "$ref": "#/components/schemas/frozen"
                  },
                  "amount_limit": {
                    "$ref": "#/components/schemas/amount_limit"
                  },
                  "delta_limit": {
                    "$ref": "#/components/schemas/delta_limit"
                  },
                  "asset": {
                    "$ref": "#/components/schemas/asset"
                  }
                },
                "required": [
                  "interval",
                  "frozen",
                  "amount_limit",
                  "asset"
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