> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBCRIBE Fills

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div>

<span class="ws-description"> Returns fills when orders are filled. </span>

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
          Channel name: `fills`
      </td>
    </tr>
  </tbody>
</Table>

`<details>`
<summary>Example</summary>

```json
{
	"op":"subscribe", 
	"data": ["fills"]
}
```

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
          Channel name: `fills`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**timestamp** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Update timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**fill** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Fill updates in `Fill` object format.

        <div style={{ height: "10px" }} />

        <table>
          <tr>
            <td class="child-header">**FILL OBJECT**</td>
          </tr>

          <tr>
            <td>**trade\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Unique ID of the trade. Eg. `DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2`</td>
          </tr>

          <tr>
            <td>**order\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Order ID. Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`</td>
          </tr>

          <tr>
            <td>**instrument\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Instrument ID number. Eg. `12` </td>
          </tr>

          <tr>
            <td>**instrument\_name** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Instrument name. Eg. `ETH-24DEC22-1250-C`</td>
          </tr>

          <tr>
            <td>**instrument\_type** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Type of instrument. Allowed values: `OPTION` `PERPETUAL`</td>
          </tr>

          <tr>
            <td>**price** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Price in USD. Eg. `12.23`</td>
          </tr>

          <tr>
            <td>**side** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Trade side. Allowed values: `buy` `sell`</td>
          </tr>

          <tr>
            <td>**fees** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Fees in USD. Eg. `12.23`</td>
          </tr>

          <tr>
            <td>**filled** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Amount filled. Eg. `10.4`</td>
          </tr>

          <tr>
            <td>**order\_status** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Order status. Allowed values: `partial` `filled`</td>
          </tr>

          <tr>
            <td>**liquidity** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Liquidity side. Allowed values: `taker` `maker`</td>
          </tr>

          <tr>
            <td>**created\_timestamp** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Created timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`</td>
          </tr>

          <tr>
            <td>**system\_type** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
            Order's system type. Eg. `WEB` or `API`</td>
          </tr>
        </table>
      </td>
    </tr>
  </tbody>
</Table>

`<details>`
<summary>Example</summary>

```json
{
	"channel": "fills",
	"data": {
		"timestamp": "1673671845685460000",
		"fill": {
				"trade_id": "DwmDn5XnEyiqx5AB5CM4W8bgD137ASX4Lz1XWBYqvpX2",
				"order_id": "0xbc39f8c0dd85c0f7059124b2d2c6c2ec87f4a0e748ae21c66e98fbfa1974981e",
				"instrument_id": "8",
				"instrument_name": "ETH-14JAN23-1500-P",
				"instrument_type": "OPTION",
				"price": "12.23",
				"side": "sell",
				"fees": "0.23",	
				"filled": "3",
				"order_status": "partial",
				"liquidity": "taker",
				"created_timestamp": "1673671845684502000"
		}
	}
}

```

<br />