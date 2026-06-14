> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /otc/account

Returns the account's otc details

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
      "amount_response": {
        "title": "amount_response",
        "type": "string",
        "description": "Number of contracts. In 6 decimals fixed number. Eg. `1000000`",
        "example": "1000000"
      },
      "asset_response": {
        "title": "asset_response",
        "type": "string",
        "description": "Name of underlying asset. Eg. `ETH`",
        "example": "ETH"
      },
      "created_timestamp_response": {
        "title": "created_timestamp_response",
        "type": "string",
        "description": "Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "entry_spot_price_response": {
        "title": "entry_spot_price_response",
        "type": "string",
        "description": "Entry spot price of the otc request Eg. `1000`",
        "example": "1000"
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
      "expiry_response": {
        "title": "expiry_response",
        "type": "string",
        "description": "Option expiry in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "indicative_price_response": {
        "title": "indicative_price_response",
        "type": "string",
        "description": "Indicative price of the otc request Eg. `1000`",
        "example": "1000"
      },
      "internal_account_response": {
        "title": "internal_account_response",
        "type": "string",
        "description": "Internal account for OTC trades Eg. `0x0`",
        "example": "0x0"
      },
      "option_type_response": {
        "title": "option_type_response",
        "type": "string",
        "description": "Type of option contract. Eg. `call`",
        "enum": [
          "put",
          "call"
        ]
      },
      "otc_id_response": {
        "title": "otc_id_response",
        "type": "string",
        "description": "Unique ID of the otc request Eg. `1`",
        "example": "1"
      },
      "premium_response": {
        "title": "premium_response",
        "type": "string",
        "description": "Premiums required for the otc request Eg. `2000000`",
        "example": "2000000"
      },
      "quote_price_response": {
        "title": "quote_price_response",
        "type": "string",
        "description": "Otc quote price Eg. `2000000`",
        "example": "2000000"
      },
      "quote_timestamp_response": {
        "title": "quote_timestamp_response",
        "type": "string",
        "description": "Timestamp of the otc quote Eg. `1692861351000000000`",
        "example": "1692861351000000000"
      },
      "request_timestamp_response": {
        "title": "request_timestamp_response",
        "type": "string",
        "description": "Timestamp of the otc request Eg. `1692861351000000000`",
        "example": "1692861351000000000"
      },
      "status_response": {
        "title": "status_response",
        "type": "string",
        "description": "Status of the otc request Eg. `requested`",
        "example": "requested"
      },
      "strike_response": {
        "title": "strike_response",
        "type": "string",
        "description": "Option strike price. Eg. `2500`",
        "example": "2500"
      },
      "unwind_id_response": {
        "title": "unwind_id_response",
        "type": "string",
        "description": "Otc id of position to unwind. Eg. `12`",
        "example": "12"
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
    "/otc/account": {
      "get": {
        "summary": "GET /otc/account",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetOtcAccount",
        "description": "Returns the account's otc details",
        "responses": {
          "200": {
            "description": "Account otc details.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetOtcAccount200Response",
                  "type": "object",
                  "properties": {
                    "internal_account": {
                      "$ref": "#/components/schemas/internal_account_response"
                    },
                    "otc_requests": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "otc_id": {
                            "$ref": "#/components/schemas/otc_id_response"
                          },
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "strike": {
                            "$ref": "#/components/schemas/strike_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "option_type": {
                            "$ref": "#/components/schemas/option_type_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "request_timestamp": {
                            "$ref": "#/components/schemas/request_timestamp_response"
                          },
                          "indicative_price": {
                            "$ref": "#/components/schemas/indicative_price_response"
                          },
                          "quote_price": {
                            "$ref": "#/components/schemas/quote_price_response"
                          },
                          "status": {
                            "$ref": "#/components/schemas/status_response"
                          },
                          "premium": {
                            "$ref": "#/components/schemas/premium_response"
                          },
                          "quote_timestamp": {
                            "$ref": "#/components/schemas/quote_timestamp_response"
                          },
                          "unwind_id": {
                            "$ref": "#/components/schemas/unwind_id_response"
                          }
                        },
                        "required": [
                          "otc_id",
                          "account",
                          "asset",
                          "strike",
                          "expiry",
                          "option_type",
                          "amount",
                          "request_timestamp",
                          "indicative_price",
                          "quote_price",
                          "status",
                          "premium",
                          "quote_timestamp",
                          "unwind_id"
                        ]
                      }
                    },
                    "otc_positions": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "otc_id": {
                            "$ref": "#/components/schemas/otc_id_response"
                          },
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "asset": {
                            "$ref": "#/components/schemas/asset_response"
                          },
                          "strike": {
                            "$ref": "#/components/schemas/strike_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "option_type": {
                            "$ref": "#/components/schemas/option_type_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "entry_spot_price": {
                            "$ref": "#/components/schemas/entry_spot_price_response"
                          },
                          "premium": {
                            "$ref": "#/components/schemas/premium_response"
                          },
                          "quote_price": {
                            "$ref": "#/components/schemas/quote_price_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "otc_id",
                          "account",
                          "asset",
                          "strike",
                          "expiry",
                          "option_type",
                          "amount",
                          "entry_spot_price",
                          "premium",
                          "quote_price",
                          "created_timestamp"
                        ]
                      }
                    }
                  },
                  "required": [
                    "internal_account"
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