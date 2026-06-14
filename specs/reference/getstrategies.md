> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /strategies

Returns the map of strategies.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "aum_response": {
        "title": "aum_response",
        "type": "string",
        "description": "AUM of a strategy Eg. `12`",
        "example": "12"
      },
      "cap_response": {
        "title": "cap_response",
        "type": "string",
        "description": "Capacity of a strategy Eg. `12`",
        "example": "12"
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
      "management_fees_response": {
        "title": "management_fees_response",
        "type": "string",
        "description": "Annualized management fee of a yield vault or strategy. Charged regardless of performance. Eg. `0.2`",
        "example": "0.2"
      },
      "max_drawdown_response": {
        "title": "max_drawdown_response",
        "type": "string",
        "description": "Maximum drawdown of a yield vault or strategy YTD Eg. `0.2`",
        "example": "0.2"
      },
      "minimum_deposit_response": {
        "title": "minimum_deposit_response",
        "type": "string",
        "description": "Minimum deposit into a strategy in the strategy collateral currency Eg. `12.23`",
        "example": "12.23"
      },
      "past_month_return_response": {
        "title": "past_month_return_response",
        "type": "string",
        "description": "Past month returns for a strategy Eg. `12.23`",
        "example": "12.23"
      },
      "performance_fees_response": {
        "title": "performance_fees_response",
        "type": "string",
        "description": "Annualized performance fee of a yield vault or strategy. Charged only on profits. Eg. `0.2`",
        "example": "0.2"
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
    "/strategies": {
      "get": {
        "summary": "GET /strategies",
        "tags": [
          "Public API"
        ],
        "operationId": "GetStrategies",
        "description": "Returns the map of strategies.",
        "responses": {
          "200": {
            "description": "Map of strategies.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStrategies200Response",
                  "type": "object",
                  "properties": {
                    "strategies": {
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
                          "aum": {
                            "$ref": "#/components/schemas/aum_response"
                          },
                          "past_month_return": {
                            "$ref": "#/components/schemas/past_month_return_response"
                          },
                          "management_fees": {
                            "$ref": "#/components/schemas/management_fees_response"
                          },
                          "performance_fees": {
                            "$ref": "#/components/schemas/performance_fees_response"
                          },
                          "max_drawdown": {
                            "$ref": "#/components/schemas/max_drawdown_response"
                          },
                          "cap": {
                            "$ref": "#/components/schemas/cap_response"
                          },
                          "minimum_deposit": {
                            "$ref": "#/components/schemas/minimum_deposit_response"
                          }
                        },
                        "required": [
                          "strategy_address",
                          "strategy_name",
                          "collateral_asset",
                          "aum",
                          "past_month_return",
                          "management_fees",
                          "performance_fees",
                          "max_drawdown",
                          "cap",
                          "minimum_deposit"
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```