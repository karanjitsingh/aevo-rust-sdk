> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /farm-boost

Returns current farm boost, avg farm boost, and boosted volume

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "aevo_earned_response": {
        "title": "aevo_earned_response",
        "type": "string",
        "description": "Aevo Earned at an epoch Eg. `10`",
        "example": "10"
      },
      "airdropped_farm_boost_level_response": {
        "title": "airdropped_farm_boost_level_response",
        "type": "string",
        "description": "airdropped farm boost level (from 1 to 5) Eg. `4`",
        "example": "4"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "boosted_volume_response": {
        "title": "boosted_volume_response",
        "type": "string",
        "description": "Boosted trading notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "curr_epoch_aevo_earned_response": {
        "title": "curr_epoch_aevo_earned_response",
        "type": "string",
        "description": "Aevo earned from current epoch (may change) Eg. `1234.56`",
        "example": "1234.56"
      },
      "epoch_boosted_volume_response": {
        "title": "epoch_boosted_volume_response",
        "type": "string",
        "description": "Boosted trading notional volume in USD terms for current epoch. Eg. `1234.56`",
        "example": "1234.56"
      },
      "epoch_response": {
        "title": "epoch_response",
        "type": "string",
        "description": "Epoch Eg. `1`",
        "example": "1"
      },
      "epoch_volume_response": {
        "title": "epoch_volume_response",
        "type": "string",
        "description": "Trading notional volume in USD terms for current epoch. Eg. `1234.56`",
        "example": "1234.56"
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
      "farm_boost_avg_response": {
        "title": "farm_boost_avg_response",
        "type": "string",
        "description": "Farming boost average Eg. `12.23`",
        "example": "12.23"
      },
      "farm_boost_response": {
        "title": "farm_boost_response",
        "type": "string",
        "description": "Farming boost Eg. `12.23`",
        "example": "12.23"
      },
      "last_rare_farm_boost_response": {
        "title": "last_rare_farm_boost_response",
        "type": "string",
        "description": "Timestamp of last rare farm boost Eg. `12.23`",
        "example": "12.23"
      },
      "prev_epoch_aevo_earned_response": {
        "title": "prev_epoch_aevo_earned_response",
        "type": "string",
        "description": "Aevo earned from previous epochs Eg. `1234.56`",
        "example": "1234.56"
      },
      "random_farm_boost_eligible_response": {
        "title": "random_farm_boost_eligible_response",
        "type": "string",
        "description": "Eligible for a random farm boost (only if airdropped farm boost or referee) Eg. `true`",
        "example": true
      },
      "staked_amount_response": {
        "title": "staked_amount_response",
        "type": "string",
        "description": "User's staked sAEVO amount Eg. `1`",
        "example": "1"
      },
      "staked_response": {
        "title": "staked_response",
        "type": "string",
        "description": "If user has sAEVO Eg. `true`",
        "example": true
      },
      "trailing_volume_response": {
        "title": "trailing_volume_response",
        "type": "string",
        "description": "Trailing volume in past x days in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "volume_response": {
        "title": "volume_response",
        "type": "string",
        "description": "Volume in USDC terms. Eg. `12.23`",
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
    "/farm-boost": {
      "get": {
        "summary": "GET /farm-boost",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetFarmBoost",
        "description": "Returns current farm boost, avg farm boost, and boosted volume",
        "responses": {
          "200": {
            "description": "Farm Boost",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetFarmBoost200Response",
                  "type": "object",
                  "properties": {
                    "farm_boost": {
                      "$ref": "#/components/schemas/farm_boost_response"
                    },
                    "farm_boost_avg": {
                      "$ref": "#/components/schemas/farm_boost_avg_response"
                    },
                    "last_rare_farm_boost": {
                      "$ref": "#/components/schemas/last_rare_farm_boost_response"
                    },
                    "epoch_volume": {
                      "$ref": "#/components/schemas/epoch_volume_response"
                    },
                    "epoch_boosted_volume": {
                      "$ref": "#/components/schemas/epoch_boosted_volume_response"
                    },
                    "boosted_volume": {
                      "$ref": "#/components/schemas/boosted_volume_response"
                    },
                    "trailing_volume": {
                      "$ref": "#/components/schemas/trailing_volume_response"
                    },
                    "prev_epoch_aevo_earned": {
                      "$ref": "#/components/schemas/prev_epoch_aevo_earned_response"
                    },
                    "curr_epoch_aevo_earned": {
                      "$ref": "#/components/schemas/curr_epoch_aevo_earned_response"
                    },
                    "staked": {
                      "$ref": "#/components/schemas/staked_response"
                    },
                    "airdropped_farm_boost_level": {
                      "$ref": "#/components/schemas/airdropped_farm_boost_level_response"
                    },
                    "random_farm_boost_eligible": {
                      "$ref": "#/components/schemas/random_farm_boost_eligible_response"
                    },
                    "staked_amount": {
                      "$ref": "#/components/schemas/staked_amount_response"
                    },
                    "aevo_earned_per_epoch": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "epoch": {
                            "$ref": "#/components/schemas/epoch_response"
                          },
                          "aevo_earned": {
                            "$ref": "#/components/schemas/aevo_earned_response"
                          }
                        },
                        "required": [
                          "epoch",
                          "aevo_earned"
                        ]
                      }
                    },
                    "account_prelaunch_volumes": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "volume": {
                            "$ref": "#/components/schemas/volume_response"
                          }
                        },
                        "required": [
                          "asset",
                          "volume"
                        ]
                      }
                    },
                    "prelaunch_volumes": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "volume": {
                            "$ref": "#/components/schemas/volume_response"
                          }
                        },
                        "required": [
                          "asset",
                          "volume"
                        ]
                      }
                    }
                  },
                  "required": [
                    "farm_boost",
                    "farm_boost_avg",
                    "last_rare_farm_boost",
                    "epoch_volume",
                    "epoch_boosted_volume",
                    "boosted_volume",
                    "trailing_volume",
                    "prev_epoch_aevo_earned",
                    "curr_epoch_aevo_earned",
                    "staked",
                    "airdropped_farm_boost_level",
                    "random_farm_boost_eligible",
                    "staked_amount"
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