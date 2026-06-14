> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# GET /check-referral

Check if user can be referred.

# OpenAPI definition

```json
{
  "components": {
    "parameters": {
      "AccountQuery": {
        "name": "account",
        "in": "query",
        "description": "Account's Ethereum address.",
        "required": true,
        "schema": {
          "type": "string",
          "example": "0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b"
        }
      },
      "ReferralCodeQuery": {
        "name": "referral_code",
        "in": "query",
        "description": "Referral Code (username of referrer) of the new account registration",
        "required": true,
        "schema": {
          "type": "string",
          "example": "Indignant-Intelligent-Satoshi"
        }
      }
    },
    "schemas": {
      "code_response": {
        "title": "code_response",
        "type": "string",
        "description": "Error code string of why user cannot be referred by a specific referral-code Eg. `User has already been referred`",
        "example": "User has already been referred"
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
      "is_referrable_response": {
        "title": "is_referrable_response",
        "type": "boolean",
        "description": "Whether or not the user can be referred by a specific referral-code Eg. `true`",
        "example": true
      },
      "referee_discount_response": {
        "title": "referee_discount_response",
        "type": "string",
        "description": "Total percentage discount for being referred by this referrer Eg. `0.1`",
        "example": "0.1"
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
    "/check-referral": {
      "get": {
        "summary": "GET /check-referral",
        "tags": [
          "Public API"
        ],
        "operationId": "GetCheckReferral",
        "description": "Check if user can be referred.",
        "responses": {
          "200": {
            "description": "Can user be referred",
            "content": {
              "application/json": {
                "schema": {
                  "title": "GetCheckReferral200Response",
                  "type": "object",
                  "properties": {
                    "is_referrable": {
                      "$ref": "#/components/schemas/is_referrable_response"
                    },
                    "referee_discount": {
                      "$ref": "#/components/schemas/referee_discount_response"
                    },
                    "code": {
                      "$ref": "#/components/schemas/code_response"
                    }
                  },
                  "required": [
                    "is_referrable",
                    "referee_discount"
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
            "$ref": "#/components/parameters/AccountQuery"
          },
          {
            "$ref": "#/components/parameters/ReferralCodeQuery"
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