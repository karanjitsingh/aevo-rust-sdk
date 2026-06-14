> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /stake

Returns the account's weekly trading volume, multiplier tier, stake history, LP NFT history, and top staker APR.

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
      "apr_response": {
        "title": "apr_response",
        "type": "string",
        "description": "Annual percentage rate for the top staker based on last week's data Eg. `125.5`",
        "example": "125.5"
      },
      "end_epoch_response": {
        "title": "end_epoch_response",
        "type": "string",
        "description": "End epoch for LP NFT Eg. `5`",
        "example": "5"
      },
      "epoch_response": {
        "title": "epoch_response",
        "type": "string",
        "description": "Epoch Eg. `1`",
        "example": "1"
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
      "id_response": {
        "title": "id_response",
        "type": "string",
        "description": "LP NFT token ID Eg. `100`",
        "example": "100"
      },
      "multiplier_response": {
        "title": "multiplier_response",
        "type": "string",
        "description": "Volume multiplier for LP NFT distribution Eg. `2`",
        "example": "2"
      },
      "reward_amount_response": {
        "title": "reward_amount_response",
        "type": "string",
        "description": "Reward amount earned for the epoch Eg. `1000`",
        "example": "1000"
      },
      "send_date_response": {
        "title": "send_date_response",
        "type": "string",
        "description": "Date when LP NFT was sent Eg. `2024-01-15`",
        "example": "2024-01-15"
      },
      "start_epoch_response": {
        "title": "start_epoch_response",
        "type": "string",
        "description": "Start epoch for LP NFT Eg. `1`",
        "example": "1"
      },
      "tier_response": {
        "title": "tier_response",
        "type": "string",
        "description": "Volume tier for LP NFT distribution (1-4) Eg. `2`",
        "example": "2"
      },
      "total_volume_response": {
        "title": "total_volume_response",
        "type": "string",
        "description": "Total traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
      },
      "vesting_weeks_response": {
        "title": "vesting_weeks_response",
        "type": "string",
        "description": "Number of vesting weeks for LP NFT Eg. `4`",
        "example": "4"
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
      },
      "weekly_volume_response": {
        "title": "weekly_volume_response",
        "type": "string",
        "description": "Weekly trading volume in USD Eg. `1234567.89`",
        "example": "1234567.89"
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
    "/stake": {
      "get": {
        "summary": "GET /stake",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetStake",
        "description": "Returns the account's weekly trading volume, multiplier tier, stake history, LP NFT history, and top staker APR.",
        "responses": {
          "200": {
            "description": "Weekly volume, multiplier tier, stake history, LP NFT history, and top staker APR.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetStake200Response",
                  "type": "object",
                  "properties": {
                    "weekly_volume": {
                      "$ref": "#/components/schemas/weekly_volume_response"
                    },
                    "multiplier": {
                      "$ref": "#/components/schemas/multiplier_response"
                    },
                    "tier": {
                      "$ref": "#/components/schemas/tier_response"
                    },
                    "week_start": {
                      "$ref": "#/components/schemas/week_start_response"
                    },
                    "week_end": {
                      "$ref": "#/components/schemas/week_end_response"
                    },
                    "stake_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "epoch": {
                            "$ref": "#/components/schemas/epoch_response"
                          },
                          "reward_amount": {
                            "$ref": "#/components/schemas/reward_amount_response"
                          }
                        },
                        "required": [
                          "account",
                          "epoch",
                          "reward_amount"
                        ]
                      }
                    },
                    "lpnft_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "start_epoch": {
                            "$ref": "#/components/schemas/start_epoch_response"
                          },
                          "end_epoch": {
                            "$ref": "#/components/schemas/end_epoch_response"
                          },
                          "vesting_weeks": {
                            "$ref": "#/components/schemas/vesting_weeks_response"
                          },
                          "send_date": {
                            "$ref": "#/components/schemas/send_date_response"
                          },
                          "id": {
                            "$ref": "#/components/schemas/id_response"
                          }
                        },
                        "required": [
                          "account",
                          "start_epoch",
                          "end_epoch",
                          "vesting_weeks",
                          "send_date",
                          "id"
                        ]
                      }
                    },
                    "apr": {
                      "$ref": "#/components/schemas/apr_response"
                    },
                    "total_volume": {
                      "$ref": "#/components/schemas/total_volume_response"
                    }
                  },
                  "required": [
                    "weekly_volume",
                    "multiplier",
                    "tier",
                    "week_start",
                    "week_end",
                    "apr",
                    "total_volume"
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