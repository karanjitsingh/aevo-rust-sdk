> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /strategy/{strategy_address}/portfolio

Returns the overall portfolio details of the strategy

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "StrategyAddressPath": {
        "name": "strategy_address",
        "in": "path",
        "description": "Address of strategy",
        "required": true,
        "schema": {
          "type": "string",
          "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
        }
      }
    },
    "schemas": {
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "available_balance_response": {
        "title": "available_balance_response",
        "type": "string",
        "description": "Available balance. Eg. `12.23`",
        "example": "12.23"
      },
      "balance_response": {
        "title": "balance_response",
        "type": "string",
        "description": "Balance. Eg. `12.23`",
        "example": "12.23"
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
      "collateral_value_response": {
        "title": "collateral_value_response",
        "type": "string",
        "description": "Value of the collateral in USD Eg. `10423`",
        "example": "10423"
      },
      "collateral_yield_bearing_response": {
        "title": "collateral_yield_bearing_response",
        "type": "boolean",
        "description": "If collateral asset is yield bearing Eg. `false`",
        "example": false
      },
      "delta_response": {
        "title": "delta_response",
        "type": "string",
        "description": "Option's Delta. Eg. `0.23`",
        "example": "0.23"
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
      "gamma_response": {
        "title": "gamma_response",
        "type": "string",
        "description": "Option's Gamma. Eg. `0.23`",
        "example": "0.23"
      },
      "margin_value_response": {
        "title": "margin_value_response",
        "type": "string",
        "description": "Margin value of the collateral in USD after collateral factor adjustments Eg. `10423`",
        "example": "10423"
      },
      "pending_withdrawals_response": {
        "title": "pending_withdrawals_response",
        "type": "string",
        "description": "Pending withdrawals for a yield vault or strategy Eg. `12`",
        "example": "12"
      },
      "pnl_response": {
        "title": "pnl_response",
        "type": "string",
        "description": "Profit and loss. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "profit_factor_response": {
        "title": "profit_factor_response",
        "type": "string",
        "description": "Profit Factor. Eg. `1.4`",
        "example": "1.4"
      },
      "realized_pnl_response": {
        "title": "realized_pnl_response",
        "type": "string",
        "description": "Realized profit and loss. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "rho_response": {
        "title": "rho_response",
        "type": "string",
        "description": "Option's Rho. Eg. `0.23`",
        "example": "0.23"
      },
      "sharpe_ratio_response": {
        "title": "sharpe_ratio_response",
        "type": "string",
        "description": "Sharpe ratio. Eg. `12.23`",
        "example": "12.23"
      },
      "theta_response": {
        "title": "theta_response",
        "type": "string",
        "description": "Option's Theta. Eg. `0.23`",
        "example": "0.23"
      },
      "unrealized_pnl_response": {
        "title": "unrealized_pnl_response",
        "type": "string",
        "description": "Unrealized PNL. Eg. `12.23`",
        "example": "12.23"
      },
      "used_response": {
        "title": "used_response",
        "type": "string",
        "description": "Margin used. Eg. `12.23`",
        "example": "12.23"
      },
      "vega_response": {
        "title": "vega_response",
        "type": "string",
        "description": "Option's Vega. Eg. `0.23`",
        "example": "0.23"
      },
      "win_rate_response": {
        "title": "win_rate_response",
        "type": "string",
        "description": "Win rate. Eg. `12.23`",
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
    "/strategy/{strategy_address}/portfolio": {
      "get": {
        "summary": "GET /strategy/{strategy_address}/portfolio",
        "tags": [
          "Public API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetStrategyStrategyAddressPortfolio",
        "description": "Returns the overall portfolio details of the strategy",
        "responses": {
          "200": {
            "description": "Portfolio details.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStrategyStrategyAddressPortfolio200Response",
                  "type": "object",
                  "properties": {
                    "balance": {
                      "$ref": "#/components/schemas/balance_response"
                    },
                    "pnl": {
                      "$ref": "#/components/schemas/pnl_response"
                    },
                    "realized_pnl": {
                      "$ref": "#/components/schemas/realized_pnl_response"
                    },
                    "profit_factor": {
                      "$ref": "#/components/schemas/profit_factor_response"
                    },
                    "win_rate": {
                      "$ref": "#/components/schemas/win_rate_response"
                    },
                    "sharpe_ratio": {
                      "$ref": "#/components/schemas/sharpe_ratio_response"
                    },
                    "greeks": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "delta": {
                            "$ref": "#/components/schemas/delta_response"
                          },
                          "gamma": {
                            "$ref": "#/components/schemas/gamma_response"
                          },
                          "rho": {
                            "$ref": "#/components/schemas/rho_response"
                          },
                          "theta": {
                            "$ref": "#/components/schemas/theta_response"
                          },
                          "vega": {
                            "$ref": "#/components/schemas/vega_response"
                          }
                        },
                        "required": [
                          "delta",
                          "gamma",
                          "rho",
                          "theta",
                          "vega"
                        ]
                      }
                    },
                    "user_margin": {
                      "type": "object",
                      "properties": {
                        "used": {
                          "$ref": "#/components/schemas/used_response"
                        },
                        "balance": {
                          "$ref": "#/components/schemas/balance_response"
                        }
                      },
                      "required": [
                        "used",
                        "balance"
                      ]
                    },
                    "collaterals": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collateral_asset": {
                            "$ref": "#/components/schemas/collateral_asset_response"
                          },
                          "collateral_value": {
                            "$ref": "#/components/schemas/collateral_value_response"
                          },
                          "margin_value": {
                            "$ref": "#/components/schemas/margin_value_response"
                          },
                          "balance": {
                            "$ref": "#/components/schemas/balance_response"
                          },
                          "available_balance": {
                            "$ref": "#/components/schemas/available_balance_response"
                          },
                          "collateral_yield_bearing": {
                            "$ref": "#/components/schemas/collateral_yield_bearing_response"
                          },
                          "pending_withdrawals": {
                            "$ref": "#/components/schemas/pending_withdrawals_response"
                          },
                          "unrealized_pnl": {
                            "$ref": "#/components/schemas/unrealized_pnl_response"
                          }
                        },
                        "required": [
                          "collateral_asset",
                          "collateral_value",
                          "margin_value",
                          "balance",
                          "available_balance",
                          "collateral_yield_bearing",
                          "pending_withdrawals",
                          "unrealized_pnl"
                        ]
                      }
                    }
                  },
                  "required": [
                    "balance",
                    "pnl",
                    "realized_pnl",
                    "profit_factor",
                    "win_rate",
                    "sharpe_ratio"
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
            "$ref": "#/components/parameters/StrategyAddressPath"
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
      "description": "Public APIs do not require authentication",
      "name": "Public API"
    }
  ],
  "x-stoplight": {
    "id": "gl4g3krom0lvw"
  }
}
```