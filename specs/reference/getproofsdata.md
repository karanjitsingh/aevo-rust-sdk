> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /proofs-data

Returns account's proof data

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AssetsQuery": {
        "name": "assets",
        "in": "query",
        "description": "Array of assets",
        "required": true,
        "schema": {
          "type": "array",
          "example": [
            "ETH",
            "BTC"
          ],
          "items": {
            "type": "string"
          }
        }
      }
    },
    "schemas": {
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
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
      "index_response": {
        "title": "index_response",
        "type": "string",
        "description": "Index of proof. Eg. `0`",
        "example": "0"
      },
      "merkle_root_response": {
        "title": "merkle_root_response",
        "type": "string",
        "description": "Merkle Root Eg. `0x0`",
        "example": "0x0"
      },
      "proof_response": {
        "title": "proof_response",
        "type": "string",
        "description": "Proof. Eg. `0x0`",
        "example": "0x0"
      },
      "token_total_response": {
        "title": "token_total_response",
        "type": "string",
        "description": "Total Tokens Eg. `0x0`",
        "example": "0x0"
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
    "/proofs-data": {
      "get": {
        "summary": "GET /proofs-data",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetProofsData",
        "description": "Returns account's proof data",
        "responses": {
          "200": {
            "description": "Proofs Data",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetProofsData200Response",
                  "type": "object",
                  "properties": {
                    "merkle_datas": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "merkle_root": {
                            "$ref": "#/components/schemas/merkle_root_response"
                          },
                          "token_total": {
                            "$ref": "#/components/schemas/token_total_response"
                          },
                          "claim": {
                            "type": "object",
                            "properties": {
                              "index": {
                                "$ref": "#/components/schemas/index_response"
                              },
                              "amount": {
                                "$ref": "#/components/schemas/amount_response"
                              },
                              "proof": {
                                "title": "proof_array",
                                "type": "array",
                                "description": "Proof.",
                                "example": [
                                  "0x0"
                                ],
                                "items": {
                                  "$ref": "#/components/schemas/proof_response"
                                },
                                "required": [
                                  "proof_array"
                                ]
                              }
                            },
                            "required": [
                              "index",
                              "amount"
                            ]
                          }
                        },
                        "required": [
                          "merkle_root",
                          "token_total"
                        ]
                      }
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
            "$ref": "#/components/parameters/AssetsQuery"
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