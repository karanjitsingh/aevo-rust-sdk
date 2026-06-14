> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /trade-fees-campaign-stats

Return the account's trading fee statistics.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "account_response": {
        "title": "account_response",
        "type": "string",
        "description": "Account's Ethereum address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
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
      "last_claim_seq_response": {
        "title": "last_claim_seq_response",
        "type": "string",
        "description": "Last claim sequence number Eg. `1323.45`",
        "example": "1323.45"
      },
      "total_fees_response": {
        "title": "total_fees_response",
        "type": "string",
        "description": "Total trading fees Eg. `1323.45`",
        "example": "1323.45"
      },
      "total_volume_response": {
        "title": "total_volume_response",
        "type": "string",
        "description": "Total traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "trade_count_response": {
        "title": "trade_count_response",
        "type": "string",
        "description": "Total trade count Eg. `1323.45`",
        "example": "1323.45"
      },
      "week_end_response": {
        "title": "week_end_response",
        "type": "string",
        "description": "End of the week Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "week_start_response": {
        "title": "week_start_response",
        "type": "string",
        "description": "Start of the week Eg. `1680249600000000000`",
        "example": "1680249600000000000"
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
    "/trade-fees-campaign-stats": {
      "get": {
        "summary": "GET /trade-fees-campaign-stats",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetTradeFeesCampaignStats",
        "description": "Return the account's trading fee statistics.",
        "responses": {
          "200": {
            "description": "Trading Fee statistics.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetTradeFeesCampaignStats200Response",
                  "type": "object",
                  "properties": {
                    "account_trade_fees_stats": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "week_start": {
                            "$ref": "#/components/schemas/week_start_response"
                          },
                          "week_end": {
                            "$ref": "#/components/schemas/week_end_response"
                          },
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "total_fees": {
                            "$ref": "#/components/schemas/total_fees_response"
                          },
                          "total_volume": {
                            "$ref": "#/components/schemas/total_volume_response"
                          },
                          "last_claim_seq": {
                            "$ref": "#/components/schemas/last_claim_seq_response"
                          },
                          "trade_count": {
                            "$ref": "#/components/schemas/trade_count_response"
                          }
                        },
                        "required": [
                          "week_start",
                          "week_end",
                          "account",
                          "total_fees",
                          "total_volume",
                          "last_claim_seq",
                          "trade_count"
                        ]
                      }
                    }
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