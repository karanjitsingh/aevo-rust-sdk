> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBSCRIBE Trades

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div>

<span class="ws-description"> Returns matched order details. </span>

<span class="ws-small-font">**REQUEST**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*op\*\* <span class="ws-data-type ws-small-font">string</span>  <span class="ws-required-tag ws-small-font">required</span>
          Operation code allowed values: `subscribe` `unsubscribe`
      </td>
    </tr>

    <tr>
      <td>
        * \*data\*\* <span class="ws-data-type ws-small-font">array of strings</span>  <span class="ws-required-tag ws-small-font">required</span>
          Channel name in the format `trades:ASSET` or `trades:INSTRUMENT_NAME`. Eg. `trades:ETH` or `trades:ETH-31MAR23-1350-C`
      </td>
    </tr>
  </tbody>
</Table>

* `ASSET` allowed values: `ETH`

<details>
  <summary>Example</summary>

  ```json
  {
  	"op":"subscribe", 
  	"data": ["trades:ETH-31MAR23-1350-C"]
  }
  ```
</details>

<br />

<span class="ws-small-font">**RESPONSE**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*channel\*\* <span class="ws-data-type ws-small-font">string</span>  <span class="ws-required-tag ws-small-font">required</span>
          Channel name in the format `index:ASSET`. Eg. `index:ETH`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**trade\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Trade transaction hash. Eg. `0x17897e438f15c459a3a4ed44687dc7f0c4c678b2b005ea87bbae35346f279f02`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**instrument\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Instrument ID number. Eg. `12`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**instrument\_name** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Instrument name. Eg.` ETH-24DEC22-1250-C`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**instrument\_type** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Type of instrument. Allowed values: `OPTION` `PERPETUAL`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**side** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Trade side. Allowed values: `buy` `sell`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**price** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Price in USD. Eg. `12.23`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**amount** <span class="ws-data-type ws-small-font">string</span>
        Amount of contracts. Eg. `10.4`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**created\_timestamp** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`
      </td>
    </tr>
  </tbody>
</Table>

<details>
  <summary>Example</summary>

  ```json
  {
      "channel": "trades:ETH-20JAN23-1200-P",
      "data": {
          "trade_id": "0x17897e438f15c459a3a4ed44687dc7f0c4c678b2b005ea87bbae35346f279f02",
          "instrument_id": 830,
          "instrument_name": "ETH-20JAN23-1200-P",
          "instrument_type": "OPTION",
          "side": "sell",
          "price": "0.58",
          "amount": "1",
          "created_ttimestamp": "1674118637229190091"
      }
  }
  ```
</details>

<br />