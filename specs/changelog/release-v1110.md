> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Release v1.1.10

This introduces breaking changes around `limit` and `start_time`.

# Breaking Changes

## 1. Max limit set to 50. Default limit set to 10

Previously, the max `limit` was set to 1000. This is adjusted to max `limit` of 50, and the default `limit` is set to 10 when `limit` is not passed in.

The affected endpoints are listed below:

* /instrument/:instrument\_name/trade-history
* /funding-history
* /index-history
* /index-histories
* /mark-history
* /settlement-history
* /trade-history
* /balance-history
* /order-history
* /order-history/stops
* /transaction-history
* /leaderboard
* /referral-statistics
* /block-history
* /quote-history
* /block-trade-history

## 2. start\_time must be >=30 days

We enforce start\_time to be a nanosecond timestamp >=30 days. If `start_time` is not passed in, we default to `now - 30 days`.