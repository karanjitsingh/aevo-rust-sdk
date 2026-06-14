> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /referral-statistics

Return the account's referral statistics.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "StartTimeQueryOptional": {
        "name": "start_time",
        "in": "query",
        "description": "Entries created prior (<) to start time are excluded in UNIX timestamp in nanoseconds. Defaults to `0`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 1672531200000000000
        }
      }
    },
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
      "referred_response": {
        "title": "referred_response",
        "type": "string",
        "description": "Number of users Referrer has referred Eg. `4`",
        "example": "4"
      },
      "total_referee_discount_response": {
        "title": "total_referee_discount_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referee Eg. `69.2`",
        "example": "69.2"
      },
      "total_referee_discount_unclaimed_response": {
        "title": "total_referee_discount_unclaimed_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referee that has not been claimed Eg. `69.2`",
        "example": "69.2"
      },
      "total_referral_bonus_response": {
        "title": "total_referral_bonus_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referrer Eg. `69.2`",
        "example": "69.2"
      },
      "total_referral_bonus_unclaimed_response": {
        "title": "total_referral_bonus_unclaimed_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referrer that has not been claimed Eg. `69.2`",
        "example": "69.2"
      },
      "total_rewards_response": {
        "title": "total_rewards_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referrer or referee Eg. `69.2`",
        "example": "69.2"
      },
      "total_rewards_unclaimed_response": {
        "title": "total_rewards_unclaimed_response",
        "type": "string",
        "description": "Total amount of rewards earned by an account for being a referrer or referee that has not been claimed Eg. `69.2`",
        "example": "69.2"
      },
      "total_trading_fees_rewards_response": {
        "title": "total_trading_fees_rewards_response",
        "type": "string",
        "description": "Total amount of trading fees rewards earned by an account Eg. `69.2`",
        "example": "69.2"
      },
      "total_trading_fees_rewards_volume_response": {
        "title": "total_trading_fees_rewards_volume_response",
        "type": "string",
        "description": "Total amount of trading fees rewards volume earned by an account Eg. `69.2`",
        "example": "69.2"
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
    "/referral-statistics": {
      "get": {
        "summary": "GET /referral-statistics",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetReferralStatistics",
        "description": "Return the account's referral statistics.",
        "responses": {
          "200": {
            "description": "Referral statistics.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetReferralStatistics200Response",
                  "type": "object",
                  "properties": {
                    "referred": {
                      "$ref": "#/components/schemas/referred_response"
                    },
                    "volume": {
                      "$ref": "#/components/schemas/volume_response"
                    },
                    "total_rewards": {
                      "$ref": "#/components/schemas/total_rewards_response"
                    },
                    "total_referral_bonus": {
                      "$ref": "#/components/schemas/total_referral_bonus_response"
                    },
                    "total_referee_discount": {
                      "$ref": "#/components/schemas/total_referee_discount_response"
                    },
                    "total_rewards_unclaimed": {
                      "$ref": "#/components/schemas/total_rewards_unclaimed_response"
                    },
                    "total_referral_bonus_unclaimed": {
                      "$ref": "#/components/schemas/total_referral_bonus_unclaimed_response"
                    },
                    "total_referee_discount_unclaimed": {
                      "$ref": "#/components/schemas/total_referee_discount_unclaimed_response"
                    },
                    "total_trading_fees_rewards": {
                      "$ref": "#/components/schemas/total_trading_fees_rewards_response"
                    },
                    "total_trading_fees_rewards_volume": {
                      "$ref": "#/components/schemas/total_trading_fees_rewards_volume_response"
                    }
                  },
                  "required": [
                    "referred",
                    "volume",
                    "total_rewards",
                    "total_referral_bonus",
                    "total_referee_discount",
                    "total_rewards_unclaimed",
                    "total_referral_bonus_unclaimed",
                    "total_referee_discount_unclaimed",
                    "total_trading_fees_rewards",
                    "total_trading_fees_rewards_volume"
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
        "parameters": [
          {
            "$ref": "#/components/parameters/StartTimeQueryOptional"
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