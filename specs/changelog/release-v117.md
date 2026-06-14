> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Release v1.1.7

## TO BE RELEASED: 29-07-2023

***

Introduces the `timestamp` field in order signing to prevent replay attacks and guarantee order uniqueness. The `timestamp` field is in UNIX seconds format.

## Order Signing Changelog

***

Order is optionally signed with the `timestamp` field included.

## REST API Changelog

***

`POST /order`

* added `timestamp` as an optional parameter

`POST /orders/{order_id}`

* added `timestamp` as an optional parameter

## Websockets Changelog

***

`PUBLISH Create Order`

* added `timestamp` as an optional parameter

`PUBLISH Edit Order`

* added `timestamp` as an optional parameter