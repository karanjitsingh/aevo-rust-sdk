> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# DELETE /orders-all

Cancel all orders. Optionally, you can specify an asset and instrument type to cancel only orders for that asset/instrument type.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "asset": {
        "title": "asset",
        "type": "string",
        "description": "Name of underlying asset.",
        "example": "ETH"
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
      "instrument_type": {
        "title": "instrument_type",
        "type": "string",
        "description": "Type of instrument.",
        "enum": [
          "OPTION",
          "PERPETUAL",
          "SPOT"
        ]
      },
      "order_id_response": {
        "title": "order_id_response",
        "type": "string",
        "description": "Order ID is the hash of the order payload Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
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
    "/orders-all": {
      "delete": {
        "summary": "DELETE /orders-all",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "DeleteOrdersAll",
        "description": "Cancel all orders. Optionally, you can specify an asset and instrument type to cancel only orders for that asset/instrument type.",
        "responses": {
          "200": {
            "description": "Order cancellation status. If any orders were successfully cancelled, they will be found in the `order_ids` field.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "DeleteOrdersAll200Response",
                  "type": "object",
                  "properties": {
                    "success": {
                      "$ref": "#/components/schemas/success_response"
                    },
                    "order_ids": {
                      "title": "order_id_array",
                      "type": "array",
                      "description": "Order ID is the hash of the order payload",
                      "example": [
                        "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
                      ],
                      "items": {
                        "$ref": "#/components/schemas/order_id_response"
                      },
                      "required": [
                        "order_id_array"
                      ]
                    }
                  },
                  "required": [
                    "success"
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
                "title": "DeleteOrdersAllPayload",
                "type": "object",
                "properties": {
                  "asset": {
                    "$ref": "#/components/schemas/asset"
                  },
                  "instrument_type": {
                    "$ref": "#/components/schemas/instrument_type"
                  }
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