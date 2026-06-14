> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBSCRIBE Orderbook Throttled (NEW)

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div><span class="ws-description"> Returns the orderbook snapshot during the initial subscription. Subsequently, returns orderbook difference when there is any created order, matched order or cancelled order. Note: The messages send are throttled by 100ms OR 500ms by default</span>

<span className="ws-small-font">**UPDATE TIME**</span>

<span className="ws-data-type ws-small-font">**100ms** OR **500ms**</span>

<span className="ws-small-font">**REQUEST**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*op\*\* <span className="ws-data-type ws-small-font">string</span>  <span className="ws-required-tag ws-small-font">required</span>
          Operation code allowed values: `subscribe` `unsubscribe`
      </td>
    </tr>

    <tr>
      <td>
        * \*data\*\* <span className="ws-data-type ws-small-font">array of strings</span>  <span className="ws-required-tag ws-small-font">required</span>
          Channel name in the format `orderbook-100ms:SYMBOL`. Eg. `orderbook-100ms:ETH-31MAR23-1350-C`
      </td>
    </tr>
  </tbody>
</Table>

<details />

<summary>Example</summary>

```json 100ms
{
	"op":"subscribe", 
	"data": ["orderbook-100ms:ETH-31MAR23-1350-C"]
}
```

```json 500ms
{
	"op":"subscribe", 
	"data": ["orderbook-500ms:ETH-31MAR23-1350-C"]
}
```

<br />

<span className="ws-small-font">**RESPONSE**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*channel\*\* <span className="ws-data-type ws-small-font">string</span>  <span className="ws-required-tag ws-small-font">required</span>
          Channel name in the format `orderbook-100ms:INSTRUMENT_NAME`. Eg. `orderbook-100ms:ETH-31MAR23-1350-C`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**type** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Type of orderbook message. Allowed values: `snapshot` `update`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**instrument\_id** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Instrument ID number. Eg. `12`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**instrument\_name** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Instrument symbol. Eg. ` ETH-24DEC22-1250-C`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**instrument\_type** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Type of instrument. Allowed values: `OPTION` `PERPETUAL`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**bids** <span className="ws-data-type ws-small-font">array of arrays</span> <span className="ws-required-tag ws-small-font">required</span>
        Array of 3 elements - price in USD, contract amount and order IV. Eg.`[["1", "10", "0.75"]]`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**asks** <span className="ws-data-type ws-small-font">array of arrays</span> <span className="ws-required-tag ws-small-font">required</span>
        Array of 3 elements - price in USD, contract amount and order IV. Eg.`[["1", "10", "0.85"]]`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**last\_updated** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Last updated timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`
      </td>
    </tr>

    <tr>
      <td>
        <span className="ws-small-font">data.</span>**checksum** <span className="ws-data-type ws-small-font">string</span> <span className="ws-required-tag ws-small-font">required</span>
        Payload checksum. Eg. `1321749405`
      </td>
    </tr>
  </tbody>
</Table>

<details />

<summary>Example</summary>

```json
{
    "channel": "orderbook-100ms:ETH-31MAR23-1350-C",
    "data": {
      	"type": "update",
        "instrument_id": "165",
        "instrument_name": "ETH-31MAR23-1350-C",
        "instrument_type": "OPTION",  
        "bids": [
            [
                "1",
                "10",
              	"0.75"
            ]
        ],
        "asks": [
            [
                "10",
                "1",
              	"0.85"
            ]
        ],
        "last_updated": "1673436052887313432",
        "checksum": "1321749405"
    }
}
```

<br />

> 📘 Note!
>
> Bid or ask contract amount can be 0. This indicates that the price level has been removed from the orderbook.