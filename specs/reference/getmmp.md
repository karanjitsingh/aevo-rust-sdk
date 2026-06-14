> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /mmp

Get market maker protection (MMP) setting.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AssetQuery": {
        "name": "asset",
        "in": "query",
        "description": "Name of underlying asset.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "ETH"
        }
      }
    },
    "schemas": {
      "amount_change_response": {
        "title": "amount_change_response",
        "type": "string",
        "description": "Change in filled contract amounts within the interval. Eg. `0.1`",
        "example": "0.1"
      },
      "amount_limit_response": {
        "title": "amount_limit_response",
        "type": "string",
        "description": "Amount limit setting for market maker protection (MMP). In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "delta_change_response": {
        "title": "delta_change_response",
        "type": "string",
        "description": "Change in delta within the interval. Eg. `0.1`",
        "example": "0.1"
      },
      "delta_limit_response": {
        "title": "delta_limit_response",
        "type": "string",
        "description": "Delta limit setting for market maker protection (MMP). In unsigned float. Eg. `0.1`",
        "example": "0.1"
      },
      "enabled_response": {
        "title": "enabled_response",
        "type": "boolean",
        "description": "Enabled if true. Eg. `true`",
        "example": true
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
      "frozen_end_time_response": {
        "title": "frozen_end_time_response",
        "type": "string",
        "description": "End time in nanoseconds where the account will be unfrozen. Eg. `1683697928273737203`",
        "example": "1683697928273737203"
      },
      "frozen_response": {
        "title": "frozen_response",
        "type": "string",
        "description": "Duration in seconds during which the account will be frozen. If set to 0, manual reset is required. Eg. `30`",
        "example": "30"
      },
      "interval_response": {
        "title": "interval_response",
        "type": "string",
        "description": "Interval in seconds. A setting for market maker protection (MMP). If set to 0, MMP is disabled. Eg. `30`",
        "example": "30"
      },
      "triggered_response": {
        "title": "triggered_response",
        "type": "boolean",
        "description": "Account under MMP if true Eg. `true`",
        "example": true
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
    "/mmp": {
      "get": {
        "summary": "GET /mmp",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetMmp",
        "description": "Get market maker protection (MMP) setting.",
        "responses": {
          "200": {
            "description": "Market maker protection (MMP) setting.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetMmp200Response",
                  "type": "object",
                  "properties": {
                    "enabled": {
                      "$ref": "#/components/schemas/enabled_response"
                    },
                    "triggered": {
                      "$ref": "#/components/schemas/triggered_response"
                    },
                    "interval": {
                      "$ref": "#/components/schemas/interval_response"
                    },
                    "frozen": {
                      "$ref": "#/components/schemas/frozen_response"
                    },
                    "frozen_end_time": {
                      "$ref": "#/components/schemas/frozen_end_time_response"
                    },
                    "amount_limit": {
                      "$ref": "#/components/schemas/amount_limit_response"
                    },
                    "delta_limit": {
                      "$ref": "#/components/schemas/delta_limit_response"
                    },
                    "amount_change": {
                      "$ref": "#/components/schemas/amount_change_response"
                    },
                    "delta_change": {
                      "$ref": "#/components/schemas/delta_change_response"
                    },
                    "asset": {
                      "$ref": "#/components/schemas/asset_response"
                    }
                  },
                  "required": [
                    "enabled",
                    "triggered",
                    "asset"
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
            "$ref": "#/components/parameters/AssetQuery"
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