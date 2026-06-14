> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /referral-history

Return the account's referral history.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "LimitQueryOptional": {
        "name": "limit",
        "in": "query",
        "description": "Limits the number of relevant entries in the response. Defaults to `10`. Maximum value is `50`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 10
        }
      },
      "OffsetQueryOptional": {
        "name": "offset",
        "in": "query",
        "description": "Offset.",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 10
        }
      }
    },
    "schemas": {
      "count_response": {
        "title": "count_response",
        "type": "string",
        "description": "total number of rows in a query Eg. `5`",
        "example": "5"
      },
      "created_timestamp_response": {
        "title": "created_timestamp_response",
        "type": "string",
        "description": "Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "daily_volume_response": {
        "title": "daily_volume_response",
        "type": "string",
        "description": "24-hour traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
      "pnl_response": {
        "title": "pnl_response",
        "type": "string",
        "description": "Profit and loss. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "referee_response": {
        "title": "referee_response",
        "type": "string",
        "description": "Referee address of a Referrer Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "referee_username_response": {
        "title": "referee_username_response",
        "type": "string",
        "description": "Username of the referee of a referrer Eg. `Officially-Evolved-Terrier`",
        "example": "Officially-Evolved-Terrier"
      },
      "referrer_username_response": {
        "title": "referrer_username_response",
        "type": "string",
        "description": "Username of the referrer Eg. `Officially-Evolved-Terrier`",
        "example": "Officially-Evolved-Terrier"
      },
      "staking_tier_response": {
        "title": "staking_tier_response",
        "type": "string",
        "description": "Staking tier of an account based on the amount of AEVO staked Eg. `Bronze`",
        "example": "Bronze"
      },
      "thirty_day_active_pair_response": {
        "title": "thirty_day_active_pair_response",
        "type": "string",
        "description": "Most active trading pair from the last 30 days Eg. `ETH`",
        "example": "ETH"
      },
      "thirty_day_active_pair_volume_response": {
        "title": "thirty_day_active_pair_volume_response",
        "type": "string",
        "description": "Volume of the most active trading pair from the last 30 days Eg. `13.53`",
        "example": "13.53"
      },
      "total_volume_response": {
        "title": "total_volume_response",
        "type": "string",
        "description": "Total traded notional volume in USD terms. Eg. `1234.56`",
        "example": "1234.56"
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
    "/referral-history": {
      "get": {
        "summary": "GET /referral-history",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetReferralHistory",
        "description": "Return the account's referral history.",
        "responses": {
          "200": {
            "description": "Referral history.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetReferralHistory200Response",
                  "type": "object",
                  "properties": {
                    "count": {
                      "$ref": "#/components/schemas/count_response"
                    },
                    "referral_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "referee": {
                            "$ref": "#/components/schemas/referee_response"
                          },
                          "referrer_username": {
                            "$ref": "#/components/schemas/referrer_username_response"
                          },
                          "referee_username": {
                            "$ref": "#/components/schemas/referee_username_response"
                          },
                          "total_volume": {
                            "$ref": "#/components/schemas/total_volume_response"
                          },
                          "daily_volume": {
                            "$ref": "#/components/schemas/daily_volume_response"
                          },
                          "pnl": {
                            "$ref": "#/components/schemas/pnl_response"
                          },
                          "thirty_day_active_pair": {
                            "$ref": "#/components/schemas/thirty_day_active_pair_response"
                          },
                          "thirty_day_active_pair_volume": {
                            "$ref": "#/components/schemas/thirty_day_active_pair_volume_response"
                          },
                          "staking_tier": {
                            "$ref": "#/components/schemas/staking_tier_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "referee",
                          "referee_username",
                          "total_volume",
                          "created_timestamp"
                        ]
                      }
                    }
                  },
                  "required": [
                    "count"
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
            "$ref": "#/components/parameters/LimitQueryOptional"
          },
          {
            "$ref": "#/components/parameters/OffsetQueryOptional"
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