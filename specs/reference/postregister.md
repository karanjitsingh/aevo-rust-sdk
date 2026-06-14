> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# POST /register

Registers a new account.

# OpenAPI definition

```json
{
  "components": {
    "schemas": {
      "account": {
        "title": "account",
        "type": "string",
        "description": "Account's Ethereum address.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "account_signature": {
        "title": "account_signature",
        "type": "string",
        "description": "Hash of EIP-712 signature signed by the account.",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "api_key_response": {
        "title": "api_key_response",
        "type": "string",
        "description": "Account's API Key. Eg. `URPtt6eNCXgL8ERuchphUretdaga2smF`",
        "example": "URPtt6eNCXgL8ERuchphUretdaga2smF"
      },
      "api_secret_response": {
        "title": "api_secret_response",
        "type": "string",
        "description": "Client's API Secret Eg. `0140af7046a63530fc4bd319823d6eee98086ef0d446584b42f68b640b60c457`",
        "example": "0140af7046a63530fc4bd319823d6eee98086ef0d446584b42f68b640b60c457"
      },
      "created_timestamp_response": {
        "title": "created_timestamp_response",
        "type": "string",
        "description": "Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
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
      "expiry": {
        "title": "expiry",
        "type": "string",
        "description": "Signing key expiry in UNIX timestamp in seconds.",
        "example": 1685520000
      },
      "expiry_response": {
        "title": "expiry_response",
        "type": "string",
        "description": "Option expiry in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`",
        "example": "1680249600000000000"
      },
      "no_api_key": {
        "title": "no_api_key",
        "type": "boolean",
        "description": "If registering without creating an api key",
        "example": false
      },
      "read_only_response": {
        "title": "read_only_response",
        "type": "boolean",
        "description": "API can only access read-only endpoints if true. Eg. `true`",
        "example": true
      },
      "referral_code": {
        "title": "referral_code",
        "type": "string",
        "description": "Referral Code (username of referrer) of the new account registration",
        "example": "Indignant-Intelligent-Satoshi"
      },
      "signing_key": {
        "title": "signing_key",
        "type": "string",
        "description": "Ethereum address of the signing key.",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "signing_key_response": {
        "title": "signing_key_response",
        "type": "string",
        "description": "Ethereum address of the signing key. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`",
        "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
      },
      "signing_key_signature": {
        "title": "signing_key_signature",
        "type": "string",
        "description": "Hash of EIP-712 signature signed by the signing key.",
        "example": "0x4c55895aa6f2dfc2da8189a71a054348ce63abbc1ae27267977e0d9cc6848e91"
      },
      "success_response": {
        "title": "success_response",
        "type": "boolean",
        "description": "Request successful if true. Eg. `true`",
        "example": true
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
    "/register": {
      "post": {
        "summary": "POST /register",
        "tags": [
          "Private API"
        ],
        "operationId": "PostRegister",
        "description": "Registers a new account.",
        "responses": {
          "200": {
            "description": "Account keys and credentials.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "PostRegister200Response",
                  "type": "object",
                  "properties": {
                    "success": {
                      "$ref": "#/components/schemas/success_response"
                    },
                    "signing_keys": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "signing_key": {
                            "$ref": "#/components/schemas/signing_key_response"
                          },
                          "expiry": {
                            "$ref": "#/components/schemas/expiry_response"
                          },
                          "created_timestamp": {
                            "$ref": "#/components/schemas/created_timestamp_response"
                          }
                        },
                        "required": [
                          "signing_key",
                          "expiry",
                          "created_timestamp"
                        ]
                      }
                    },
                    "api_key": {
                      "$ref": "#/components/schemas/api_key_response"
                    },
                    "api_secret": {
                      "$ref": "#/components/schemas/api_secret_response"
                    },
                    "read_only": {
                      "$ref": "#/components/schemas/read_only_response"
                    }
                  },
                  "required": [
                    "success",
                    "api_key",
                    "api_secret",
                    "read_only"
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
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "title": "PostRegisterPayload",
                "type": "object",
                "properties": {
                  "account": {
                    "$ref": "#/components/schemas/account"
                  },
                  "signing_key": {
                    "$ref": "#/components/schemas/signing_key"
                  },
                  "expiry": {
                    "$ref": "#/components/schemas/expiry"
                  },
                  "account_signature": {
                    "$ref": "#/components/schemas/account_signature"
                  },
                  "signing_key_signature": {
                    "$ref": "#/components/schemas/signing_key_signature"
                  },
                  "referral_code": {
                    "$ref": "#/components/schemas/referral_code"
                  },
                  "no_api_key": {
                    "$ref": "#/components/schemas/no_api_key"
                  }
                },
                "required": [
                  "account",
                  "signing_key",
                  "expiry",
                  "account_signature",
                  "signing_key_signature"
                ]
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