> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /strategies/account

Returns the account's strategy positions

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "amount_float_response": {
        "title": "amount_float_response",
        "type": "string",
        "description": "Amount in unsigned float string Eg. `12.23`",
        "example": "12.23"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "collateral_asset_response": {
        "title": "collateral_asset_response",
        "type": "string",
        "description": "Name of the collateral asset. Eg. `USDT`",
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
      "epoch_response": {
        "title": "epoch_response",
        "type": "string",
        "description": "Epoch Eg. `1`",
        "example": "1"
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
      "pending_deposits_response": {
        "title": "pending_deposits_response",
        "type": "string",
        "description": "Pending deposits for a yield vault or strategy Eg. `12`",
        "example": "12"
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
      },
      "roi_response": {
        "title": "roi_response",
        "type": "string",
        "description": "ROI Eg. `12.23`",
        "example": "12.23"
      },
      "shares_response": {
        "title": "shares_response",
        "type": "string",
        "description": "Amount in shares for a strategy Eg. `12.23`",
        "example": "12.23"
      },
      "strategy_address_response": {
        "title": "strategy_address_response",
        "type": "string",
        "description": "Address of strategy Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "strategy_name_response": {
        "title": "strategy_name_response",
        "type": "string",
        "description": "Name of strategy Eg. `Covered Calls`",
        "example": "Covered Calls"
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
    "/strategies/account": {
      "get": {
        "summary": "GET /strategies/account",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetStrategiesAccount",
        "description": "Returns the account's strategy positions",
        "responses": {
          "200": {
            "description": "Map of strategy positions",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStrategiesAccount200Response",
                  "type": "object",
                  "properties": {
                    "strategy_positions": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "strategy_address": {
                            "$ref": "#/components/schemas/strategy_address_response"
                          },
                          "strategy_name": {
                            "$ref": "#/components/schemas/strategy_name_response"
                          },
                          "collateral_asset": {
                            "$ref": "#/components/schemas/collateral_asset_response"
                          },
                          "pending_deposits": {
                            "$ref": "#/components/schemas/pending_deposits_response"
                          },
                          "pending_withdrawals": {
                            "$ref": "#/components/schemas/pending_withdrawals_response"
                          },
                          "shares": {
                            "$ref": "#/components/schemas/shares_response"
                          },
                          "pps": {
                            "$ref": "#/components/schemas/pps_response"
                          },
                          "roi": {
                            "$ref": "#/components/schemas/roi_response"
                          }
                        },
                        "required": [
                          "strategy_address",
                          "strategy_name",
                          "collateral_asset",
                          "pending_deposits",
                          "pending_withdrawals",
                          "shares",
                          "pps",
                          "roi"
                        ]
                      }
                    },
                    "strategy_rewards": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "strategy_address": {
                            "$ref": "#/components/schemas/strategy_address_response"
                          },
                          "epoch_rewards": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "epoch": {
                                  "$ref": "#/components/schemas/epoch_response"
                                },
                                "epoch_reward": {
                                  "type": "object",
                                  "properties": {
                                    "asset_rewards": {
                                      "type": "array",
                                      "items": {
                                        "type": "object",
                                        "properties": {
                                          "asset": {
                                            "$ref": "#/components/schemas/asset_response"
                                          },
                                          "amount_float": {
                                            "$ref": "#/components/schemas/amount_float_response"
                                          }
                                        },
                                        "required": [
                                          "asset",
                                          "amount_float"
                                        ]
                                      }
                                    }
                                  }
                                }
                              },
                              "required": [
                                "epoch"
                              ]
                            }
                          }
                        },
                        "required": [
                          "strategy_address"
                        ]
                      }
                    }
                  }
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