> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBCRIBE Orders

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div>

<span class="ws-description"> Returns order status if any changes occur. </span>

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
          Channel name: `orders`
      </td>
    </tr>
  </tbody>
</Table>

<details>
  <summary>Example</summary>

  ```json
  {
  	"op":"subscribe", 
  	"data": ["orders"]
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
            Channel name: `orders`
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
          <span class="ws-small-font">data.</span>**orders** <span class="ws-data-type ws-small-font">array of objects</span> <span class="ws-required-tag ws-small-font">required</span>
          Order updates in `Order` object format.

          <div style={{ height: "10px" }} />

          <table>
            <tr>
              <td class="child-header">**ORDER OBJECT**</td>
            </tr>

            <tr>
              <td>**order\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Order ID. Eg. `0x4c43e0ab72a4edb72dfe4b129148899815d816837b9a7e22d964b884834639f8`</td>
            </tr>

            <tr>
              <td>**account** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Account's Ethereum address. Eg. `0xE9b3a48d15BE316A8e34FAd53fFDFDddf0C3D24b`</td>
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
              <td>**order\_type** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Order type. Allowed values: `limit` `market`</td>
            </tr>

            <tr>
              <td>**side** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Trade side. Allowed values: `buy` `sell`</td>
            </tr>

            <tr>
              <td>**price** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Price in USD. Eg. `12.23`</td>
            </tr>

            <tr>
              <td>**amount** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Amount of contracts. Eg. `12.23`</td>
            </tr>

            <tr>
              <td>**filled** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Amount filled. Eg. `10.4`</td>
            </tr>

            <tr>
              <td>**order\_status** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
              Order status. Allowed values: `opened` `cancelled` `partial` `filled`</td>
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

  <details>
    <summary>Example</summary>

    ```json
    {
    	"channel": "orders",
    	"data": {
    		"timestamp": "1673671845685460000",
    		"orders": [
    			{
    				"order_id": "0xbc39f8c0dd85c0f7059124b2d2c6c2ec87f4a0e748ae21c66e98fbfa1974981e",
    				"account": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    				"instrument_id": "8",
    				"instrument_name": "ETH-14JAN23-1500-P",
    				"instrument_type": "OPTION",
    				"order_type": "limit",
    				"side": "sell",
    				"price": "1200",
    				"amount": "2",
    				"filled": "0",
    				"order_status": "opened",
    				"created_timestamp": 1673671845684502000
    			}
    		]
    	}
    }
    ```

    <br />
  </details>
</details>