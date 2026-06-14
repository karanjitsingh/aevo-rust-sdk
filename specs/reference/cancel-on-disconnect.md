> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Cancel on Disconnect

The Cancel on Disconnect feature can be enabled for an account using the [POST /account/cancel-on-disconnect](https://api-docs.aevo.xyz/reference/postaccountcancelondisconnect) endpoint. The status for Cancel on Disconnect can be checked on [GET /account/cancel-on-disconnect](https://docs.aevo.xyz/reference/getaccountcancelondisconnect).

When enabled, CoD automatically cancels all orders that are made with a websocket connection that is closed. This means if an account has multiple websocket connections, and only 1 connection is closed, only the orders that are created from the closed connection are cancelled.

Below is an example of how CoD works:

1. Account enables Cancel on Disconnect.
2. Account opens 2 websocket connection to Aevo. Connection 1 and Connection 2.
3. Account creates orders using Connection 1.
4. Connection 1 is closed, all orders created with Connection 1 are cancelled. Any orders made with Connection 2 are not cancelled.