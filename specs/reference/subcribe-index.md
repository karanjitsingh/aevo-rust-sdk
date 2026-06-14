> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBSCRIBE Index

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div>

<span class="ws-description"> Returns the index price. </span>

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
          Channel name in the format `index:ASSET`. Eg. `index:ETH`
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
    "data": ["index:ETH"]
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
        <span class="ws-small-font">data.</span>**price** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Price in USD. Eg. `12.23`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**timestamp** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Index update timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`
      </td>
    </tr>
  </tbody>
</Table>

<details>
  <summary>Example</summary>

  ```json
  {
      "channel": "index:ETH",
      "data": {
          "price": "1337.16",
          "timestamp": "1673438070391698947"
      }
  }
  ```
</details>

<br />