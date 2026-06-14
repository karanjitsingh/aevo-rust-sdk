> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /leaderboard/campaign

Returns the leaderboard for a campaign by pnl in descending order

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "EndTimeQuery": {
        "name": "end_time",
        "in": "query",
        "description": "Entries created after (>) end time are excluded in UNIX timestamp in nanoseconds. Defaults to current time.",
        "required": true,
        "schema": {
          "type": "integer",
          "example": 1675036800000000000
        }
      },
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
      "StartTimeQuery": {
        "name": "start_time",
        "in": "query",
        "description": "Entries created prior (<) to start time are excluded in UNIX timestamp in nanoseconds. Defaults to `0`",
        "required": true,
        "schema": {
          "type": "integer",
          "example": 1672531200000000000
        }
      },
      "UsernameQuery": {
        "name": "username",
        "in": "query",
        "description": "Account's auto-generated username based on their Ethereum address.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "officially-evolved-terrier"
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
      "options_volume_response": {
        "title": "options_volume_response",
        "type": "string",
        "description": "Options Volume in USDC terms. Eg. `12.23`",
        "example": "12.23"
      },
      "perp_volume_response": {
        "title": "perp_volume_response",
        "type": "string",
        "description": "Perpetuals Volume in USDC terms. Eg. `12.23`",
        "example": "12.23"
      },
      "pnl_response": {
        "title": "pnl_response",
        "type": "string",
        "description": "Profit and loss. Signed float string. Eg. `12.23`",
        "example": "12.23"
      },
      "ranking_response": {
        "title": "ranking_response",
        "type": "string",
        "description": "Ranking. Eg. ``",
        "example": ""
      },
      "username_response": {
        "title": "username_response",
        "type": "string",
        "description": "Account's auto-generated username based on their Ethereum address. Eg. `officially-evolved-terrier`",
        "example": "officially-evolved-terrier"
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
    "/leaderboard/campaign": {
      "get": {
        "summary": "GET /leaderboard/campaign",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetLeaderboardCampaign",
        "description": "Returns the leaderboard for a campaign by pnl in descending order",
        "responses": {
          "200": {
            "description": "Campaign Leaderboard.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetLeaderboardCampaign200Response",
                  "type": "object",
                  "properties": {
                    "leaderboard": {
                      "type": "object",
                      "properties": {
                        "daily": {
                          "type": "object",
                          "properties": {
                            "ranking": {
                              "$ref": "#/components/schemas/ranking_response"
                            },
                            "options_volume": {
                              "$ref": "#/components/schemas/options_volume_response"
                            },
                            "perp_volume": {
                              "$ref": "#/components/schemas/perp_volume_response"
                            },
                            "pnl": {
                              "$ref": "#/components/schemas/pnl_response"
                            },
                            "username": {
                              "$ref": "#/components/schemas/username_response"
                            }
                          },
                          "required": [
                            "ranking",
                            "options_volume",
                            "perp_volume",
                            "pnl",
                            "username"
                          ]
                        },
                        "weekly": {
                          "type": "object",
                          "properties": {
                            "ranking": {
                              "$ref": "#/components/schemas/ranking_response"
                            },
                            "options_volume": {
                              "$ref": "#/components/schemas/options_volume_response"
                            },
                            "perp_volume": {
                              "$ref": "#/components/schemas/perp_volume_response"
                            },
                            "pnl": {
                              "$ref": "#/components/schemas/pnl_response"
                            },
                            "username": {
                              "$ref": "#/components/schemas/username_response"
                            }
                          },
                          "required": [
                            "ranking",
                            "options_volume",
                            "perp_volume",
                            "pnl",
                            "username"
                          ]
                        },
                        "monthly": {
                          "type": "object",
                          "properties": {
                            "ranking": {
                              "$ref": "#/components/schemas/ranking_response"
                            },
                            "options_volume": {
                              "$ref": "#/components/schemas/options_volume_response"
                            },
                            "perp_volume": {
                              "$ref": "#/components/schemas/perp_volume_response"
                            },
                            "pnl": {
                              "$ref": "#/components/schemas/pnl_response"
                            },
                            "username": {
                              "$ref": "#/components/schemas/username_response"
                            }
                          },
                          "required": [
                            "ranking",
                            "options_volume",
                            "perp_volume",
                            "pnl",
                            "username"
                          ]
                        },
                        "all_time": {
                          "type": "object",
                          "properties": {
                            "ranking": {
                              "$ref": "#/components/schemas/ranking_response"
                            },
                            "options_volume": {
                              "$ref": "#/components/schemas/options_volume_response"
                            },
                            "perp_volume": {
                              "$ref": "#/components/schemas/perp_volume_response"
                            },
                            "pnl": {
                              "$ref": "#/components/schemas/pnl_response"
                            },
                            "username": {
                              "$ref": "#/components/schemas/username_response"
                            }
                          },
                          "required": [
                            "ranking",
                            "options_volume",
                            "perp_volume",
                            "pnl",
                            "username"
                          ]
                        }
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
        },
        "parameters": [
          {
            "$ref": "#/components/parameters/UsernameQuery"
          },
          {
            "$ref": "#/components/parameters/LimitQueryOptional"
          },
          {
            "$ref": "#/components/parameters/StartTimeQuery"
          },
          {
            "$ref": "#/components/parameters/EndTimeQuery"
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