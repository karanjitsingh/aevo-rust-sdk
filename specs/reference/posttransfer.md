> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /transfer

Transfer USDC between accounts.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "account": {
        "title": "account",
        "type": "string",
        "description": "Account's Ethereum address.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "amount_usdc": {
        "title": "amount_usdc",
        "type": "string",
        "description": "Amount in USDC. In 6 decimals fixed number.",
        "example": 1000000
      },
      "collateral": {
        "title": "collateral",
        "type": "string",
        "description": "Ethereum address of the collateral asset.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
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
      "label": {
        "title": "label",
        "type": "string",
        "description": "Transfer label.",
        "example": "Rewards"
      },
      "referenceId": {
        "title": "referenceId",
        "type": "string",
        "description": "Transfer reference id.",
        "example": "1"
      },
      "salt": {
        "title": "salt",
        "type": "string",
        "description": "A randomly generated number to guarantee transaction uniqueness. In 6 decimals fixed number.",
        "example": "12345678"
      },
      "signature": {
        "title": "signature",
        "type": "string",
        "description": "Hash of order payload signature signed by the account.",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "success_response": {
        "title": "success_response",
        "type": "boolean",
        "description": "Request successful if true. Eg. `true`",
        "example": true
      },
      "to": {
        "title": "to",
        "type": "string",
        "description": "Ethereum address to send withdrawals to.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
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
    "/transfer": {
      "post": {
        "summary": "POST /transfer",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostTransfer",
        "description": "Transfer USDC between accounts.",
        "responses": {
          "200": {
            "description": "Transfer status.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostTransfer200Response",
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
                "title": "PostTransferPayload",
                "type": "object",
                "properties": {
                  "account": {
                    "$ref": "#/components/schemas/account"
                  },
                  "collateral": {
                    "$ref": "#/components/schemas/collateral"
                  },
                  "to": {
                    "$ref": "#/components/schemas/to"
                  },
                  "amount": {
                    "$ref": "#/components/schemas/amount_usdc"
                  },
                  "salt": {
                    "$ref": "#/components/schemas/salt"
                  },
                  "signature": {
                    "$ref": "#/components/schemas/signature"
                  },
                  "label": {
                    "$ref": "#/components/schemas/label"
                  },
                  "referenceId": {
                    "$ref": "#/components/schemas/referenceId"
                  }
                },
                "required": [
                  "account",
                  "collateral",
                  "to",
                  "amount",
                  "salt",
                  "signature"
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