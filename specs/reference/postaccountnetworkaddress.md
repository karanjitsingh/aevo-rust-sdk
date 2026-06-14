> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /account/network-address

Set a non-eth network's address to the account

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
      "network": {
        "title": "network",
        "type": "string",
        "description": "A non-eth network.",
        "enum": [
          "SOL",
          "SUI"
        ]
      },
      "network_address": {
        "title": "network_address",
        "type": "string",
        "description": "Account's non-eth network address.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "network_address_response": {
        "title": "network_address_response",
        "type": "string",
        "description": "Account's non-eth network address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
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
    "/account/network-address": {
      "post": {
        "summary": "POST /account/network-address",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostAccountNetworkAddress",
        "description": "Set a non-eth network's address to the account",
        "responses": {
          "200": {
            "description": "Whether or not the setting was successfully updated.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostAccountNetworkAddress200Response",
                  "type": "object",
                  "properties": {
                    "success": {
                      "$ref": "#/components/schemas/success_response"
                    },
                    "network_address": {
                      "$ref": "#/components/schemas/network_address_response"
                    }
                  },
                  "required": [
                    "success",
                    "network_address"
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
                "title": "PostAccountNetworkAddressPayload",
                "type": "object",
                "properties": {
                  "network": {
                    "$ref": "#/components/schemas/network"
                  },
                  "network_address": {
                    "$ref": "#/components/schemas/network_address"
                  }
                },
                "required": [
                  "network",
                  "network_address"
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