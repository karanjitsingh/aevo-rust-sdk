> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /swap/preview

Previews a collateral swap

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "amount__swap_preview_response": {
        "title": "amount__swap_preview_response",
        "type": "string",
        "description": "The base amount Eg. `1`",
        "example": "1"
      },
      "base_amount": {
        "title": "base_amount",
        "type": "string",
        "description": "The collateral amount. In 6 decimals fixed number for USDT, 18 decimals for WETH, and 8 decimals for WBTC.",
        "example": "1000000"
      },
      "base_balance_response": {
        "title": "base_balance_response",
        "type": "string",
        "description": "The collateral balance. Eg. `12.23`",
        "example": "12.23"
      },
      "base_capacity_response": {
        "title": "base_capacity_response",
        "type": "string",
        "description": "The collateral swap capacity balance. Eg. `12.23`",
        "example": "12.23"
      },
      "collateral_asset": {
        "title": "collateral_asset",
        "type": "string",
        "description": "Name of the collateral asset.",
        "enum": [
          "USDC",
          "USDT",
          "WETH",
          "WBTC",
          "aeUSD",
          "SDAI",
          "weETH"
        ]
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
      "fee_rate_response": {
        "title": "fee_rate_response",
        "type": "string",
        "description": "Funding fee rate in decimals. Only applies to funding trade type. Eg. `0.000065`",
        "example": "0.000065"
      },
      "fees_response": {
        "title": "fees_response",
        "type": "string",
        "description": "Fees paid for the trade. Eg. `12.23`",
        "example": "12.23"
      },
      "is_buy": {
        "title": "is_buy",
        "type": "boolean",
        "description": "True for long order, false for short order.",
        "example": true
      },
      "price__swap_preview_response": {
        "title": "price__swap_preview_response",
        "type": "string",
        "description": "The price Eg. `1982`",
        "example": "1982"
      },
      "quote_amount": {
        "title": "quote_amount",
        "type": "string",
        "description": "Amount of USDC. In 6 decimals fixed number.",
        "example": "1000000"
      },
      "quote_amount_response": {
        "title": "quote_amount_response",
        "type": "string",
        "description": "Amount of USDC. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "quote_balance_response": {
        "title": "quote_balance_response",
        "type": "string",
        "description": "The USDC balance. Eg. `12.23`",
        "example": "12.23"
      },
      "quote_capacity_response": {
        "title": "quote_capacity_response",
        "type": "string",
        "description": "The USDC swap capacity balance. Eg. `12.23`",
        "example": "12.23"
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
    "/swap/preview": {
      "post": {
        "summary": "POST /swap/preview",
        "tags": [
          "Public API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "PostSwapPreview",
        "description": "Previews a collateral swap",
        "responses": {
          "200": {
            "description": "Swap details.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostSwapPreview200Response",
                  "type": "object",
                  "properties": {
                    "quote_amount": {
                      "$ref": "#/components/schemas/quote_amount_response"
                    },
                    "fees": {
                      "$ref": "#/components/schemas/fees_response"
                    },
                    "fee_rate": {
                      "$ref": "#/components/schemas/fee_rate_response"
                    },
                    "base_balance": {
                      "$ref": "#/components/schemas/base_balance_response"
                    },
                    "quote_balance": {
                      "$ref": "#/components/schemas/quote_balance_response"
                    },
                    "base_capacity": {
                      "$ref": "#/components/schemas/base_capacity_response"
                    },
                    "quote_capacity": {
                      "$ref": "#/components/schemas/quote_capacity_response"
                    },
                    "amount": {
                      "$ref": "#/components/schemas/amount__swap_preview_response"
                    },
                    "price": {
                      "$ref": "#/components/schemas/price__swap_preview_response"
                    }
                  },
                  "required": [
                    "quote_amount",
                    "fees",
                    "fee_rate",
                    "base_balance",
                    "quote_balance",
                    "base_capacity",
                    "quote_capacity",
                    "amount",
                    "price"
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
                "title": "PostSwapPreviewPayload",
                "type": "object",
                "properties": {
                  "collateral_asset": {
                    "$ref": "#/components/schemas/collateral_asset"
                  },
                  "is_buy": {
                    "$ref": "#/components/schemas/is_buy"
                  },
                  "base_amount": {
                    "$ref": "#/components/schemas/base_amount"
                  },
                  "quote_amount": {
                    "$ref": "#/components/schemas/quote_amount"
                  }
                },
                "required": [
                  "collateral_asset"
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```