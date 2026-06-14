> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# DELETE /quotes

Cancel multiple quotes.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "BlockIdQueryOptional": {
        "name": "block_id",
        "in": "query",
        "description": "Block ID is the unique identifier of the block",
        "required": false,
        "schema": {
          "type": "string",
          "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
        }
      },
      "QuoteIdsQueryOptional": {
        "name": "quote_ids",
        "in": "query",
        "description": "List of quote IDs.",
        "required": false,
        "schema": {
          "type": "array",
          "example": [
            "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8",
            "0x850c5569fe9f289c923188ad64b62a329b5f074fc84b404a2577b6bac2a592b4"
          ],
          "items": {
            "type": "string"
          }
        }
      }
    },
    "schemas": {
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
      "quote_id_response": {
        "title": "quote_id_response",
        "type": "string",
        "description": "Quote ID is the hash of the quote payload Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`",
        "example": "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
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
    "/quotes": {
      "delete": {
        "summary": "DELETE /quotes",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "DeleteQuotes",
        "description": "Cancel multiple quotes.",
        "responses": {
          "200": {
            "description": "Quote cancellation status. If any quote were successfully cancelled, they will be found in the `cancelled` field.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "DeleteQuotes200Response",
                  "type": "object",
                  "properties": {
                    "cancelled": {
                      "title": "quote_id_array",
                      "type": "array",
                      "description": "Quote ID is the hash of the quote payload",
                      "example": [
                        "0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8"
                      ],
                      "items": {
                        "$ref": "#/components/schemas/quote_id_response"
                      },
                      "required": [
                        "quote_id_array"
                      ]
                    }
                  }
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
            "$ref": "#/components/parameters/QuoteIdsQueryOptional"
          },
          {
            "$ref": "#/components/parameters/BlockIdQueryOptional"
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