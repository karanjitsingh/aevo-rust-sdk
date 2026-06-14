> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /transaction-history

Return the account's deposit and withdraw history.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "EndTimeQueryOptional": {
        "name": "end_time",
        "in": "query",
        "description": "Entries created after (>) end time are excluded in UNIX timestamp in nanoseconds. Defaults to current time.",
        "required": false,
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
      "OffsetQueryOptional": {
        "name": "offset",
        "in": "query",
        "description": "Offset.",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 10
        }
      },
      "StartTimeQueryOptional": {
        "name": "start_time",
        "in": "query",
        "description": "Entries created prior (<) to start time are excluded in UNIX timestamp in nanoseconds. Defaults to `0`",
        "required": false,
        "schema": {
          "type": "integer",
          "example": 1672531200000000000
        }
      },
      "TxStatusQueryOptional": {
        "name": "tx_status",
        "in": "query",
        "description": "Transaction status.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "initiated",
          "enum": [
            "initiated",
            "finalized"
          ]
        }
      },
      "TxTypeQueryOptional": {
        "name": "tx_type",
        "in": "query",
        "description": "Type of user transaction.",
        "required": false,
        "schema": {
          "type": "string",
          "example": "deposit",
          "enum": [
            "deposit",
            "withdraw",
            "send",
            "receive",
            "swap",
            "yv_deposit",
            "yv_withdraw"
          ]
        }
      }
    },
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
      "chain_id_response": {
        "title": "chain_id_response",
        "type": "string",
        "description": "The specified chain ID. Eg. `10`",
        "example": "10"
      },
      "collateral_response": {
        "title": "collateral_response",
        "type": "string",
        "description": "Ethereum address of the collateral asset. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "count_response": {
        "title": "count_response",
        "type": "string",
        "description": "total number of rows in a query Eg. `5`",
        "example": "5"
      },
      "counter_party_response": {
        "title": "counter_party_response",
        "type": "string",
        "description": "Counter party address of a transfer transaction. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "description_response": {
        "title": "description_response",
        "type": "string",
        "description": "Reward description Eg. `Taker Rewards`",
        "example": "Taker Rewards"
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
      "fees_response": {
        "title": "fees_response",
        "type": "string",
        "description": "Fees paid for the trade. Eg. `12.23`",
        "example": "12.23"
      },
      "finalized_timestamp_response": {
        "title": "finalized_timestamp_response",
        "type": "string",
        "description": "Timestamp where transaction is finalized in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "id_response": {
        "title": "id_response",
        "type": "string",
        "description": "LP NFT token ID Eg. `100`",
        "example": "100"
      },
      "initiated_timestamp_response": {
        "title": "initiated_timestamp_response",
        "type": "string",
        "description": "Timestamp where transaction is initiated in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "l1_tx_hash_response": {
        "title": "l1_tx_hash_response",
        "type": "string",
        "description": "L1 Transaction hash. Eg. `0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91`",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "l2_tx_hash_response": {
        "title": "l2_tx_hash_response",
        "type": "string",
        "description": "L2 Transaction hash. Eg. `0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91`",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "label_response": {
        "title": "label_response",
        "type": "string",
        "description": "Transfer label. Eg. `Rewards`",
        "example": "Rewards"
      },
      "link_response": {
        "title": "link_response",
        "type": "string",
        "description": "Link to details for the reward Eg. `https://app.aevo.xyz`",
        "example": "https://app.aevo.xyz"
      },
      "price_response": {
        "title": "price_response",
        "type": "string",
        "description": "Limit price for the order. Eg. `12.34`",
        "example": "12.34"
      },
      "side_response": {
        "title": "side_response",
        "type": "string",
        "description": "Trade side. Eg. `buy`",
        "enum": [
          "buy",
          "sell"
        ]
      },
      "swap_asset_response": {
        "title": "swap_asset_response",
        "type": "string",
        "description": "Base asset of a collateral swap. Eg. `ETH`",
        "example": "ETH"
      },
      "tx_status_response": {
        "title": "tx_status_response",
        "type": "string",
        "description": "Transaction status. Eg. `initiated`",
        "enum": [
          "initiated",
          "finalized"
        ]
      },
      "tx_type_response": {
        "title": "tx_type_response",
        "type": "string",
        "description": "Type of user transaction. Eg. `deposit`",
        "enum": [
          "deposit",
          "withdraw",
          "send",
          "receive",
          "swap",
          "yv_deposit",
          "yv_withdraw"
        ]
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
    "/transaction-history": {
      "get": {
        "summary": "GET /transaction-history",
        "tags": [
          "Private API"
        ],
        "security": [
          {
            "api_key": [],
            "api_secret": []
          }
        ],
        "operationId": "GetTransactionHistory",
        "description": "Return the account's deposit and withdraw history.",
        "responses": {
          "200": {
            "description": "Transaction history",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetTransactionHistory200Response",
                  "type": "object",
                  "properties": {
                    "count": {
                      "$ref": "#/components/schemas/count_response"
                    },
                    "transaction_history": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "id": {
                            "$ref": "#/components/schemas/id_response"
                          },
                          "account": {
                            "$ref": "#/components/schemas/account_response"
                          },
                          "amount": {
                            "$ref": "#/components/schemas/amount_response"
                          },
                          "collateral": {
                            "$ref": "#/components/schemas/collateral_response"
                          },
                          "counter_party": {
                            "$ref": "#/components/schemas/counter_party_response"
                          },
                          "transfer_details": {
                            "type": "object",
                            "properties": {
                              "description": {
                                "$ref": "#/components/schemas/description_response"
                              },
                              "link": {
                                "$ref": "#/components/schemas/link_response"
                              }
                            },
                            "required": [
                              "description",
                              "link"
                            ]
                          },
                          "finalized_timestamp": {
                            "$ref": "#/components/schemas/finalized_timestamp_response"
                          },
                          "initiated_timestamp": {
                            "$ref": "#/components/schemas/initiated_timestamp_response"
                          },
                          "l1_tx_hash": {
                            "$ref": "#/components/schemas/l1_tx_hash_response"
                          },
                          "l2_tx_hash": {
                            "$ref": "#/components/schemas/l2_tx_hash_response"
                          },
                          "chain_id": {
                            "$ref": "#/components/schemas/chain_id_response"
                          },
                          "tx_status": {
                            "$ref": "#/components/schemas/tx_status_response"
                          },
                          "tx_type": {
                            "$ref": "#/components/schemas/tx_type_response"
                          },
                          "label": {
                            "$ref": "#/components/schemas/label_response"
                          },
                          "swap_asset": {
                            "$ref": "#/components/schemas/swap_asset_response"
                          },
                          "side": {
                            "$ref": "#/components/schemas/side_response"
                          },
                          "fees": {
                            "$ref": "#/components/schemas/fees_response"
                          },
                          "price": {
                            "$ref": "#/components/schemas/price_response"
                          }
                        },
                        "required": [
                          "account",
                          "amount",
                          "collateral",
                          "finalized_timestamp",
                          "initiated_timestamp",
                          "l1_tx_hash",
                          "l2_tx_hash",
                          "chain_id",
                          "tx_status",
                          "tx_type"
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
            "$ref": "#/components/parameters/StartTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/EndTimeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/TxTypeQueryOptional"
          },
          {
            "$ref": "#/components/parameters/TxStatusQueryOptional"
          },
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